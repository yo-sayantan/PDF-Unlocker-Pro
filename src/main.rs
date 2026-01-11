mod cli;
#[cfg(windows)]
mod gui;
mod cracker;

use clap::Parser;
#[cfg(not(windows))]
use clap::CommandFactory;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // If arguments are provided (more than just the binary name), try to run as CLI
    if args.len() > 1 {
        let cli_args = cli::Args::parse();
        cli::run(cli_args);
    } else {
        // No arguments provided
        #[cfg(windows)]
        {
            // On Windows, default to GUI
            gui::run();
        }

        #[cfg(not(windows))]
        {
            // On non-Windows (Mac/Linux), show help because there is no GUI
            println!("PDF Unlocker - CLI Mode");
            println!("(GUI is only available on Windows)");
            println!();
            let mut cmd = cli::Args::command();
            cmd.print_help().unwrap();
        }
    }
}
