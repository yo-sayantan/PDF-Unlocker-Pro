# 🔓 PDF Password Unlocker

A high-performance, multi-threaded PDF password recovery tool built with Rust and Native Windows GUI.

## ✨ Features

- **Multi-threaded Cracking**: Utilizes all available CPU cores for maximum speed.
- **Pattern Matching**: Target specific password formats using a flexible pattern guide.
- **Cross-Platform**: Runs on Windows (GUI/CLI), macOS (CLI/TUI), and Linux (CLI/TUI).
- **User-friendly Interface**: Native GUI for Windows and rich Terminal UI for other platforms.

## 🚀 How to Use

1. **Select PDF**: Click 'BROWSE' to select the password-protected PDF file.
2. **Set Threads**: Enter the number of threads (Recommended: 50-200 for modern CPUs).
3. **Enter Pattern**: Provide the password pattern based on the guide below.
4. **Start**: Click 'START CRACKING' to begin the process.

## 🔑 Pattern Guide

The tool uses special characters to represent unknown parts of a password:

- `n` = Numeric (0-9)
- `c` = Alphabetic (a-z, A-Z)
- `a` = Alphanumeric (0-9, a-z, A-Z)
- `x` = Any printable character

### Example:
If you know the password starts with `1234`, ends with `5678`, and has 6 digits in between:
**Pattern**: `1234nnnnnn5678`

## 🛠️ Build Requirements

- Rust (latest stable)
- MSVC Build Tools (for Windows GUI)

```bash
cargo build --release
```

The executable will be located in `target/release/pdf_unlocker.exe`.

## 🍎 Running on macOS & Linux

If you downloaded the binary from GitHub Releases:

1.  **Open Terminal**.
2.  Navigate to the download folder:
    ```bash
    cd ~/Downloads
    ```
3.  **Make it executable** (Required):
    ```bash
    chmod +x pdf_unlocker
    ```
4.  **Run it**:
    ```bash
    ./pdf_unlocker
    ```
    *Note: On macOS, if you see a security warning, Go to System Settings > Privacy & Security and click "Open Anyway".*

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.
