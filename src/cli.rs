use clap::Parser;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use crate::cracker;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{CrosstermBackend, Backend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
    Terminal,
};
use std::io;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the PDF file
    #[arg(short, long)]
    pub input: String,

    /// Password pattern (e.g., "0000nnnn", "1234xxxx")
    #[arg(short, long)]
    pub pattern: String,

    /// Number of threads to use (Optional, default: 150)
    #[arg(short, long, default_value_t = 150)]
    pub threads: usize,
}

struct AppState {
    input_file: String,
    pattern: String,
    threads: usize,
    attempts: Arc<AtomicU64>,
    stop_flag: Arc<AtomicBool>,
    found_password: Arc<Mutex<Option<String>>>,
    start_time: Instant,
    total_combinations: u64,
}

pub fn run(args: Args) {
    // Setup terminal
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let stop_flag = Arc::new(AtomicBool::new(false));
    let attempts_counter = Arc::new(AtomicU64::new(0));
    let found_password = Arc::new(Mutex::new(None));
    
    // Calculate total combinations for progress bar
    let mut combinations: u64 = 1;
    for ch in args.pattern.chars() {
        match ch {
            'n' => combinations = combinations.saturating_mul(10),
            'c' => combinations = combinations.saturating_mul(52),
            'a' => combinations = combinations.saturating_mul(62),
            'x' => combinations = combinations.saturating_mul(95),
            _ => {}
        }
    }

    let app_state = AppState {
        input_file: args.input.clone(),
        pattern: args.pattern.clone(),
        threads: args.threads,
        attempts: attempts_counter.clone(),
        stop_flag: stop_flag.clone(),
        found_password: found_password.clone(),
        start_time: Instant::now(),
        total_combinations: combinations,
    };

    // Spawn cracker thread
    let input_clone = args.input.clone();
    let pattern_clone = args.pattern.clone();
    let threads = args.threads;
    let stop_flag_thread = stop_flag.clone();
    let attempts_thread = attempts_counter.clone();
    let found_password_thread = found_password.clone();

    let _cracker_handle = thread::spawn(move || {
        let result = cracker::run_password_cracker(
            &input_clone,
            &pattern_clone,
            threads,
            stop_flag_thread,
            attempts_thread
        );
        
        match result {
             Ok(pw) => {
                 *found_password_thread.lock().unwrap() = Some(pw);
             }
             Err(_) => {
                 // Error or not found, handled by UI checking state
             }
        }
    });

    let res = run_app(&mut terminal, app_state);

    // Restore terminal
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).unwrap();
    terminal.show_cursor().unwrap();

    if let Err(err) = res {
        println!("{:?}", err);
    }
    
    // Ensure cracker stops if it hasn't already
    stop_flag.store(true, Ordering::SeqCst);
    
    // Print final result to stdout after clearing TUI
    // We clone the string to avoid holding the lock too long or dealing with complicated lifetimes
    let final_pw = found_password.lock().unwrap().clone();
    if let Some(pw) = final_pw {
        println!("SUCCESS! Password found: {}", pw);
        println!("Unlocked PDF saved.");
    } else {
        println!("Process finished without finding password.");
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, state: AppState) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(100);

    loop {
        terminal.draw(|f| ui(f, &state))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        state.stop_flag.store(true, Ordering::SeqCst);
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
        
        if last_tick.elapsed() >= tick_rate {
             last_tick = Instant::now();
        }

        // Check completion
        if state.found_password.lock().unwrap().is_some() {
            // Give user a moment to see 100% or success message if desired, 
            // or return immediately. Let's wait for 'q' or return after a short delay?
            // For now, we return immediately so the result prints to stdout.
            return Ok(());
        }
        
        // Check if we exhausted everything (approximate check, unrelated to real thread status)
        let current = state.attempts.load(Ordering::Relaxed);
        if current >= state.total_combinations && state.total_combinations > 0 {
             // Finished without success
             return Ok(());
        }
    }
}

fn ui(f: &mut ratatui::Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Title
                Constraint::Length(5), // Info Grid (One row)
                Constraint::Length(5), // Live Stats (Attempts & Speed)
                Constraint::Length(3), // Progress Bar
                Constraint::Min(0),    // Fill remainder
            ]
            .as_ref(),
        )
        .split(f.size());

    // 1. Title
    let title_block = Paragraph::new("🔓 PDF Unlocker Pro - Universal Edition")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title_block, chunks[0]);

    // 2. Info Grid (File | Pattern | Threads)
    let info_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // File takes more space
            Constraint::Percentage(30), // Pattern
            Constraint::Percentage(20), // Threads
        ].as_ref())
        .split(chunks[1]);

    let file_info = Paragraph::new(state.input_file.as_str())
        .block(Block::default().title("📄 Target File").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .style(Style::default().fg(Color::White));
    f.render_widget(file_info, info_chunks[0]);

    let pattern_info = Paragraph::new(state.pattern.as_str())
        .block(Block::default().title("🔑 Pattern").borders(Borders::ALL).border_style(Style::default().fg(Color::Yellow)))
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));
    f.render_widget(pattern_info, info_chunks[1]);

    let thread_info = Paragraph::new(format!("{} Workers", state.threads))
        .block(Block::default().title("⚙️ Engine").borders(Borders::ALL).border_style(Style::default().fg(Color::Magenta)))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(thread_info, info_chunks[2]);

    // 3. Live Stats (Attempts | Speed)
    let stats_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[2]);

    let current_attempts = state.attempts.load(Ordering::Relaxed);
    let duration = state.start_time.elapsed();
    let speed = if duration.as_secs() > 0 {
        current_attempts / duration.as_secs()
    } else {
        0
    };

    let attempts_widget = Paragraph::new(format!("{}", current_attempts))
        .block(Block::default().title("🛡️ Total Attempts").borders(Borders::ALL))
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(attempts_widget, stats_chunks[0]);

    let speed_widget = Paragraph::new(format!("~{} /sec", speed))
        .block(Block::default().title("⚡ Cracking Speed").borders(Borders::ALL))
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(speed_widget, stats_chunks[1]);

    // 4. Progress Bar
    let progress = if state.total_combinations > 0 {
        (current_attempts as f64 / state.total_combinations as f64).min(1.0)
    } else {
        0.0
    };
    
    let label = format!("{:.1}%", progress * 100.0);
    let gauge = Gauge::default()
        .block(Block::default().title("Overall Progress").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::LightBlue))
        .ratio(progress)
        .label(label);
    f.render_widget(gauge, chunks[3]);

    // Quit Hint (Bottom, minimal)
    let bottom_text = Paragraph::new("Press 'q' to quit")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(bottom_text, chunks[4]); // Render in remainder area
}
