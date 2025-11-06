# TRustBrowser - Multi-Instance Edition

A minimal, focused desktop web browser application built with Rust, GTK4, and WebKit2GTK.

The object of this exercise was to create the most simplistic browser possible focussing on speed to display as the primary goal.  The main use case was as a place to run web applications like WhatsApp Web, Google Calendar, and Messenger without the crashes experienced using mainstream browsers (being a Linux user there is no native application available - cheers Meta - https://larcombe.tech/blog/meta-whatsapp-data-strategy.html).

This branch features a **multi-instance** design where each browser window loads a single configured URL from a config file. You can run multiple instances simultaneously, each dedicated to a specific web application. TRustBrowser stands for Tony's Rust Browser.

## Features

- **Multi-Instance Support**: Run multiple browser windows simultaneously, each dedicated to a specific web application
- **Config-Based URLs**: Load sites from a simple configuration file (`sites.conf`)
- **Minimal UI**: Clean interface with no navigation controls - just the web content and a status bar
- **Domain Isolation**: Each instance stays within its configured domain; external links open in the default browser (Chromium)
- **Web Rendering**: Uses WebKit rendering engine for full modern web standards support
- **Status Bar**: Shows loading status and current page information
- **Lightweight**: Fast startup with minimal resource usage

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

## Configuration

The browser loads URLs from a `sites.conf` file in the project directory. The format is simple:

```
# TRustBrowser Sites Configuration
# Format: Name | URL

WhatsApp | https://web.whatsapp.com
Google Calendar | https://calendar.google.com/calendar/u/0/r
Messenger | https://www.messenger.com/e2ee/t/25417791761168110/
```

Each line contains a display name and URL separated by a pipe (`|`). Lines starting with `#` are comments.

## Installation

### Quick Install

```bash
# Build the project
cargo build --release

# Run the installation script
./install.sh
```

This will:
- Install the binary as `rbrowser` in `~/.local/bin/`
- Install launcher scripts for each site
- Create desktop menu entries for application launcher
- Copy config to `~/.config/trustbrowser/sites.conf`

### Manual Installation

If you prefer manual installation:
```bash
# Copy binary
cp target/release/trustbrowser ~/.local/bin/rbrowser

# Create config directory
mkdir -p ~/.config/trustbrowser
cp sites.conf ~/.config/trustbrowser/

# Copy launcher scripts (optional)
cp scripts/rbrowser-* ~/.local/bin/

# Copy desktop files (optional)
cp desktop-files/*.desktop ~/.local/share/applications/
update-desktop-database ~/.local/share/applications
```

## Usage

### Easy Launching (After Installation)

Once installed, you have multiple ways to launch:

**1. From Application Menu**
- Search for "TRustBrowser WhatsApp", "TRustBrowser Calendar", or "TRustBrowser Messenger"
- Click to launch

**2. Using Named Scripts**
```bash
rbrowser-whatsapp     # Launch WhatsApp
rbrowser-calendar     # Launch Google Calendar
rbrowser-messenger    # Launch Messenger
rbrowser-all          # Launch all three at once
```

**3. Using Site Names**
```bash
rbrowser whatsapp              # Launch by name
rbrowser messenger             # Case-insensitive
rbrowser "google calendar"     # Spaces need quotes
rbrowser --list                # Show available sites
```

**4. Using Index Numbers**
```bash
rbrowser 0    # WhatsApp (first site)
rbrowser 1    # Google Calendar (second site)
rbrowser 2    # Messenger (third site)
```

### Development Usage

When working on the code, use cargo:

```bash
# Launch by site name
cargo run whatsapp
cargo run messenger

# Launch by index
cargo run 0
cargo run 1

# List available sites
cargo run -- --list
```

### Adding New Sites

Edit `~/.config/trustbrowser/sites.conf` (or `sites.conf` in project directory):
```
GitHub | https://github.com
YouTube | https://youtube.com
```

Then launch with:
```bash
rbrowser github    # By name
rbrowser 3         # By index
```

### Navigation Behavior

- **Domain Isolation**: Each instance only navigates within its configured domain
- **External Links**: Clicking links to external domains opens them in Chromium
- **No Navigation Controls**: The interface is minimal - no back/forward buttons or address bar
- **Independent Sessions**: Each site runs in its own process with separate cookies and storage

## File Locations

After installation:
- **Binary**: `~/.local/bin/rbrowser`
- **Config**: `~/.config/trustbrowser/sites.conf`
- **Launcher scripts**: `~/.local/bin/rbrowser-*`
- **Desktop files**: `~/.local/share/applications/rbrowser-*.desktop`
- **Icon**: `~/.local/share/icons/trustbrowser.png`

## Development

### Project Structure

```
src/
└── main.rs          # Complete browser application
sites.conf           # URL configuration file
```

The entire application is contained in a single `main.rs` file with:
- `Browser` struct managing the GTK window and WebKit view
- `load_config()` function to parse `sites.conf`
- Domain-based navigation policy
- Multi-instance support via unique application IDs
- Clean, minimal UI focused on content

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

- [ ] Desktop file entries for easy launching from application menu
- [ ] System tray integration for quick instance switching
- [ ] Automatic browser selection based on URL patterns
- [ ] Session persistence and restore
- [ ] Optional notification support for web apps
- [ ] Custom per-site settings (zoom level, user agent)

## Branch Differences

This `multi-instance` branch differs from the `master` branch:

**Master branch** features:
- Single browser window with full navigation controls
- Address bar for entering URLs
- Back/forward buttons
- History management
- Keyboard shortcuts for navigation

**Multi-instance branch** features (this branch):
- Multiple independent browser instances
- Config file-based URL loading
- Minimal UI without navigation controls
- Domain isolation per instance
- Dedicated to specific web applications

## Acknowledgments

Built with GTK4, WebKit2GTK, and the Rust ecosystem. Thanks to the maintainers of the gtk4-rs and webkit2gtk-rs projects.
