# 📄 LineCounter

<div align="center">
  <img src="linecounter.png" width="96" height="96" alt="LineCounter Logo" style="border-radius: 20px;" />
  <h2>LineCounter</h2>
  <p><b>Ultra-fast, accurate code and line counter with native desktop GUI and terminal CLI.</b></p>
  <p>Counts <b>every single line</b> — including pure empty lines (<code>\n\n</code>), whitespace-only lines, content lines, words, and characters.</p>

  <p>
    <a href="https://github.com/amma-dada/linecounter/releases"><img src="https://img.shields.io/github/v/release/amma-dada/linecounter?color=blue&label=Download" alt="Latest Release"></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/License-Apache_2.0-blue.svg" alt="License: Apache 2.0"></a>
    <a href="TRADEMARKS.md"><img src="https://img.shields.io/badge/Logo-Protected-green.svg" alt="Logo Trademark Policy"></a>
    <a href="https://ko-fi.com/dada0mortada"><img src="https://img.shields.io/badge/Support-Ko--fi-ff5e5b.svg" alt="Support on Ko-fi"></a>
  </p>
</div>

---

## 🚀 Installation

### 🪟 Windows (Portable Standalone `.exe`)
No installer, no dependencies, no Python, no Node, and no ports or local servers needed:
1. Download **[LineCounter.exe](https://github.com/amma-dada/linecounter/releases/latest/download/LineCounter.exe)** from the [Releases page](https://github.com/amma-dada/linecounter/releases).
2. Double-click **`LineCounter.exe`** to launch immediately.

---

### 🐧 Linux (Native Desktop & CLI)
One-click installer registering desktop entry, icons, and terminal command:
```bash
git clone https://github.com/amma-dada/linecounter.git
cd linecounter
./install.sh
```
Or download the standalone binary directly from [Releases](https://github.com/amma-dada/linecounter/releases).

---

### 📦 Flatpak (Sandboxed)
```bash
./flatpak/build-flatpak.sh
flatpak run com.linecounter.desktop
```

---

## ✨ Features

- **Accurate Empty Line Counting**: Distinguishes between pure empty lines (`\n`), whitespace-only lines, and content lines.
- **Ultra-Fast Parallel Engine**: SIMD-accelerated line scanning in Rust (`memchr` + `rayon`).
- **Drag & Drop**: Drop files or entire folders to scan nested subdirectories automatically.
- **Single File Inspector**: Line-by-line syntax viewer, empty-line navigation, and 1-click cleaned code export.
- **In-App Code Editor**: Built-in editor with Find & Replace (<kbd>Ctrl</kbd>+<kbd>F</kbd>, <kbd>Ctrl</kbd>+<kbd>H</kbd>), jump to line (<kbd>Ctrl</kbd>+<kbd>G</kbd>), and soft line wrap (<kbd>Ctrl</kbd>+<kbd>W</kbd>).
- **Batch Scanner & Export**: Scan directories and export results as Markdown tables or CSV files.
- **Self-Contained**: Ships with all dependencies bundled inside the application binary.

---

## ⌨️ Shortcuts

| Shortcut | Action |
| :--- | :--- |
| <kbd>Ctrl</kbd> + <kbd>F</kbd> | Open Find Widget |
| <kbd>Ctrl</kbd> + <kbd>H</kbd> | Open Find & Replace |
| <kbd>Ctrl</kbd> + <kbd>E</kbd> | Toggle Code Editor Mode |
| <kbd>Ctrl</kbd> + <kbd>S</kbd> | Save Edited File |
| <kbd>Ctrl</kbd> + <kbd>G</kbd> | Go to Line |
| <kbd>Ctrl</kbd> + <kbd>W</kbd> | Toggle Soft Line Wrap |
| <kbd>Alt</kbd> + <kbd>E</kbd> / <kbd>Alt</kbd> + <kbd>Shift</kbd> + <kbd>E</kbd> | Next / Previous Empty Line |
| <kbd>Ctrl</kbd> + <kbd>1</kbd> - <kbd>4</kbd> | Switch Tabs (Drag & Drop, Single File, Live Paste, Batch Scanner) |

---

## 💻 Terminal CLI

Run headless line counts in scripts or CI:
```bash
# Count a single file
linecounter path/to/file.py

# Recursively scan a folder
linecounter path/to/project
```

---

## 📬 Contact

Developed and maintained by **Mortada (Dada)**.

- **LinkedIn**: [Ahmed Mortada Zidane](https://www.linkedin.com/in/ahmed-mortada-zidane)
- **Email**: [mortada6333@gmail.com](mailto:mortada6333@gmail.com) — Feedback
- **Ko-fi**: [ko-fi.com/dada0mortada](https://ko-fi.com/dada0mortada) — Support ongoing maintenance

---

## 📄 License & Trademarks

- **Code License**: [Apache License 2.0](LICENSE) — see [NOTICE](NOTICE).
- **Logo & Artwork**: The official dial logo is copyright and trademark of **Mortada (Dada)**. The name "LineCounter" is free and unrestricted for anyone to use. See [TRADEMARKS.md](TRADEMARKS.md).
- **Contributing**: See [CONTRIBUTING.md](CONTRIBUTING.md).
