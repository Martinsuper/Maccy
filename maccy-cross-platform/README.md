# Maccy Cross-Platform

A lightweight, cross-platform clipboard manager built with Tauri 2.x, React, and Rust.

## Features

- 🚀 **Lightweight & Fast**: Built with Tauri, uses minimal system resources (~5MB binary, 30-80MB RAM)
- 🌍 **Cross-Platform**: Supports macOS, Windows, and Linux
- ⌨️ **Keyboard-First**: Quick access with global hotkeys
- 🔍 **Smart Search**: Fuzzy search across clipboard history
- 📌 **Pin Items**: Keep important items at the top
- 🎨 **Native UI**: Clean, modern interface with Tailwind CSS
- 🔒 **Privacy-Focused**: Local storage, no cloud sync

## Tech Stack

- **Frontend**: React 18 + TypeScript + Tailwind CSS
- **Backend**: Rust + Tauri 2.x
- **State Management**: Zustand
- **Database**: SQLite (via rusqlite)
- **Build Tool**: Vite

## Project Structure

```
maccy-cross-platform/
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── platform/       # Platform abstraction traits
│   │   ├── platform_impl/  # Platform-specific implementations
│   │   ├── commands/       # Tauri command handlers
│   │   ├── models/         # Data models
│   │   └── services/       # Business logic
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── stores/             # Zustand stores
│   ├── hooks/              # Custom React hooks
│   └── services/           # Frontend services
├── package.json
├── vite.config.ts
└── README.md
```

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70+)
- [Node.js](https://nodejs.org/) (18+)
- [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

### Setup

```bash
# Install frontend dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Platform Support

### Phase 1 (MVP)
- ✅ macOS clipboard monitoring
- ⏳ Windows clipboard monitoring
- ⏳ Linux clipboard monitoring (X11/Wayland)
- ✅ Basic UI with search
- ✅ History persistence
- ⏳ Global hotkeys
- ⏳ System tray

### Phase 2 (Advanced Features)
- [ ] Paste simulation
- [ ] Pin/favorites functionality
- [ ] Rich content (images, HTML, RTF)
- [ ] Settings interface
- [ ] Ignore rules

### Phase 3 (Production Ready)
- [ ] Launch at login
- [ ] Auto-update
- [ ] OCR support
- [ ] Internationalization
- [ ] Accessibility

## Architecture

### Platform Abstraction Layer

The application uses a trait-based architecture to abstract platform-specific functionality:

```rust
// 7 core platform traits
trait ClipboardPlatform { ... }
trait HotkeyPlatform { ... }
trait WindowPlatform { ... }
trait TrayPlatform { ... }
trait StoragePlatform { ... }
trait AutoStartPlatform { ... }
trait OcrPlatform { ... }
```

Each trait has platform-specific implementations:
- `macos/`: Uses Cocoa, CoreGraphics, Carbon APIs
- `windows/`: Uses Win32 APIs
- `linux/`: Uses xclip, X11, D-Bus (Wayland)

## License

MIT License - see LICENSE file for details

## Contributing

Contributions are welcome! Please read our contributing guidelines before submitting PRs.

## Acknowledgments

This project is inspired by the original [Maccy](https://github.com/p0deje/Maccy) for macOS.
