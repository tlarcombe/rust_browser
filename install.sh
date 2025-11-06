#!/bin/bash
# Installation script for TRustBrowser multi-instance

set -e  # Exit on error

echo "Installing TRustBrowser..."

# Create directories if they don't exist
mkdir -p "$HOME/.local/bin"
mkdir -p "$HOME/.local/share/applications"
mkdir -p "$HOME/.local/share/icons"

# Copy the binary
echo "Installing binary..."
cp target/release/trustbrowser "$HOME/.local/bin/rbrowser"
chmod +x "$HOME/.local/bin/rbrowser"

# Copy the config file
echo "Installing config file..."
mkdir -p "$HOME/.config/trustbrowser"
if [ ! -f "$HOME/.config/trustbrowser/sites.conf" ]; then
    cp sites.conf "$HOME/.config/trustbrowser/sites.conf"
    echo "  Created default config at ~/.config/trustbrowser/sites.conf"
else
    echo "  Config file already exists, skipping..."
fi

# Copy launcher scripts
echo "Installing launcher scripts..."
cp scripts/rbrowser-* "$HOME/.local/bin/"
chmod +x "$HOME/.local/bin"/rbrowser-*

# Copy desktop files
echo "Installing desktop entries..."
cp desktop-files/*.desktop "$HOME/.local/share/applications/"

# Copy icon if it exists
if [ -f "icons/trustbrowser-256.png" ]; then
    cp icons/trustbrowser-256.png "$HOME/.local/share/icons/trustbrowser.png"
    echo "  Installed icon"
fi

# Update desktop database
if command -v update-desktop-database &> /dev/null; then
    echo "Updating desktop database..."
    update-desktop-database "$HOME/.local/share/applications"
fi

echo ""
echo "Installation complete!"
echo ""
echo "You can now launch TRustBrowser instances:"
echo "  - From application menu: Search for 'TRustBrowser WhatsApp', 'Calendar', or 'Messenger'"
echo "  - From command line:"
echo "      rbrowser whatsapp"
echo "      rbrowser messenger"
echo "      rbrowser 'google calendar'"
echo "      rbrowser-all          # Launch all instances"
echo ""
echo "Configuration file: ~/.config/trustbrowser/sites.conf"
