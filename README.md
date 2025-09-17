# 🎬 TV Tracker

A modern, cross-platform desktop application for tracking your favorite movies and TV shows. Built with Rust, Tauri, and Vue.js for blazing-fast performance and native OS integration.

![TV Tracker](https://img.shields.io/badge/Platform-Desktop-blue)
![Tauri](https://img.shields.io/badge/Built%20with-Tauri-orange)
![Vue.js](https://img.shields.io/badge/Frontend-Vue.js-green)
![Rust](https://img.shields.io/badge/Backend-Rust-red)

## ✨ Features

- 🔥 **Trending Content**: Browse trending movies and TV shows from TMDB
- 🔍 **Smart Search**: Find any movie or TV show with real-time search
- 📝 **Personal Watchlist**: Save movies and shows you want to watch
- ⭐ **Rating System**: Rate content with a 5-star system (0.5 increments)
- 📊 **Watched History**: Track what you've watched with timestamps
- 🎨 **Modern UI**: Clean, responsive interface with dark theme
- ⚡ **Native Performance**: Built with Tauri for optimal speed
- 💾 **Local Storage**: All your data stored locally with SQLite

## 🏗️ Architecture

```mermaid
graph TB
    A[Vue.js Frontend] -->|IPC| B[Tauri Runtime]
    B --> C[Rust Backend]
    C --> D[SQLite Database]
    C --> E[TMDB API]
    
    subgraph "Frontend (Vue.js)"
        F[Components]
        G[Stores (Pinia-like)]
        H[Router]
        I[Services]
    end
    
    subgraph "Backend (Rust)"
        J[Tauri Commands]
        K[Logic Crate]
        L[Database Layer]
        M[API Client]
    end
    
    A --> F
    A --> G
    A --> H
    A --> I
    
    C --> J
    C --> K
    C --> L
    C --> M
```

### Technology Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| **Frontend** | Vue.js 3 + TypeScript | Reactive UI with type safety |
| **Build Tool** | Vite | Fast development and building |
| **Runtime** | Tauri 2 | Native desktop app framework |
| **Backend** | Rust | High-performance system operations |
| **Database** | SQLite + rusqlite | Local data storage |
| **HTTP Client** | reqwest | API communication |
| **Package Manager** | pnpm | Fast, efficient package management |

## 🚀 Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- **Node.js** (v18 or higher)
- **pnpm** (v8 or higher)
- **Rust** (latest stable)
- **Tauri CLI** (v2.x)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/tv-tracker.git
   cd tv-tracker
   ```

2. **Install pnpm** (if not already installed)
   ```bash
   npm install -g pnpm
   ```

3. **Install dependencies**
   ```bash
   pnpm install
   ```

4. **Install Rust** (if not already installed)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

5. **Install Tauri CLI**
   ```bash
   cargo install tauri-cli --version "^2.0"
   # OR using pnpm
   pnpm add -g @tauri-apps/cli
   ```

### TMDB API Setup

The application uses The Movie Database (TMDB) API to fetch movie and TV show data.

1. **Create a TMDB account**
   - Go to [themoviedb.org](https://www.themoviedb.org)
   - Sign up for a free account

2. **Get your API key**
   - Navigate to your account settings
   - Go to the API section
   - Request an API key (choose "Developer" option)
   - Copy your API Read Access Token (v4 auth)

3. **Configure the API key**
   
   The application will create a config file automatically. You can add your API key in two ways:
   
   **Option 1: Through the app** (Recommended)
   - Run the application
   - When prompted, enter your TMDB API key
   - The app will validate and save it automatically
   
   **Option 2: Manual configuration**
   - Create the config directory:
     ```bash
     mkdir -p ~/.config/tv
     ```
   - Create the config file:
     ```bash
     echo '[tmdb]' > ~/.config/tv/config.toml
     echo 'api_key = "your_api_key_here"' >> ~/.config/tv/config.toml
     ```

### Development

Start the development server:

```bash
pnpm desktop:dev
```

This command will:
1. Build the Rust backend
2. Start the Vite development server
3. Launch the Tauri development window
4. Enable hot-reload for both frontend and backend

### Building for Production

Create a production build:

```bash
pnpm tauri build
```

The built application will be available in:
- **Windows**: `target/release/bundle/msi/`
- **macOS**: `target/release/bundle/dmg/`
- **Linux**: `target/release/bundle/appimage/` or `target/release/bundle/deb/`

## 🛠️ How Tauri Works

Tauri creates a bridge between your web frontend and native system capabilities:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Vue.js App    │    │  Tauri Bridge   │    │   Rust Backend  │
│                 │    │                 │    │                 │
│ ┌─────────────┐ │    │ ┌─────────────┐ │    │ ┌─────────────┐ │
│ │ Components  │ │◄──►│ │ IPC Channel │ │◄──►│ │   Commands  │ │
│ └─────────────┘ │    │ └─────────────┘ │    │ └─────────────┘ │
│ ┌─────────────┐ │    │ ┌─────────────┐ │    │ ┌─────────────┐ │
│ │   Stores    │ │    │ │   Events    │ │    │ │  Database   │ │
│ └─────────────┘ │    │ └─────────────┘ │    │ └─────────────┘ │
│ ┌─────────────┐ │    │ ┌─────────────┐ │    │ ┌─────────────┐ │
│ │   Router    │ │    │ │   WebView   │ │    │ │   API Lib   │ │
│ └─────────────┘ │    │ └─────────────┘ │    │ └─────────────┘ │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Key Benefits:

- **Performance**: Native Rust backend handles heavy operations
- **Security**: Sandboxed frontend with controlled system access
- **Bundle Size**: Smaller than Electron (no Node.js runtime)
- **Memory Usage**: Lower memory footprint
- **Native Feel**: True native window decorations and behaviors

## 📁 Project Structure

```
tv-tracker/
├── apps/
│   └── desktop/                 # Vue.js frontend application
│       ├── src/
│       │   ├── components/      # Reusable Vue components
│       │   ├── views/           # Page components
│       │   ├── stores/          # State management
│       │   ├── services/        # API services
│       │   ├── utils/           # Utility functions
│       │   └── assets/          # Static assets
│       ├── package.json
│       └── vite.config.ts
├── crates/
│   ├── logic/                   # Business logic crate
│   │   ├── src/
│   │   │   ├── api/            # TMDB API client
│   │   │   ├── config/         # Configuration management
│   │   │   ├── database/       # SQLite operations
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   └── tauri/                   # Tauri backend crate
│       ├── src/
│       │   ├── commands.rs     # Tauri command handlers
│       │   ├── lib.rs
│       │   └── main.rs
│       ├── Cargo.toml
│       └── tauri.conf.json     # Tauri configuration
├── Cargo.toml                   # Workspace configuration
├── package.json                 # Root package.json
└── README.md
```

## 🔧 Available Scripts

| Command | Description |
|---------|-------------|
| `pnpm desktop:dev` | Start development server |
| `pnpm tauri build` | Build for production |
| `pnpm tauri dev` | Alternative dev command |
| `pnpm prettier:desktop` | Check code formatting |
| `pnpm format:desktop` | Format code |
| `pnpm lint` | Run ESLint |
| `pnpm fix` | Fix linting issues |
| `pnpm rustfmt` | Format Rust code |
| `pnpm isgood` | Check formatting and linting |
| `pnpm begood` | Format and fix all issues |

## 🎯 Usage Guide

### Adding Movies/TV Shows to Watchlist

1. Navigate to the "Popular" section
2. Browse trending content or use the search bar
3. Click the "+" button on any movie or TV show card
4. The item will be added to your watchlist

### Rating Content

1. Click on any movie or TV show to view details
2. Click "Rate This Movie/Show" button
3. Select your rating (0.5 to 5.0 stars)
4. Choose when you watched it (optional)
5. Click "Save Rating"

### Managing Your Watchlist

1. Navigate to the "Watchlist" section
2. Switch between Movies and TV Shows tabs
3. Use the search bar to filter your list
4. Click the trash icon to remove items

### Viewing Watched Content

1. Navigate to the "Watched" section
2. View all your rated content with timestamps
3. Filter by rating or search for specific titles
4. Edit or remove ratings as needed

## 🔐 Privacy & Data

- **Local Storage**: All your data is stored locally on your device
- **No Tracking**: The app doesn't collect or transmit personal data
- **TMDB API**: Only movie/TV metadata is fetched from TMDB
- **Secure**: Tauri's security model protects against common vulnerabilities

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes
4. Run tests: `pnpm test`
5. Format code: `pnpm begood`
6. Commit changes: `git commit -m 'Add amazing feature'`
7. Push to branch: `git push origin feature/amazing-feature`
8. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [TMDB](https://www.themoviedb.org/) for providing the movie and TV show data
- [Tauri](https://tauri.app/) for the amazing desktop app framework
- [Vue.js](https://vuejs.org/) for the reactive frontend framework
- [Rust](https://www.rust-lang.org/) for the powerful systems programming language

## 📞 Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/yourusername/tv-tracker/issues) page
2. Create a new issue with detailed information
3. Join our [Discord community](https://discord.gg/yourdiscord) for real-time help

---

**Made with ❤️ using Tauri, Vue.js, and Rust**