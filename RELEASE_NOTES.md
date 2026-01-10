# Release Notes - v1.0

## 🔓 Overview
This is the first stable production release of the PDF Password Unlocker.

## ✨ Key Features
- **High-Performance Engine**: Multi-threaded cracking using Rust's `rayon` for maximum CPU utilization.
- **Pattern Matching**: Advanced pattern guide (`n`, `c`, `a`, `x`) for targeted recovery.
- **Interactive UI**: Clean, premium Windows interface with real-time status updates.
- **Logging**: Detailed attempt logging for tracking progress.

## 🛠️ Internal Improvements
- Extracted cracking logic into a dedicated module for better scalability.
- Optimized release builds with LTO (Link Time Optimization).
- Implemented `nwg::Notice` for thread-safe UI updates.

## 📦 Downloads
Securely download the `pdf_unlocker.exe` attached to this release.
