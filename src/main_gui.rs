use native_windows_gui as nwg;
use native_windows_derive as nwd;
use std::sync::{Arc, Mutex};
use std::thread;

mod cracker;
use cracker::run_password_cracker;

#[derive(Default, nwd::NwgUi)]
pub struct PDFUnlockerApp {
    #[nwg_control(size: (600, 500), position: (300, 300), title: "PDF Password Unlocker", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [PDFUnlockerApp::exit])]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 10)]
    layout: nwg::GridLayout,

    // PDF File Selection
    #[nwg_control(text: "PDF File:")]
    #[nwg_layout_item(layout: layout, row: 0, col: 0)]
    pdf_label: nwg::Label,

    #[nwg_control(readonly: true)]
    #[nwg_layout_item(layout: layout, row: 0, col: 1, col_span: 2)]
    pdf_path: nwg::TextInput,

    #[nwg_control(text: "Browse...")]
    #[nwg_layout_item(layout: layout, row: 0, col: 3)]
    #[nwg_events(OnButtonClick: [PDFUnlockerApp::select_pdf])]
    browse_btn: nwg::Button,

    // Thread Count
    #[nwg_control(text: "Number of Threads:")]
    #[nwg_layout_item(layout: layout, row: 1, col: 0)]
    thread_label: nwg::Label,

    #[nwg_control(text: "50")]
    #[nwg_layout_item(layout: layout, row: 1, col: 1)]
    thread_input: nwg::TextInput,

    #[nwg_control(text: "(Default: 50, adjust based on your CPU)")]
    #[nwg_layout_item(layout: layout, row: 1, col: 2, col_span: 2)]
    thread_hint: nwg::Label,

    // Password Pattern
    #[nwg_control(text: "Password Pattern:")]
    #[nwg_layout_item(layout: layout, row: 2, col: 0)]
    pattern_label: nwg::Label,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: layout, row: 2, col: 1, col_span: 3)]
    pattern_input: nwg::TextInput,

    // Pattern Guide
    #[nwg_control(text: "Pattern Guide:\n  n = numeric (0-9)\n  c = alphabetic (a-z, A-Z)\n  a = alphanumeric (0-9, a-z, A-Z)\n  x = any printable character\n  Any other = fixed character\n\nExample: 0924nnnnnnnn3687\n(Tries 0924 + 8 unknown digits + 3687)", flags: "VISIBLE")]
    #[nwg_layout_item(layout: layout, row: 3, col: 0, col_span: 4, row_span: 3)]
    pattern_guide: nwg::Label,

    // Start Button
    #[nwg_control(text: "Start Cracking", size: (200, 40))]
    #[nwg_layout_item(layout: layout, row: 6, col: 1, col_span: 2)]
    #[nwg_events(OnButtonClick: [PDFUnlockerApp::start_cracking])]
    start_btn: nwg::Button,

    // Progress/Status
    #[nwg_control(text: "Ready", readonly: true, flags: "VISIBLE|VSCROLL")]
    #[nwg_layout_item(layout: layout, row: 7, col: 0, col_span: 4, row_span: 2)]
    status_output: nwg::TextBox,

    #[nwg_control()]
    file_dialog: nwg::FileDialog,
}

impl PDFUnlockerApp {
    fn select_pdf(&self) {
        if let Ok(d) = nwg::FileDialog::builder()
            .title("Select PDF File")
            .filters("PDF Files(*.pdf)|Any(*.*)")
            .multiselect(false)
            .build()
        {
            if d.run(Some(&self.window)) {
                if let Ok(selected) = d.get_selected_item() {
                    self.pdf_path.set_text(&selected);
                }
            }
        }
    }

    fn start_cracking(&self) {
        let pdf_path = self.pdf_path.text();
        let pattern = self.pattern_input.text();
        let thread_count = self.thread_input.text();

        // Validation
        if pdf_path.is_empty() {
            nwg::modal_error_message(&self.window, "Error", "Please select a PDF file!");
            return;
        }

        if pattern.is_empty() {
            nwg::modal_error_message(&self.window, "Error", "Please enter a password pattern!");
            return;
        }

        let num_threads: usize = match thread_count.parse() {
            Ok(n) if n > 0 && n <= 800 => n,
            _ => {
                nwg::modal_error_message(&self.window, "Error", "Thread count must be between 1 and 800!");
                return;
            }
        };

        // Disable start button
        self.start_btn.set_enabled(false);
        self.status_output.set_text("Starting password cracking...\n");

        // Start cracking in a separate thread
        let pdf_path_clone = pdf_path.clone();
        let pattern_clone = pattern.clone();
        
        thread::spawn(move || {
            match run_password_cracker(&pdf_path_clone, &pattern_clone, num_threads) {
                Ok(result) => {
                    nwg::modal_info_message(
                        &nwg::Window::default(),
                        "Success!",
                        &format!("Password found: {}\nUnlocked PDF saved!", result),
                    );
                }
                Err(e) => {
                    nwg::modal_error_message(
                        &nwg::Window::default(),
                        "Error",
                        &format!("Failed: {}", e),
                    );
                }
            }
        });
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    
    let _app = PDFUnlockerApp::build_ui(Default::default()).expect("Failed to build UI");
    
    nwg::dispatch_thread_events();
}
