#![cfg(windows)]

use native_windows_gui as nwg;
use native_windows_derive::NwgUi;
use nwg::NativeUi;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::time::Duration;
use crate::cracker::run_password_cracker;

#[derive(Default, NwgUi)]
pub struct PDFUnlockerApp {
    #[nwg_resource(source_file: Some("assets/icon.ico"))]
    window_icon: nwg::Icon,

    #[nwg_control(
        size: (800, 700),
        position: (200, 50),
        title: "🔓 PDF Unlocker - Universal Edition",
        flags: "WINDOW|VISIBLE",
        accept_files: true,
        icon: Some(&data.window_icon)
    )]
    #[nwg_events(OnWindowClose: [PDFUnlockerApp::exit])]
    window: nwg::Window,

    #[nwg_resource(family: "Segoe UI", size: 36, weight: 700)]
    title_font: nwg::Font,

    #[nwg_resource(family: "Segoe UI", size: 20)]
    subtitle_font: nwg::Font,

    #[nwg_resource(family: "Segoe UI", size: 16, weight: 700)]
    section_font: nwg::Font,

    #[nwg_resource(family: "Segoe UI", size: 20, weight: 600)]
    button_font: nwg::Font,

    #[nwg_resource(family: "Segoe UI", size: 15)]
    input_font: nwg::Font,

    #[nwg_resource(family: "Consolas", size: 13)]
    console_font: nwg::Font,

    #[nwg_resource(source_file: Some("assets/logo.png"))]
    logo: nwg::Bitmap,

    #[nwg_layout(parent: window, spacing: 10, margin: [10, 10, 10, 10])]
    layout: nwg::GridLayout,

    // --- HEADER ---
    #[nwg_control(bitmap: Some(&data.logo))]
    #[nwg_layout_item(layout: layout, row: 0, col: 0, row_span: 2)]
    logo_frame: nwg::ImageFrame,

    #[nwg_control(text: "🔓 PDF Unlocker - Universal Edition", font: Some(&data.title_font))]
    #[nwg_layout_item(layout: layout, row: 0, col: 1, col_span: 4)]
    title_label: nwg::Label,

    #[nwg_control(text: "High-performance multi-threaded recovery tool. Fast, secure, and reliable.", font: Some(&data.subtitle_font))]
    #[nwg_layout_item(layout: layout, row: 1, col: 1, col_span: 4)]
    subtitle_label: nwg::Label,

    // --- SECTION 1: PDF FILE (Stacked for length) ---
    #[nwg_control(text: "Target PDF File:", font: Some(&data.section_font))]
    #[nwg_layout_item(layout: layout, row: 2, col: 0, col_span: 1)]
    file_section: nwg::Label,

    #[nwg_control(readonly: true, font: Some(&data.input_font), placeholder_text: Some("Select a PDF file..."))]
    #[nwg_layout_item(layout: layout, row: 2, col: 1, col_span: 4)]
    pdf_path: nwg::TextInput,

    #[nwg_control(text: "ADD PDF", font: Some(&data.button_font))]
    #[nwg_layout_item(layout: layout, row: 2, col: 5)]
    #[nwg_events(OnButtonClick: [PDFUnlockerApp::select_pdf])]
    browse_btn: nwg::Button,

    // --- SECTION 2: CONFIGURATION (Inline) ---
    #[nwg_control(text: "Number of Threads:", font: Some(&data.section_font), v_align: nwg::VTextAlign::Center)]
    #[nwg_layout_item(layout: layout, row: 3, col: 0, col_span: 1)]
    config_section: nwg::Label,

    #[nwg_control(text: "250", font: Some(&data.input_font))]
    #[nwg_layout_item(layout: layout, row: 3, col: 1, col_span: 2)]
    thread_input: nwg::TextInput,

    #[nwg_control(text: "Threads (Higher = Faster, 200-600 rec.)", font: Some(&data.input_font), v_align: nwg::VTextAlign::Center)]
    #[nwg_layout_item(layout: layout, row: 3, col: 3, col_span: 2)]
    thread_hint: nwg::Label,

    // --- SECTION 3: PATTERN (Inline) ---
    #[nwg_control(text: "Password Pattern:", font: Some(&data.section_font), v_align: nwg::VTextAlign::Center)]
    #[nwg_layout_item(layout: layout, row: 4, col: 0, col_span: 1)]
    pattern_section: nwg::Label,

    #[nwg_control(text: "", placeholder_text: Some("Example: 0000nnnn1234"), font: Some(&data.input_font))]
    #[nwg_layout_item(layout: layout, row: 4, col: 1, col_span: 2)]
    pattern_input: nwg::TextInput,

    // --- SECTION 4: PATTERN GUIDE ---
    #[nwg_control(
        text: "n = Numeric (0-9)\nc = Alphabetic (a-z, A-Z)\na = Alphanumeric (0-9, a-z, A-Z)\nx = Any printable character",
        font: Some(&data.input_font)
    )]
    #[nwg_layout_item(layout: layout, row: 4, col: 1, col_span: 2, row_span: 3)]
    guide_label1: nwg::Label,

    #[nwg_control(
        text: "Example:\nPattern: 1234nnnnnn5678\nMeaning: 1234 + 6 digits + 5678\nCombinations: 1,000,000",
        font: Some(&data.input_font)
    )]
    #[nwg_layout_item(layout: layout, row: 4, col: 3, col_span: 2, row_span: 3)]
    guide_label2: nwg::Label,

    // --- SECTION 5: PROGRESS ---
    #[nwg_control(range: 0..100)]
    #[nwg_layout_item(layout: layout, row: 6, col: 0, col_span: 6)]
    progress_bar: nwg::ProgressBar,

    // --- SECTION 6: STATUS ---
    #[nwg_control(
        text: ">> Ready to start. Select a file and define a pattern above.\r\n",
        readonly: true,
        flags: "VISIBLE|VSCROLL",
        font: Some(&data.console_font)
    )]
    #[nwg_layout_item(layout: layout, row: 7, col: 0, col_span: 5, row_span: 3)]
    status_output: nwg::TextBox,

    // --- SECTION 7: ACTIONS ---
    #[nwg_control(text: "START", font: Some(&data.button_font))]
    #[nwg_layout_item(layout: layout, row: 7, col: 5, col_span: 1)]
    #[nwg_events(OnButtonClick: [PDFUnlockerApp::start_cracking])]
    start_btn: nwg::Button,

    #[nwg_control(text: "STOP", font: Some(&data.button_font), enabled: false)]
    #[nwg_layout_item(layout: layout, row: 8, col: 5, col_span: 1)]
    #[nwg_events(OnButtonClick: [PDFUnlockerApp::stop_cracking])]
    stop_btn: nwg::Button,

    #[nwg_control(text: "CLEAR", font: Some(&data.button_font))]
    #[nwg_layout_item(layout: layout, row: 9, col: 5, col_span: 1)]
    #[nwg_events(OnButtonClick: [PDFUnlockerApp::clear_logs])]
    clear_btn: nwg::Button,

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
            .title("Select PDF File")
            .action(nwg::FileDialogAction::Open)
            .filters("PDF Files(*.pdf)")
            .build(&mut dialog)
            .is_ok()
        {
            if dialog.run(Some(&self.window)) {
                if let Ok(path) = dialog.get_selected_item() {
                    if let Some(path_str) = path.to_str() {
                        self.pdf_path.set_text(path_str);
                        // Log file selection
                        let old_text = self.status_output.text();
                        self.status_output.set_text(&format!("{}>> [INFO]: Selected file: {}\r\n", old_text, path_str));
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

        let num_threads: usize = match thread_count_str.parse() {
            Ok(n) if n > 0 && n <= 600 => n,
            _ => {
                nwg::modal_error_message(&self.window, "Error", "Thread count must be between 1 and 600!");
                return;
            }
        };
        
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
        
        let mut status = self.status_output.text();
        status.push_str("\r\n>> [SYSTEM]:  Cracking engine registered.\r\n");
        status.push_str(&format!(">> [SYSTEM]:  Password Pattern: {}\r\n", pattern));
        status.push_str(&format!(">> [SYSTEM]:  Number of Threads: {}\r\n", num_threads));
        status.push_str(&format!(">> [SYSTEM]:  Total combinations to test: {}\r\n", combinations));
        status.push_str(">> [SYSTEM]:  Running...\r\n");
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
        let mut status = self.status_output.text();
        status.push_str(">> [SYSTEM]:  Stopping...\r\n");
        self.status_output.set_text(&status);
        self.stop_btn.set_enabled(false);
    }

    fn clear_logs(&self) {
        self.status_output.set_text(">> Logs cleared.\r\n");
    }

    fn update_progress(&self) {
        let current = self.attempts_counter.load(Ordering::Relaxed);
        let total = self.total_combinations.load(Ordering::Relaxed);
        if total > 0 {
            let percentage = (current as f64 / total as f64 * 100.0) as u32;
            self.progress_bar.set_pos(percentage);
        }
    }

    fn on_cracking_finished(&self) {
        self.start_btn.set_enabled(true);
        self.stop_btn.set_enabled(false);
        self.timer.stop();
        
        let result = self.cracking_result.lock().unwrap().take();
        if let Some(res) = result {
            let mut status = self.status_output.text();
            match res {
                Ok(password) => {
                    self.progress_bar.set_pos(100);
                    status.push_str(">> [SYSTEM]:  Password Found!\r\n");
                    status.push_str(&format!(">> [SUCCESS]: Password is: {}\r\n", password));
                    status.push_str(">> [SYSTEM]:  Unlocked PDF saved successfully.\r\n");
                    self.status_output.set_text(&status);
                    
                    nwg::simple_message(
                        "PASSWORD FOUND!",
                        &format!("SUCCESS!\n\nPassword: {}\n\nUnlocked PDF saved successfully.", password),
                    );
                }
                Err(e) => {
                    status.push_str(&format!(">> [SYSTEM]:  Failed: {}\r\n", e));
                    self.status_output.set_text(&status);
                    nwg::simple_message("Cracking Failed", &format!("Error: {}", e));
                }
            }
        }
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

pub fn run() {
    nwg::init().expect("Failed to init NWG");
    let _app = PDFUnlockerApp::build_ui(Default::default()).expect("Failed UI build");
    nwg::dispatch_thread_events();
}
