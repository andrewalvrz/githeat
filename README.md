# Githeat — Git Contribution Heatmap

`githeat` is a terminal-based tool that visualizes Git commit activity by file or author using a dynamic TUI interface. It's written in Rust and powered by `git2`, `ratatui`, and `clap`.

## 🚀 Features

- 🔥 Heatmap of file or author commit activity
- 📅 Filter by time (`--since`, `--since-date`)
- 🧑‍💻 Group by author or file
- 🧹 Sort output by commit count (`--sort`)
- 🎯 Filter by path or extension (`--path`, `--ext`)
- ⬆️ Limit to top N entries (`--top`)
- 📤 Export to JSON or Markdown (`--export`)

---

## 🧪 Commands

### 🔹 Basic Usage
```sh
githeat                       # Default file heatmap
githeat --by-author           # Author contribution heatmap
```

### 🔹 Time Filtering
```sh
githeat --since 30            # Last 30 days
githeat --since-date 2025-01-01  # Since specific date
```

### 🔹 Sorting & Limiting
```sh
githeat --top 10              # Top 10 files or authors
githeat --sort asc            # Sort by ascending commit count
githeat --sort desc           # Sort by descending (default)
```

### 🔹 Filtering by File
```sh
githeat --ext rs              # Only .rs files
githeat --path src            # Only files in src/
```

### 🔹 Exporting
```sh
githeat --export json         # Export to githeat_export.json
githeat --export md           # Export to githeat_export.md
```

### 🔹 Combine Filters
```sh
githeat --by-author --since-date 2025-01-01

githeat --path src --ext rs --top 5 --sort desc --export json
```

---

## 🖥️ Local Usage (Development Mode)

From your project folder:
```sh
cargo run -- --by-author
cargo run -- --path src --top 5 --sort desc
```

---

## 📦 Make `githeat` a Global Command

### 🔧 Install it globally
```sh
cargo install --path .
```

This installs the binary into:
```
C:\Users\<your_username>\.cargo\bin
```

### 🧭 Add to PATH (Permanent)
1. Open **Environment Variables** in Windows
2. Under **User Variables**, edit `Path`
3. Add this new entry:
```
C:\Users\<your_username>\.cargo\bin
```
4. Click OK and restart PowerShell

### ✅ Now use from anywhere:
```sh
githeat --since 14 --top 5
```

---

## 🔧 Dev Notes

- Built with Rust 2021
- Depends on: `git2`, `ratatui`, `crossterm`, `clap`, `serde`, `serde_json`, `chrono`
- TUI is rendered via `ratatui`
- `git2` parses repo history and diffs

---

## 📄 License
MIT — © 2025 Andrew Alvarez

---

Contributions, issues, and PRs welcome!

Made with ❤️ for developers who ship.
