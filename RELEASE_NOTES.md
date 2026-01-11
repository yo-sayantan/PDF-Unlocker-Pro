# 🔓 PDF Unlocker - Optimized Edition (v4.1)

## 📦 Build Optimizations
- **Reduced Binary Size**: Debug symbols are now automatically stripped from the release binary, resulting in a significantly smaller executable.
- **Cleaner Workspace**: Disabled incremental compilation for release builds to minimize the number of intermediate folders and files generated in the `target` directory.
- **Production Ready**: The build process is now tuned for efficient, standalone distribution.

---

# 🔓 PDF Unlocker - Universal Edition (v4.0)

## 🌍 Cross-Platform Support
- **MacOS & Linux**: Now fully compatible! Run the application natively on macOS and Linux terminals.
- **Universal Binary**: The same codebase now smartly adapts to the operating system it's running on.

## 🖥️ Terminal User Interface (TUI)
- **Rich Dashboard**: For non-GUI environments (and Windows CLI), a beautiful new terminal dashboard has been added.
- **Visual Feedback**: Features real-time progress bars, speed stats (attempts/sec), and status logs directly in your terminal.
- **Grid Layout**: New organized grid design displays Target File, Pattern, and Engine stats clearly.
- **Interactive**: Clean, responsive interface powered by `ratatui`.

## ⚡ CLI & GUI Hybrid
- **Windows**: Best of both worlds. Run normally for the classic Native GUI, or run with arguments to instantly switch to the high-performance CLI/TUI mode.
- **Command Line Arguments**: Added support for standard arguments:
  - `-i` / `--input`: Path to PDF file.
  - `-p` / `--pattern`: Password pattern.
  - `-t` / `--threads`: Number of threads (Optional, default: 150).

---

# 🔓 PDF Unlocker - Professional (v3.1)

## 💎 Pro Branding & Visuals
- **Application Icon**: The executable now features a native, high-quality icon (`.ico`), making it look professional in Windows Explorer and the Taskbar.
- **Pro Badge**: Introducing a "Verified Pro" gold shield badge in the UI header to signify the professional edition status.
- **Taskbar Integration**: The application window now correctly displays the custom icon in the taskbar and title bar.

## 🛠 Polish & Fixes
- **Resource Management**: Optimized how image assets (Logo, Badge, Icon) are loaded and embedded.
- **Build System**: integrated `winres` for native Windows resource compilation, ensuring proper metadata and icon embedding in the final binary.
- **Layout Refinements**: Adjusted header metrics to perfectly balance the Logo (left) and Pro Badge (right).

---

# 🔓 PDF Password Unlocker - Professional (v3.0)
- **New Logo Integration**: Added a custom, professional lock logo for stronger branding identity.
- **Enhanced Typography**: Upgraded all fonts to larger, clearer Segoe UI weights for better readability.
- **Spacious Layout**: Expanded window dimensions and improved spacing to prevent clutter.
- **Visual Cues**: Added color-coded unicode indicators to buttons (Start, Stop) for intuitive usage.
- **Clear Logs**: Implemented a "CLEAR" button to easily reset the system activity log.

## ✨ Features
- **Inline Configuration**: Restructured input fields for threads and patterns to be inline with labels.
- **Improved Guides**: Added detailed, non-intrusive pattern syntax guides directly on the main interface.
- **Dynamic Threading**: Increased supported thread count recommendation in UI hints (up to 600).
- **Log History**: System logs now append history rather than overwriting, allowing users to track multiple operations.

## 🚀 Performance & Stability
- **Stable Native Buttons**: Reverted to standard native button text stability to prevent crashes while maintaining visual distinction.
- **Thread Safety**: Enhanced thread management for starting and stopping the cracking engine.

*The Professional Edition represents a significant leap in usability and aesthetics.*

---

# 🔓 PDF Password Unlocker - Elite Edition (v2.0)
- **Premium User Interface**: Completely redesigned with a modern, clear, and professional layout.
- **Progress Tracking**: Added a real-time progress bar to visualize the cracking status.
- **Stop Control**: New "STOP" button allows safely halting the engine at any time.
- **Smart Hints**: Added explicit guide for pattern length matching (`length must match exact password length`).
- **Log Console**: Integrated real-time system activity log with scrollable history.
