# YouTube Music Desktop

A lightweight YouTube Music desktop wrapper built with **Tauri v2** and **Rust** — no Electron, no bloat, just the music.

![Platform](https://img.shields.io/badge/platform-Windows-blue?style=for-the-badge)
![Tauri](https://img.shields.io/badge/Tauri-v2-orange?style=for-the-badge)
![Rust](https://img.shields.io/badge/Rust-backend-orange?style=for-the-badge&logo=rust)
![License](https://img.shields.io/github/license/yuvrajpinkman/yt_music_advance?style=for-the-badge)
<!-- ![Release](https://img.shields.io/github/v/release/yuvrajpinkman/yt_music_advance?style=for-the-badge)-->
---

## A Resource-friendly Alternative to Electron

- **3MB installer** — downloads in seconds
- **50MB RAM usage** — light enough to forget it's running
- **Pure Rust backend** — no Node.js, no npm runtime overhead
- **Zero third-party UI bloat** — no React, no Vue, no framework tax

---

## Features

- Loads `music.youtube.com` as a native desktop app
- Minimize to system tray on close — keeps playing in background
- System tray with Show / Quit options
- Now Playing desktop notifications on track change

---

## Download

Head to [Releases](https://github.com/yuvrajpinkman/yt_music_advance/releases) and download the latest `.exe` installer.

- Installs to `%LocalAppData%`

---

## Prerequisites

- Windows 10/11
- [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) — comes pre-installed on Windows 11, auto-downloaded on Windows 10 if missing

---

## Build from Source

**Requirements:**

- [Rust](https://rustup.rs/)
- [Node.js v18+](https://nodejs.org/)
- [VS Build Tools 2022](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with "Desktop development with C++"

```bash
# Clone the repo
git clone https://github.com/yuvrajpinkman/yt_music_advance.git
cd yt_music_advance

# Install dependencies
npm install

# Run in dev mode
npm run tauri dev

# Build installer
npm run tauri build
```

The installer will be in `src-tauri/target/release/bundle/nsis/`.

---

## License

MIT — see [LICENSE](LICENSE)
