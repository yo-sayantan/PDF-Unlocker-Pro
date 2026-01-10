#![windows_subsystem = "windows"]

use native_windows_gui as nwg;
use native_windows_derive::NwgUi;
use nwg::NativeUi;
use std::sync::{Arc, Mutex};
use std::thread;

mod cracker;
use cracker::run_password_cracker;

#[derive(Default, NwgUi)]
pub struct PDFUnlockerApp {
    #[nwg_control(
        size: (950, 750), 
        position: (250, 100), 
        title: "🔓 PDF Password Unlocker - Advanced Edition", 
        flags: "WINDOW|VISIBLE",
        accept_files: true
    )]
    #[nwg_events(OnWindowClose: [PDFUnlockerApp::exit])]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 15, margin: [20, 20, 20, 20])]
    layout: nwg::GridLayout,

    // Header
    #[nwg_control(text: "🔓 PDF PASSWORD UNLOCKER", font: Some(&data.title_font))]
    #[nwg_layout_item(layout: layout, row: 0, col: 0, col_span: 5)]
    title_label: nwg::Label,

    #[nwg_control(text: "Advanced Multi-Threaded PDF Password Recovery System", font: Some(&data.subtitle_font))]
    #[nwg_layout_item(layout: layout, row: 1, col: 0, col_span: 5)]
    subtitle_label: nwg::Label,

    // PDF File Selection
    #[nwg_control(text: "📄 PDF FILE", font: Some(&data.section_font))]
    #[nwg_layout_item(layout: layout, row: 2, col: 0)]
    pdf_label: nwg::Label,

    #[nwg_control(readonly: true, font: Some(&data.input_font))]
    #[nwg_layout_item(layout: layout, row: 2, col: 1, col_span: 3)]
    pdf_path: nwg::TextInput,

    #[nwg_control(text: "BROWSE", font: Some(&data.button_font))]
    #[nwg_layout_item(layout: layout, row: 2, col: 4)]
    #[nwg_events(OnButtonClick: [PDFUnlockerApp::select_pdf])]
    browse_btn: nwg::Button,

    // Thread Count
    #[nwg_control(text: "⚡ THREADS", font: Some(&data.section_font))]
    #[nwg_layout_item(layout: layout, row: 3, col: 0)]
    thread_label: nwg::Label,

    #[nwg_control(text: "100", font: Some(&data.input_font))]
    #[nwg_layout_item(layout: layout, row: 3, col: 1)]
    thread_input: nwg::TextInput,

    #[nwg_control(text: "💡 Recommended: 50-200 threads for modern CPUs", font: Some(&data.hint_font))]
    #[nwg_layout_item(layout: layout, row: 3, col: 2, col_span: 3)]
    thread_hint: nwg::Label,

    // Password Pattern
    #[nwg_control(text: "🔑 PASSWORD PATTERN", font: Some(&data.section_font))]
    #[nwg_layout_item(layout: layout, row: 4, col: 0)]
    pattern_label: nwg::Label,

    #[nwg_control(text: "", placeholder_text: Some("Example: 1234nnnnnn5678"), font: Some(&data.input_font))]
    #[nwg_layout_item(layout: layout, row: 4, col: 1, col_span: 4)]
    pattern_input: nwg::TextInput,

    // Pattern Guide
    #[nwg_control(
        text: "PATTERN GUIDE\n\nn = Numeric (0-9)\nc = Alphabetic (a-z, A-Z)\na = Alphanumeric (0-9, a-z, A-Z)\nx = Any printable character\n\nExample:\nPattern: 1234nnnnnn5678\nMeaning: 1234 + 6 digits + 5678\nCombinations: 1,000,000",
        readonly: true,
        flags: "VISIBLE|VSCROLL",
        font: Some(&data.guide_font)
    )]
    #[nwg_layout_item(layout: layout, row: 5, col: 0, col_span: 5, row_span: 3)]
    pattern_guide: nwg::TextBox,

    // Start Button
    #[nwg_control(text: "🚀 START CRACKING", font: Some(&data.start_font))]
    #[nwg_layout_item(layout: layout, row: 8, col: 1, col_span: 3)]
    #[nwg_events(OnButtonClick: [PDFUnlockerApp::start_cracking])]
    start_btn: nwg::Button,

    // Progress/Status
    #[nwg_control(
        text: "SYSTEM READY\n\nAwaiting PDF file and password pattern...", 
        readonly: true, 
        flags: "VISIBLE|VSCROLL", 
        font: Some(&data.status_font)
    )]
    #[nwg_layout_item(layout: layout, row: 9, col: 0, col_span: 5, row_span: 3)]
    status_output: nwg::TextBox,

    // Fonts
    #[nwg_resource(family: "Segoe UI", size: 28, weight: 900)]
    title_font: nwg::Font,

    #[nwg_resource(family: "Segoe UI", size: 14, weight: 400)]
    subtitle_font: nwg::Font,

    #[nwg_resource(family: "Segoe UI", size: 16, weight: 700)]
    section_font: nwg::Font,

    #[nwg_resource(family: "Segoe UI", size: 20, weight: 900)]
    start_font: nwg::Font,

    #[nwg_resource(family: "Segoe UI", size: 14, weight: 600)]
    button_font: nwg::Font,

    #[nwg_resource(family: "Segoe UI", size: 13)]
    input_font: nwg::Font,

    #[nwg_resource(family: "Segoe UI", size: 12)]
    hint_font: nwg::Font,

    #[nwg_resource(family: "Segoe UI", size: 13)]
    guide_font: nwg::Font,

    #[nwg_resource(family: "Consolas", size: 12)]
    status_font: nwg::Font,

    #[nwg_control]
    #[nwg_events(OnNotice: [PDFUnlockerApp::on_cracking_finished])]
    cracking_notice: nwg::Notice,

    cracking_result: Arc<Mutex<Option<Result<String, String>>>>,
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
                    }
                }
            }
        }
    }

    fn start_cracking(&self) {
        let pdf_path = self.pdf_path.text();
        let pattern = self.pattern_input.text();
        let thread_count_str = self.thread_input.text();

        if pdf_path.is_empty() {
            nwg::modal_error_message(&self.window, "Error", "Please select a PDF file!");
            return;
        }

        if pattern.is_empty() {
            nwg::modal_error_message(&self.window, "Error", "Please enter a password pattern!");
            return;
        }

        let num_threads: usize = match thread_count_str.parse() {
            Ok(n) if n > 0 && n <= 500 => n,
            _ => {
                nwg::modal_error_message(&self.window, "Error", "Thread count must be between 1 and 500!");
                return;
            }
        };
        
        self.start_btn.set_enabled(false);
        
        let mut status = String::new();
        status.push_str("CRACKING IN PROGRESS\n\n");
        status.push_str(&format!("File: {}\n", pdf_path));
        status.push_str(&format!("Pattern: {}\n", pattern));
        status.push_str(&format!("Threads: {}\n\n", num_threads));
        status.push_str("Check password_attempts.log for detailed progress.");
        self.status_output.set_text(&status);
        
        let pdf_path_clone = pdf_path.clone();
        let pattern_clone = pattern.clone();
        let result_store = self.cracking_result.clone();
        let notice_sender = self.cracking_notice.sender();

        thread::spawn(move || {
            let result = run_password_cracker(&pdf_path_clone, &pattern_clone, num_threads);
            *result_store.lock().unwrap() = Some(result);
            notice_sender.notice();
        });
    }

    fn on_cracking_finished(&self) {
        self.start_btn.set_enabled(true);
        self.status_output.set_text("CRACKING COMPLETED\n\nCheck password_attempts.log for full details.");
        
        let result = self.cracking_result.lock().unwrap().take();
        if let Some(res) = result {
            match res {
                Ok(password) => {
                    nwg::simple_message(
                        "PASSWORD FOUND!",
                        &format!("SUCCESS!\n\nPassword: {}\n\nUnlocked PDF saved successfully.", password),
                    );
                }
                Err(e) => {
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
    nwg::init().expect("Failed to init Native Windows GUI");
    let _app = PDFUnlockerApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
