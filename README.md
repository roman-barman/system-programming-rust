# Code examples from "Practical System Programming for Rust Developers" book

This repository contains various system programming examples implemented in Rust, as featured in the book "Practical System Programming for Rust Developers".

## Projects

### 1. Calculator (`src/calculator`)
A command-line calculator that demonstrates lexical analysis and parsing. It uses a custom-built recursive descent parser to evaluate mathematical expressions.

### 2. Image CLI (`src/imagecli`)
A command-line tool for image processing. It provides functionality to resize images and gather image statistics using the `image` crate.

### 3. RStat (`src/rstat`)
A source code statistics tool. It analyzes source directories and provides metrics such as line counts and file distributions.

### 4. Shell (`src/shell`)
A basic implementation of a Unix-like shell, demonstrating process management and command execution in Rust.

### 5. TCP Proxy (`src/tcpproxy`)
A simple TCP proxy server. It includes both the proxy and an origin server to demonstrate network programming, byte manipulation, and basic proxy logic.

### 6. Template Engine (`src/template-engine`)
A lightweight template engine implementation, showing how to process text templates and substitute variables.

### 7. TUI Text Viewer (`src/tui`)
A terminal-based text viewer using the `termion` crate. It demonstrates how to build interactive Terminal User Interfaces (TUI) and handle terminal events.

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)

### Building
Each project is a separate Cargo package. To build a specific project:

```bash
cd src/<project_name>
cargo build
```

### Running
To run a specific project:

```bash
cd src/<project_name>
cargo run
```
