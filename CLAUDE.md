# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based browser engine project starting from an empty repository.

## Initial Setup Commands

Since this is a new project, you'll likely need to initialize it:

```bash
# Initialize new Rust project
cargo init

# Or for a library
cargo init --lib
```

## Common Rust Development Commands

```bash
# Build the project
cargo build

# Run the project
cargo run

# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Run clippy linter
cargo clippy

# Build for release
cargo build --release

# Update dependencies
cargo update

# Add a dependency
cargo add <crate_name>
```

## Browser Engine Architecture Considerations

When developing this browser engine, consider these key architectural components:

- **HTML Parser**: Parse HTML documents into DOM trees
- **CSS Parser**: Parse CSS stylesheets and compute styles
- **Layout Engine**: Calculate element positions and sizes
- **Rendering Engine**: Paint elements to the screen
- **JavaScript Engine**: Execute JavaScript (consider integrating existing engines)
- **Networking**: Handle HTTP/HTTPS requests and responses
- **DOM API**: Provide JavaScript-accessible DOM interfaces

## Recommended Crate Dependencies

Consider these Rust crates for browser engine development:

- `html5ever` - HTML parsing
- `cssparser` - CSS parsing
- `webrender` - GPU-accelerated rendering
- `tokio` - Async runtime for networking
- `reqwest` or `hyper` - HTTP client/server
- `serde` - Serialization/deserialization
- `winit` - Window creation and event handling
- `wgpu` - Graphics API abstraction

## Development Workflow

1. Start with basic HTML parsing and DOM representation
2. Add CSS parsing and style computation
3. Implement layout algorithms (box model, flexbox, grid)
4. Add rendering capabilities
5. Integrate networking for resource loading
6. Consider JavaScript engine integration as a later phase