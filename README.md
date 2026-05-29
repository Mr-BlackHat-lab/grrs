# 🦀 grrs — Mini-Grep CLI

A lightweight, blazing-fast Command Line Interface (CLI) tool written in Rust for finding text patterns within files.

## 🚀 About This Project

`grrs` is a small, focused `grep`-like utility implemented as part of a hands-on CLI-building exercise. The goal is to learn core Rust patterns for argument parsing, streaming file I/O, and idiomatic error handling while laying groundwork toward a future `diff` tool.

## 🛠️ Features

- **Pattern Matching:** Scans files line-by-line and prints lines that contain the exact query string.
- **Memory Efficient:** Uses buffered streaming to handle large files without loading them entirely into memory.
- **Idiomatic Error Handling:** Propagates errors with `Result` and `?`, keeping the main flow concise and safe.

## 💻 Prerequisites

- Rust and Cargo: install from https://www.rust-lang.org/tools/install

## 📦 Installation

1. Clone the repository:

```bash
git clone https://github.com/[your-username]/[your-repo-name].git
cd [your-repo-name]
```

2. Build in release mode:

```bash
cargo build --release
```

## 🎮 Usage

Run with Cargo (use `--` to separate Cargo args from program args):

```bash
# General syntax
cargo run -- <pattern> <path-to-file>

# Example: search for the word "error" in a log file
cargo run -- error /var/log/system.log
```

Or run the compiled binary directly:

```bash
./target/release/grrs "search_term" ./sample.txt
# On Windows PowerShell/CMD use:
\.\target\release\grrs.exe "search_term" .\sample.txt
```

## 📚 What I'm Learning (30 Days Challenge)

Building `grrs` helped reinforce these Rust concepts:

- Argument parsing with `clap` and deriving a typed CLI interface.
- File I/O using `std::fs::File` and `std::io::BufReader` for efficient streaming.
- Error propagation using `Result` and the `?` operator to keep top-level logic tidy.

## ⏭️ Next Steps

The next phase is to evolve this single-file reader into a true `diff`-style comparator:

- Read two `BufReader` streams concurrently.
- Implement a comparison algorithm to show inserted, removed, and changed lines.
- Add options for context lines, unified/side-by-side output, and colorized diffs.

---

License: MIT / Apache-2.0
