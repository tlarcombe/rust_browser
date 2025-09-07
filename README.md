# Rust Browser Engine

A modern, high-performance browser engine written in Rust. This project implements core browser technologies including HTML parsing, CSS styling, layout computation, and rendering.

## Features

- **HTML Parser**: Full HTML5 parsing with DOM tree construction
- **CSS Engine**: Complete CSS parsing and style computation
- **Layout System**: Advanced layout engine supporting:
  - Box model calculations
  - Flexbox layout
  - CSS Grid layout
  - Responsive design
- **Rendering Engine**: High-performance rendering with GPU acceleration support
- **Networking**: Asynchronous HTTP/HTTPS request handling
- **Resource Loading**: Efficient loading and caching of web resources

## Architecture

The browser engine is built with a modular architecture:

- `html_parser` - HTML document parsing and DOM construction
- `css_parser` - CSS stylesheet parsing and rule matching
- `layout` - Layout computation and positioning algorithms
- `render` - Graphics rendering and display
- `network` - HTTP client and resource fetching
- `dom` - Document Object Model implementation

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (version 1.70 or later)
- [Git](https://git-scm.com/)

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

```rust
use rust_browser::*;

fn main() {
    // Create a new browser instance
    let mut browser = Browser::new();
    
    // Load and render a web page
    browser.navigate("https://example.com").await;
}
```

### API Examples

#### HTML Parsing
```rust
use rust_browser::html_parser::HtmlParser;

let html = "<html><body><h1>Hello World</h1></body></html>";
let dom = HtmlParser::parse(html);
```

#### CSS Styling
```rust
use rust_browser::css_parser::CssParser;

let css = "h1 { color: blue; font-size: 24px; }";
let stylesheet = CssParser::parse(css);
```

## Development

### Project Structure

```
src/
├── main.rs          # Application entry point
├── browser.rs       # Main browser engine
├── html_parser/     # HTML parsing modules
├── css_parser/      # CSS parsing modules  
├── layout/          # Layout computation
├── render/          # Rendering engine
├── network/         # Networking components
└── dom/             # DOM implementation
```

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

Run the full test suite:
```bash
cargo test
```

Run specific test modules:
```bash
cargo test html_parser
cargo test css_parser
cargo test layout
```

## Dependencies

Key dependencies include:

- `html5ever` - HTML parsing
- `cssparser` - CSS parsing
- `tokio` - Async runtime
- `reqwest` - HTTP client
- `winit` - Window management
- `wgpu` - Graphics API
- `serde` - Serialization

See `Cargo.toml` for the complete dependency list.

## Performance

The browser engine is designed for high performance:

- Zero-copy parsing where possible
- GPU-accelerated rendering
- Efficient memory management
- Parallel layout computation
- Optimized network requests

## Browser Compatibility

Currently supports:

- HTML5 standard features
- CSS3 styling and layout
- Modern web standards
- Responsive design

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] JavaScript engine integration
- [ ] WebGL support  
- [ ] Service worker support
- [ ] Progressive Web App features
- [ ] Developer tools integration
- [ ] WebAssembly support

## Acknowledgments

Built with modern Rust ecosystem tools and inspired by other browser engine projects like Servo and Webkit.