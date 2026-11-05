# 📄 LineCounter 1.0 — High-Performance Code & Line Counter Desktop Application

<div align="center">
  <img src="linecounter-64.png" width="96" height="96" alt="LineCounter Logo" style="border-radius: 20px; box-shadow: 0 10px 30px rgba(0,0,0,0.5);" />
  <h2>LineCounter 1.0</h2>
  <p><b>Ultra-fast, accurate code and line counter with native GUI, Flatpak sandbox, and CLI.</b></p>
  <p>Accurately counts <b>every single line</b> — including pure empty lines (<code>\n\n</code>), whitespace-only lines, content lines, words, characters, and line endings.</p>

  <p>
    <a href="LICENSE"><img src="https://img.shields.io/badge/License-Apache_2.0-blue.svg" alt="License: Apache 2.0"></a>
    <a href="TRADEMARKS.md"><img src="https://img.shields.io/badge/Trademark-Protected-green.svg" alt="Trademark Policy"></a>
    <a href="https://ko-fi.com/dada0mortada"><img src="https://img.shields.io/badge/Support-Ko--fi-ff5e5b.svg" alt="Support on Ko-fi"></a>
  </p>
</div>

---

## 🌟 Key Highlights & Features

- **Accurate Empty Line Counting**: Unlike naive tools (`wc -l` or basic scripts) that ignore blank lines or miscount trailing editor newlines, LineCounter accurately identifies **Pure Empty** lines (`\n`), **Whitespace-only** lines (`   \t `), and **Content** lines.
- **Ultra-Fast Parallel Scanning Engine**: Powered by Rust (`rayon` + `memchr` SIMD line scanner). Capable of scanning **hundreds of thousands of files in seconds** with instant binary file detection and skipping.
- **Standalone Native Application & Flatpak**: Available as a native compiled desktop application for Linux (`linecounter-desktop`), standalone terminal CLI (`linecounter`), and Flatpak package (`com.linecounter.desktop`).
- **Smooth 60fps Infinite Scroll**: Uses `IntersectionObserver` chunked streaming to inspect files with tens of thousands of lines effortlessly without browser lag or UI stutter.
- **In-App Code Editor with Find & Replace**: Press <kbd>Ctrl</kbd> + <kbd>E</kbd> to toggle live editing, <kbd>Ctrl</kbd> + <kbd>F</kbd> for floating search with match counters (`1 of 3`), and <kbd>Ctrl</kbd> + <kbd>H</kbd> for full find & replace with replace-all.
- **Built-in Inline `# Go to` Line**: Jump straight to any line number with the inline number box beside `# Go to` or via <kbd>Ctrl</kbd> + <kbd>G</kbd> — zero annoying browser alert popups.
- **Native OS Drag & Drop**: Drag and drop any single file or entire directory directly onto the app window. Nested subdirectories are automatically traversed in parallel.
- **Single File Inspector**: Line-by-line syntax inspection, empty-line navigation (<kbd>Alt</kbd>+<kbd>E</kbd> / <kbd>Alt</kbd>+<kbd>Shift</kbd>+<kbd>E</kbd>), longest-line indicator, and 1-click **Cleaned Code** copy (copies content with blank lines stripped).
- **Batch Table with Chunked Rendering**: Instantly renders directory results, supporting real-time search, extension filter pills, sorting, and export to Markdown table or CSV file.
- **Restrained Dark UI**: Modern Obsidian slate palette (`#070a12`), Indigo brand accents (`#6366f1`), and Emerald metric badges (`#10b981`).
- **Zero Driver Noise**: Completely suppresses Mesa Intel Vulkan `FINISHME` notices and `libGL: Can't open configuration file /etc/drirc` warnings on Linux.

---

## ❓ What Does "Wrap" (Soft Line Wrap) Do?

In the **Single File Inspector** tab, you will find the **↩ Wrap** button (<kbd>Ctrl</kbd> + <kbd>W</kbd>):

| Wrap State | Visual Behavior | When to Use |
| :--- | :--- | :--- |
| **Wrap OFF** *(Default)* | Long lines stretch horizontally without breaking. A horizontal scrollbar appears. | When examining tabular data, wide arrays, CSVs, or verifying column alignment. |
| **Wrap ON** *(Soft Wrap)* | Long lines automatically break and wrap within the visible window width so you can read everything without horizontal scrolling. | When reading long comments, prose, documentation, or nested statements on smaller screens. |

> [!NOTE]
> **Soft Line Wrap is purely visual.** It only affects how text is displayed in the preview viewer. It never modifies your original file or inserts real newline characters into your code.

---

## 🏗️ Architecture & Stack

LineCounter is engineered with a modern Rust + Tauri desktop architecture:

1. **Native Tauri 2 / Rust Core (`src-tauri/`)**:
   - High-performance native compiled binary (`linecounter-desktop`).
   - Line counting executed at CPU memory bandwidth limits using Rust's `memchr` SIMD and `rayon` work-stealing threadpool.
   - Native OS file and directory dialogs via `tauri-plugin-dialog`.
   - Minimal memory footprint (< 35 MB RAM).
2. **High-Performance CLI (`src/`)**:
   - Standalone CLI executable (`linecounter`) with instant terminal output and JSON/CSV streaming.
