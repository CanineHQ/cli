#!/bin/sh
set -e

REPO="CanineHQ/cli"
BINARY_NAME="k9"
INSTALL_DIR="/usr/local/bin"

main() {
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)

    case "$OS" in
        darwin*)
            echo "macOS detected."
            echo ""
            echo "Install Canine CLI using Homebrew:"
            echo ""
            echo "  brew tap CanineHQ/canine"
            echo "  brew install canine"
            echo ""
            exit 0
            ;;
        linux*)
            install_linux
            ;;
        mingw*|msys*|cygwin*|windows*)
            echo "Error: Windows is not supported."
            echo "Please use WSL (Windows Subsystem for Linux) and run this script again."
            exit 1
            ;;
        *)
            echo "Error: Unsupported operating system: $OS"
            exit 1
            ;;
    esac
}

install_linux() {
    echo "Linux detected."

    # Detect architecture
    case "$ARCH" in
        x86_64|amd64)
            ARCH="amd64"
            ;;
        aarch64|arm64)
            ARCH="arm64"
            ;;
        *)
            echo "Error: Unsupported architecture: $ARCH"
            exit 1
            ;;
    esac

    echo "Architecture: $ARCH"

    # Get latest release version
    echo "Fetching latest release..."
    LATEST_VERSION=$(curl -sL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

    if [ -z "$LATEST_VERSION" ]; then
        echo "Error: Could not determine latest version."
        exit 1
    fi

    echo "Latest version: $LATEST_VERSION"

    # Construct download URL
    DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${LATEST_VERSION}/${BINARY_NAME}-linux-${ARCH}"

    echo "Downloading from: $DOWNLOAD_URL"

    # Create temp directory
    TMP_DIR=$(mktemp -d)
    trap "rm -rf $TMP_DIR" EXIT

    # Download binary
    curl -sL "$DOWNLOAD_URL" -o "$TMP_DIR/$BINARY_NAME"

    # Make executable
    chmod +x "$TMP_DIR/$BINARY_NAME"

    # Install to /usr/local/bin (may require sudo)
    echo "Installing to $INSTALL_DIR/$BINARY_NAME..."
    if [ -w "$INSTALL_DIR" ]; then
        mv "$TMP_DIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
    else
        sudo mv "$TMP_DIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
    fi

    echo ""
    echo "Canine CLI installed successfully!"
    echo "Run 'k9 --help' to get started."
}

main
