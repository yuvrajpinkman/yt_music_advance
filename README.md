# YouTube Music Desktop

A lightweight YouTube Music desktop wrapper built with **Tauri v2** and **Rust** — no Electron, no bloat, just the music.

![Platform](https://img.shields.io/badge/platform-Windows-blue?style=for-the-badge)
![Tauri](https://img.shields.io/badge/Tauri-v2-orange?style=for-the-badge)
![Rust](https://img.shields.io/badge/Rust-backend-orange?style=for-the-badge&logo=rust)
![License](https://img.shields.io/github/license/yuvrajpinkman/yt_music_advance?style=for-the-badge)
![Release](https://img.shields.io/github/v/release/yuvrajpinkman/yt_music_advance?style=for-the-badge)

---

## Why this over ytmdesktop?

| | YouTube Music Desktop (this) | ytmdesktop |
|---|---|---|
| **Framework** | Tauri v2 (Rust) | Electron |
| **RAM Usage** | ~50MB | ~300MB+ |
| **Installer Size** | ~3MB | ~150MB+ |
| **Third-party bloat** | None | Heavy |
| **Backend** | Pure Rust | Node.js |
| **WebView** | System WebView2 | Bundled Chromium |

Tauri uses your system's native WebView2 (already installed with Windows) instead of bundling an entire Chromium browser like Electron does. That's the difference.

---

## Features

- Loads `music.youtube.com` as a native desktop app
- Minimize to system tray on close — keeps playing in background
- System tray with Show / Quit options
- Now Playing desktop notifications on track change
- Notification toggle (On/Off) from tray submenu
- Single instance lock — second launch focuses existing window
- User-level install — no admin rights required

---

## Download

Head to [Releases](https://github.com/yuvrajpinkman/yt_music_advance/releases) and download the latest `.exe` installer.

- No admin prompt required
- Installs to `%LocalAppData%` (just like VSCode)
- Uninstall cleanly from Settings → Apps

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