3. **Flatpak Sandbox (`flatpak/`)**:
   - Sandboxed desktop package with Wayland and X11 support, fully compliant with GNOME Software, Flatseal, and AppStream specifications.

---

## 🚀 Installation & Running

### 1. One-Click Linux Desktop Install:
```bash
git clone https://github.com/amma-dada/linecounter.git
cd linecounter
./install.sh
```
This registers the application in your desktop menu with the official vector icon and terminal launcher `linecounter`.

### 2. Local Flatpak Build & Install:
```bash
./flatpak/build-flatpak.sh
```
Launch the Flatpak sandbox anytime using:
```bash
flatpak run com.linecounter.desktop
```

### 3. Run Directly:
```bash
./linecounter-desktop
```

---

## ⌨️ Built-in Keyboard Shortcuts

| Shortcut | Action | Description |
| :--- | :--- | :--- |
| <kbd>Ctrl</kbd> + <kbd>F</kbd> | **Find Widget** | Opens sleek floating Find & Match widget with count badge (`1 of 3`) |
| <kbd>Ctrl</kbd> + <kbd>H</kbd> | **Find & Replace** | Opens Find widget with expandable Replace row, Replace all & Replace ↵ |
| <kbd>Ctrl</kbd> + <kbd>E</kbd> | **Toggle Edit Mode** | Switches Single File tab between read-only View and dynamic Code Editor |
| <kbd>Ctrl</kbd> + <kbd>S</kbd> | **Save File** | Saves edited file directly to disk and updates metrics |
| <kbd>Ctrl</kbd> + <kbd>G</kbd> | **Go to Line** | Focuses the inline line number box without popup dialogs |
| <kbd>Ctrl</kbd> + <kbd>W</kbd> | **Toggle Wrap** | Toggles soft line wrapping on long lines |
| <kbd>Alt</kbd> + <kbd>E</kbd> | **Next Empty Line** | Jumps and highlights the next empty line |
| <kbd>Alt</kbd> + <kbd>Shift</kbd> + <kbd>E</kbd> | **Prev Empty Line** | Jumps to the previous empty line |
| <kbd>Ctrl</kbd> + <kbd>O</kbd> | **Browse Single File** | Opens the native OS file picker |
| <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>O</kbd> | **Browse Folder** | Opens the native OS folder picker |
| <kbd>Ctrl</kbd> + <kbd>1</kbd> | **Drag & Drop** | Switches to Drag & Drop tab |
| <kbd>Ctrl</kbd> + <kbd>2</kbd> | **Single File** | Switches to Single File Inspector |
| <kbd>Ctrl</kbd> + <kbd>3</kbd> | **Live Paste** | Switches to Live Paste Counter |
| <kbd>Ctrl</kbd> + <kbd>4</kbd> | **Batch Scanner** | Switches to Batch Scanner tab |
| <kbd>Enter</kbd> / <kbd>Shift</kbd>+<kbd>Enter</kbd> | **Find Next / Prev** | Navigates through search matches in Find widget |
| <kbd>Esc</kbd> | **Close / Dismiss** | Dismisses Find widget, Help modal, or Follow dialog |

---

## 💻 Terminal CLI Mode

For headless servers, automation scripts, and CI/CD pipelines:

### Single File Inspection:
```bash
linecounter /path/to/main.py
```
```text
============================================================
 📄 File: /path/to/main.py
============================================================
 TOTAL LINES (All):   142  (Empty lines included)
 Empty Lines:         28   (Pure: 24, Whitespace-only: 4)
 Content Lines:       114
 File Size:           4.8 KB (4,915 bytes)
 Total Words:         620
 Total Characters:    4,892
 Detected Line End:   LF (Linux / Unix)
============================================================
```

### Recursive Directory Scan:
```bash
linecounter /path/to/project
```

---

## 🤝 Author & Connect

Developed and maintained by **Mortada (Dada)**.

- **GitHub**: [@amma-dada](https://github.com/amma-dada) — Source code, stars, issues & releases
- **LinkedIn**: [Ahmed Mortada Zidane](https://www.linkedin.com/in/ahmed-mortada-zidane) — Career updates and networking
- **Email**: [mortada6333@gmail.com](mailto:mortada6333@gmail.com) — Direct inquiries and feedback
- **Ko-fi**: [ko-fi.com/dada0mortada](https://ko-fi.com/dada0mortada) — Support ongoing maintenance and buy a coffee

---

## 🗑️ Uninstallation

- **Native Linux App**: Run `./uninstall.sh` to remove the binary, desktop shortcut, and all registered icons.
- **Flatpak Sandbox**: Run `flatpak uninstall com.linecounter.desktop`.

---

## 📄 License & Trademarks

- **Code License**: Licensed under the **Apache License, Version 2.0** — see [LICENSE](LICENSE) and [NOTICE](NOTICE) for details.
- **Trademarks & Branding**: LineCounter name and official logo are protected marks of Mortada (Dada). See [TRADEMARKS.md](TRADEMARKS.md) for usage and branding policy.
- **Contributing**: Contributions are welcome under the Apache 2.0 license. See [CONTRIBUTING.md](CONTRIBUTING.md).

Copyright 2026 **Mortada (Dada)**.
