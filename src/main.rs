#![windows_subsystem = "windows"]

use native_windows_gui as nwg;
use native_windows_derive::NwgUi;
use nwg::NativeUi;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

mod cracker;
use cracker::run_password_cracker;

#[derive(Default, NwgUi)]
pub struct PDFUnlockerApp {
    #[nwg_control(
        size: (900, 750), 
        position: (200, 100), 
        title: "🔓 PDF Unlocker - Professional", 
        flags: "WINDOW|VISIBLE",
        accept_files: true
    )]
    #[nwg_events(OnWindowClose: [PDFUnlockerApp::exit])]
    window: nwg::Window,

    #[nwg_resource(family: "Segoe UI", size: 22, weight: 700)]
    title_font: nwg::Font,

    #[nwg_resource(family: "Segoe UI", size: 10)]
    subtitle_font: nwg::Font,

    #[nwg_resource(family: "Segoe UI", size: 11, weight: 700)]
    section_font: nwg::Font,

    #[nwg_resource(family: "Segoe UI", size: 12, weight: 600)]
    button_font: nwg::Font,

    #[nwg_resource(family: "Segoe UI", size: 11)]
    input_font: nwg::Font,

    #[nwg_resource(family: "Consolas", size: 10)]
    console_font: nwg::Font,

    #[nwg_layout(parent: window, spacing: 10, margin: [20, 20, 20, 20])]
    layout: nwg::GridLayout,

    // --- HEADER ---
    #[nwg_control(text: "🔓 PDF PASSWORD UNLOCKER", font: Some(&data.title_font))]
    #[nwg_layout_item(layout: layout, row: 0, col: 0, col_span: 6)]
    title_label: nwg::Label,

    #[nwg_control(text: "High-performance multi-threaded recovery tool", font: Some(&data.subtitle_font))]
    #[nwg_layout_item(layout: layout, row: 1, col: 0, col_span: 6)]
    subtitle_label: nwg::Label,

    // --- SECTION 1: PDF FILE ---
    #[nwg_control(text: "TARGET FILE", font: Some(&data.section_font))]
    #[nwg_layout_item(layout: layout, row: 2, col: 0, col_span: 6)]
    file_section: nwg::Label,

    #[nwg_control(readonly: true, font: Some(&data.input_font))]
    #[nwg_layout_item(layout: layout, row: 3, col: 0, col_span: 5)]
    pdf_path: nwg::TextInput,

    #[nwg_control(text: "Browse", font: Some(&data.button_font))]
    #[nwg_layout_item(layout: layout, row: 3, col: 5)]
    #[nwg_events(OnButtonClick: [PDFUnlockerApp::select_pdf])]
    browse_btn: nwg::Button,

    // --- SECTION 2: CONFIGURATION ---
    #[nwg_control(text: "CONFIGURATION", font: Some(&data.section_font))]
    #[nwg_layout_item(layout: layout, row: 4, col: 0, col_span: 6)]
    config_section: nwg::Label,

    #[nwg_control(text: "128", font: Some(&data.input_font))]
    #[nwg_layout_item(layout: layout, row: 5, col: 0, col_span: 1)]
    thread_input: nwg::TextInput,

    #[nwg_control(text: "Threads (Recommended: 64-128)", font: Some(&data.input_font))]
    #[nwg_layout_item(layout: layout, row: 5, col: 1, col_span: 5)]
    thread_hint: nwg::Label,

    // --- SECTION 3: PATTERN ---
    #[nwg_control(text: "PASSWORD PATTERN", font: Some(&data.section_font))]
    #[nwg_layout_item(layout: layout, row: 6, col: 0, col_span: 6)]
    pattern_section: nwg::Label,

    #[nwg_control(text: "", placeholder_text: Some("e.g. 0000nnnn"), font: Some(&data.input_font))]
    #[nwg_layout_item(layout: layout, row: 7, col: 0, col_span: 6)]
    pattern_input: nwg::TextInput,

    #[nwg_control(
        text: "n=0-9 | c=a-z,A-Z | a=0-9,a-z | x=Any\r\nNote: The pattern length must match the password length exactly.",
        font: Some(&data.input_font)
    )]
    #[nwg_layout_item(layout: layout, row: 8, col: 0, col_span: 6, row_span: 2)]
    guide_label: nwg::Label,

    // --- PROGRESS ---
    #[nwg_control(text: "PROGRESS", font: Some(&data.section_font))]
    #[nwg_layout_item(layout: layout, row: 10, col: 0, col_span: 6)]
    prog_section: nwg::Label,

    #[nwg_control(range: 0..100)]
    #[nwg_layout_item(layout: layout, row: 11, col: 0, col_span: 6)]
    progress_bar: nwg::ProgressBar,

    // --- ACTIONS ---
    #[nwg_control(text: "Start Recovery", font: Some(&data.button_font))]
    #[nwg_layout_item(layout: layout, row: 12, col: 0, col_span: 3)]
    #[nwg_events(OnButtonClick: [PDFUnlockerApp::start_cracking])]
    start_btn: nwg::Button,

    #[nwg_control(text: "Stop", font: Some(&data.button_font), enabled: false)]
    #[nwg_layout_item(layout: layout, row: 12, col: 3, col_span: 3)]
    #[nwg_events(OnButtonClick: [PDFUnlockerApp::stop_cracking])]
    stop_btn: nwg::Button,

    // --- LOGS ---
    #[nwg_control(text: "SYSTEM LOG", font: Some(&data.section_font))]
    #[nwg_layout_item(layout: layout, row: 13, col: 0, col_span: 6)]
    log_section: nwg::Label,

    #[nwg_control(
        text: ">> Ready to start. Select a file and define a pattern above.", 
        readonly: true, 
        flags: "VISIBLE|VSCROLL", 
        font: Some(&data.console_font)
    )]
    #[nwg_layout_item(layout: layout, row: 14, col: 0, col_span: 6, row_span: 5)]
    status_output: nwg::TextBox,

    #[nwg_control]
    #[nwg_events(OnNotice: [PDFUnlockerApp::on_cracking_finished])]
    cracking_notice: nwg::Notice,

    #[nwg_control(interval: Duration::from_millis(500))]
    #[nwg_events(OnTimerTick: [PDFUnlockerApp::update_progress])]
    timer: nwg::AnimationTimer,

    cracking_result: Arc<Mutex<Option<Result<String, String>>>>,
    stop_flag: Arc<AtomicBool>,
    attempts_counter: Arc<AtomicU64>,
    total_combinations: Arc<AtomicU64>,
}

