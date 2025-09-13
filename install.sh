#!/bin/bash

set -e

APP_NAME="Floater"
CLI_NAME="floatercli"
INSTALL_DIR="/usr/local/bin"
APP_PATH="/Applications/$APP_NAME.app"
CLI_SOURCE="$APP_PATH/Contents/MacOS/$CLI_NAME"

echo "🚀 Installing $APP_NAME..."

# Check if app exists in Applications folder
if [ ! -d "$APP_PATH" ]; then
    echo "❌ Error: $APP_NAME.app not found in /Applications/"
    echo "Please drag $APP_NAME.app to your Applications folder first."
    exit 1
fi

# Check if CLI binary exists in the app bundle
if [ ! -f "$CLI_SOURCE" ]; then
    echo "❌ Error: CLI binary not found in app bundle"
    echo "Expected: $CLI_SOURCE"
    exit 1
fi

# Create symlink for CLI
echo "📦 Installing CLI to $INSTALL_DIR/$CLI_NAME..."
sudo ln -sf "$CLI_SOURCE" "$INSTALL_DIR/$CLI_NAME"

# Verify installation
if command -v $CLI_NAME >/dev/null 2>&1; then
    echo "✅ $APP_NAME installed successfully!"
    echo "🔧 CLI is available as '$CLI_NAME'"
    echo ""
    echo "Usage:"
    echo "  $CLI_NAME show \"Hello, World!\""
    echo "  $CLI_NAME show \"Timer test\" --timer"
    echo "  $CLI_NAME hide"
else
    echo "⚠️  CLI installation may have failed. Try adding $INSTALL_DIR to your PATH."
    exit 1
fi