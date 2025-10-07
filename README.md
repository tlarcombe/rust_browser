# TRustBrowser

A desktop web browser application built with Rust, GTK4, and WebKit2GTK.

The object of this exercise was to create the most simplistic browser possible focussing on speed to display as the primary goal.

This project provides a simple, functional web browser with navigation controls, history management, and keyboard shortcuts. TRustBrowser stands for Tony's Rust Browser.

## Features

- **Web Navigation**: Load and display web pages using WebKit rendering engine
- **Navigation Controls**: Back/forward buttons with history support
- **Address Bar**: URL entry with automatic HTTPS formatting
- **Keyboard Shortcuts**: 
  - `Ctrl+O` - Focus address bar
  - `Alt+Left` - Navigate back
  - `Alt+Right` - Navigate forward
- **Status Bar**: Shows loading status and current page information
- **History Management**: Tracks visited pages for navigation

## Architecture

The application is built with a single-file architecture using:

- **GTK4**: Cross-platform GUI toolkit for the user interface
- **WebKit2GTK**: WebKit-based web engine for rendering web content
- **Rust**: Memory-safe systems programming language

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (version 1.70 or later)
- [Git](https://git-scm.com/)
- GTK4 development libraries
- WebKit2GTK development libraries

#### On Ubuntu/Debian:
```bash
sudo apt update
sudo apt install libgtk-4-dev libwebkit2gtk-4.1-dev build-essential
```

#### On Fedora:
```bash
sudo dnf install gtk4-devel webkit2gtk4.1-devel
```

#### On Arch Linux:
```bash
sudo pacman -S gtk4 webkit2gtk-4.1
```

### Building from Source

1. Clone the repository:
```bash
git clone https://github.com/tlarcombe/rust_browser.git
cd rust_browser
```

2. Build the project:
```bash
cargo build --release
```

3. Run the browser engine:
```bash
cargo run
```

### Development Setup

For development, you can build and run in debug mode:

```bash
# Build in debug mode
cargo build

# Run with debug information
cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Usage

### Basic Usage

1. Run the browser:
```bash
cargo run
```

2. The browser will start with a test page displayed
3. Type a URL in the address bar and press Enter to navigate
4. Use the back/forward buttons or keyboard shortcuts to navigate

### Keyboard Shortcuts

- **Ctrl+O**: Focus the address bar and select all text
- **Alt+Left**: Go back in history
- **Alt+Right**: Go forward in history
- **Enter**: Navigate to URL in address bar

## Development

### Project Structure

```
src/
└── main.rs          # Complete browser application
```

The entire application is contained in a single `main.rs` file with:
- `Browser` struct managing the GTK window and WebKit view
- Navigation controls (back/forward buttons, address bar)
- History management with Vec-based storage
- Event handling for user interactions
- Keyboard shortcut support

### Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and add tests
4. Run the test suite: `cargo test`
5. Format your code: `cargo fmt`
6. Run the linter: `cargo clippy`
7. Commit your changes: `git commit -m "Description"`
8. Push to your fork: `git push origin feature-name`
9. Submit a pull request

### Testing

Run the test suite:
```bash
cargo test
```

Test the browser manually:
1. Run `cargo run`
2. Try navigating to different websites
3. Test keyboard shortcuts
4. Verify back/forward functionality

## Dependencies

Key dependencies include:

- `gtk4` - GTK4 bindings for Rust
- `webkit2gtk` - WebKit2GTK bindings for web rendering
- `gdk` - GDK bindings for keyboard events

See `Cargo.toml` for the complete dependency list.

## Performance

The browser leverages WebKit's performance optimizations:

- Hardware-accelerated rendering via WebKit
- Efficient memory management through Rust's ownership system
- Fast startup time with minimal dependencies
- Responsive UI built on GTK4's modern event system

## Browser Compatibility

Supports modern web standards through WebKit:

- HTML5 and CSS3
- JavaScript (ES6+)
- Modern web APIs
- Mobile-responsive websites

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] Bookmarks management
- [ ] Multiple tabs support
- [ ] Download manager
- [ ] Settings/preferences dialog
- [ ] Developer tools integration
- [ ] Extensions support

## Acknowledgments

Built with GTK4, WebKit2GTK, and the Rust ecosystem. Thanks to the maintainers of the gtk4-rs and webkit2gtk-rs projects.
