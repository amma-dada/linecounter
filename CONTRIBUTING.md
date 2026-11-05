# Contributing to LineCounter

Thank you for your interest in contributing to **LineCounter**! We welcome bug reports, feature requests, documentation improvements, and code contributions.

---

## 📜 Licensing Agreement

By submitting a Pull Request or contributing code to this repository, you agree that your contributions will be licensed under the **Apache License, Version 2.0** as outlined in the [LICENSE](LICENSE) file, without any additional terms or restrictions.

---

## 🛠️ Development Setup

LineCounter is written in **Rust** (with a **Tauri 2** desktop harness and native web frontend):

### Prerequisites:
- **Rust & Cargo** (stable, 1.77+)
- **Node.js** (for running frontend validation tools)
- **Linux Packages** (WebKitGTK, GTK3/4, Flatpak if building packages)

### Building Locally:
```bash
# Clone the repository
git clone https://github.com/amma-dada/linecounter.git
cd linecounter

# Build the desktop binary
cd src-tauri
cargo build --release

# Run the app
cd ..
./linecounter-desktop
```

---

## 📋 Pull Request Process

1. **Fork the Repository**: Create your own feature branch (`git checkout -b feature/amazing-feature`).
2. **Follow Code Conventions**:
   - Keep line counting logic SIMD-optimized and multithreaded (`memchr` + `rayon`).
   - Maintain the obsidian dark theme and smooth 60fps UI performance.
   - Test UI responsiveness on both small and large files.
3. **Run Checks**:
   - `cd src-tauri && cargo check`
   - Test CLI: `cargo test`
4. **Submit Your PR**:
   - Provide a clear, descriptive title and summary of changes.
   - Reference any related issues.

---

## 🐛 Reporting Issues

If you find a bug or have a suggestion:
1. Search existing [Issues](https://github.com/amma-dada/linecounter/issues) to avoid duplicates.
2. Open a new issue detailing:
   - Your OS, desktop environment, and hardware.
   - Steps to reproduce the bug.
   - Expected vs actual behavior.

---

## 🤝 Community & Support

- **Author**: Mortada (Dada) ([@amma-dada](https://github.com/amma-dada))
- **Ko-fi**: [ko-fi.com/dada0mortada](https://ko-fi.com/dada0mortada)
- **Email**: [mortada6333@gmail.com](mailto:mortada6333@gmail.com)