impl PDFUnlockerApp {
    fn select_pdf(&self) {
        let mut dialog = nwg::FileDialog::default();
        if nwg::FileDialog::builder()
            .title("Select Encrypted PDF")
            .filters("PDF (*.pdf)")
            .build(&mut dialog)
            .is_ok()
        {
            if dialog.run(Some(&self.window)) {
                if let Ok(path) = dialog.get_selected_item() {
                    if let Some(path_str) = path.to_str() {
                        self.pdf_path.set_text(path_str);
                        self.status_output.set_text(&format!(">> [INFO]: Selected file: {}\r\n", path_str));
                    }
                }
            }
        }
    }

    fn start_cracking(&self) {
        let pdf_path = self.pdf_path.text();
        let pattern = self.pattern_input.text();
        let thread_count_str = self.thread_input.text();

        if pdf_path.is_empty() || pattern.is_empty() {
            nwg::modal_error_message(&self.window, "Error", "Please fill in all fields.");
            return;
        }

        let num_threads: usize = thread_count_str.parse().unwrap_or(128);
        
        self.stop_flag.store(false, Ordering::SeqCst);
        self.attempts_counter.store(0, Ordering::SeqCst);
        self.progress_bar.set_pos(0);
        
        let mut combinations: u64 = 1;
        for ch in pattern.chars() {
            match ch {
                'n' => combinations = combinations.saturating_mul(10),
                'c' => combinations = combinations.saturating_mul(52),
                'a' => combinations = combinations.saturating_mul(62),
                'x' => combinations = combinations.saturating_mul(95),
                _ => {}
            }
        }
        self.total_combinations.store(combinations, Ordering::SeqCst);

        self.start_btn.set_enabled(false);
        self.stop_btn.set_enabled(true);
        self.timer.start();
        
        let mut status = String::new();
        status.push_str(">> [SYSTEM]: Cracking engine registered.\r\n");
        status.push_str(&format!(">> [SYSTEM]: Total combinations to test: {}\r\n", combinations));
        status.push_str(">> [SYSTEM]: Running...\r\n");
        self.status_output.set_text(&status);
        
        let pdf_path_clone = pdf_path.clone();
        let pattern_clone = pattern.clone();
        let result_store = self.cracking_result.clone();
        let notice_sender = self.cracking_notice.sender();
        let stop_flag = self.stop_flag.clone();
        let attempts_counter = self.attempts_counter.clone();

        thread::spawn(move || {
            let result = run_password_cracker(&pdf_path_clone, &pattern_clone, num_threads, stop_flag, attempts_counter);
            *result_store.lock().unwrap() = Some(result);
            notice_sender.notice();
        });
    }

    fn stop_cracking(&self) {
        self.stop_flag.store(true, Ordering::SeqCst);
        self.status_output.set_text(">> [SYSTEM]: Stopping...\r\n");
        self.stop_btn.set_enabled(false);
    }

    fn update_progress(&self) {
        let current = self.attempts_counter.load(Ordering::Relaxed);
        let total = self.total_combinations.load(Ordering::Relaxed);
        if total > 0 {
            let percentage = (current as f64 / total as f64 * 100.0) as u32;
            self.progress_bar.set_pos(percentage);
            
            if current % 100000 == 0 {
                let status = format!(">> [STATUS]: {} / {} ({:.2}%)\r\n", current, total, (current as f64 / total as f64 * 100.0));
                self.status_output.set_text(&status);
            }
        }
    }

    fn on_cracking_finished(&self) {
        self.start_btn.set_enabled(true);
        self.stop_btn.set_enabled(false);
        self.timer.stop();
        
        let result = self.cracking_result.lock().unwrap().take();
        if let Some(res) = result {
            match res {
                Ok(password) => {
                    self.progress_bar.set_pos(100);
                    self.status_output.set_text(">> [SYSTEM]: Password Found!\r\nUnlocked PDF saved successfully.");
                    nwg::simple_message(
                        "PASSWORD FOUND!",
                        &format!("SUCCESS!\n\nPassword: {}\n\nUnlocked PDF saved successfully.", password),
                    );
                }
                Err(e) => {
                    self.status_output.set_text(&format!(">> [SYSTEM]: Failed: {}\r\n", e));
                    nwg::simple_message("Cracking Failed", &format!("Error: {}", e));
                }
            }
        }
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to init NWG");
    let _app = PDFUnlockerApp::build_ui(Default::default()).expect("Failed UI build");
    nwg::dispatch_thread_events();
}
