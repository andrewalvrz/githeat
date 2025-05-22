# 🔥 githeat

> A blazing-fast Git contribution heatmap — right from your terminal.

`githeat` visualizes which files are most actively modified in your Git repository and who’s contributing to them. Perfect for onboarding, code review, and understanding your codebase's "hot zones".

---

## 📸 Demo

┌────────────────────────────────────────────────────── Git Heatmap ──────────────────────────────────────────────────────┐
│                                                                                                                        │
│ src/main.rs                              █████████████████████████████████████████████████████████ 123 commits        │
│ src/ui/mod.rs                            ██████████████████████████████████████████████            82 commits         │
│ src/parser/git_parser.rs                 ██████████████████████████████                        67 commits             │
│ src/telemetry.rs                         ████████████████████                            52 commits                   │
│ README.md                                ████████████                               34 commits                       │
│ src/auth/session.rs                      ████████                          23 commits                                │
│ Cargo.toml                               █████                            19 commits                                 │
│ .github/workflows/ci.yml                 ██                               10 commits                                 │
│ src/lib.rs                               █                                7 commits                                  │
│                                                                                                                        │
└────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘

---

## 🚀 Features

- 📁 Heatmap of file modification frequency
- 🧠 Author-level contribution stats
- ⏳ Filter by date (e.g. `--since 30`)
- 🧩 Works with all Git repos
- 🖥️ Terminal UI using `ratatui`


---

## 📄 License

MIT © [Andrew Alvarez](https://github.com/andrewalvrz)


---

## 🛠 Installation

### Via Cargo (Rust)
```bash
cargo install --git https://github.com/yourname/githeat.git



