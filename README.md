# ⌨️ Ethical Keylogger (Educational)

A Rust-based keylogger developed for **educational purposes only**. This project demonstrates low-level system input handling, file I/O operations, and ethical security research principles.

## ⚠️ IMPORTANT DISCLAIMER

**This tool is for educational and research purposes only.** It must only be used on devices you own and control. Unauthorized use on others' systems is illegal and unethical.

## 🧠 Learning Objectives

This project was built to understand:
- Rust's foreign function interface (FFI) and low-level system APIs
- Global input hooking and event handling
- File I/O operations with error handling
- Ethical security research principles
- Rust's ownership model in practical applications

## ✨ Features

- **⌨️ Key Press Capture**: Logs all keyboard input with timestamps
- **⏰ Precision Timing**: Records events with millisecond precision
- **📁 Stealth Logging**: Saves logs to hidden file (`.keylog.txt`)
- **🛡️ Error Handling**: Robust error management with Rust's Result type
- **🔧 Cross-Platform Ready**: Designed with portability in mind

## 🛠️ Technical Implementation

### Dependencies
```toml
[dependencies]
rdev = "0.5.0"    # Cross-platform input capture
chrono = "0.4.38" # Precise timestamping

Installation-----
git clone https://github.com/Osinemem1/ethical-keylogger.git
cd ethical-keylogger
cargo build --release

cargo run
# Press keys to see them logged
# Press ESC to exit
