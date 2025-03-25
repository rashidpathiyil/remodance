#!/bin/bash

# Platform-specific test environment setup script
# This script detects the platform and sets up the necessary environment for testing

# Exit on error
set -e

# Detect platform
PLATFORM="unknown"
case "$(uname -s)" in
    Darwin*)    PLATFORM="macos" ;;
    Linux*)     PLATFORM="linux" ;;
    CYGWIN*)    PLATFORM="windows" ;;
    MINGW*)     PLATFORM="windows" ;;
    MSYS*)      PLATFORM="windows" ;;
    *)          PLATFORM="unknown" ;;
esac

echo "Detected platform: $PLATFORM"

# Platform-specific setup
case "$PLATFORM" in
    macos)
        echo "Setting up macOS test environment..."
        
        # Check for required tools
        if ! command -v cargo &> /dev/null; then
            echo "Rust is not installed. Installing..."
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            source $HOME/.cargo/env
        fi
        
        if ! command -v node &> /dev/null; then
            echo "Node is not installed. Installing..."
            if command -v brew &> /dev/null; then
                brew install node
            else
                echo "Homebrew not found. Please install Node.js manually."
                exit 1
            fi
        fi
        
        # Check for Xcode Command Line Tools
        if ! xcode-select -p &> /dev/null; then
            echo "Xcode Command Line Tools not found. Installing..."
            xcode-select --install
            echo "Please complete the installation and run this script again."
            exit 1
        fi
        ;;
        
    linux)
        echo "Setting up Linux test environment..."
        DISTRO="unknown"
        
        # Detect distribution
        if [ -f /etc/os-release ]; then
            source /etc/os-release
            DISTRO=$ID
        fi
        
        echo "Detected Linux distribution: $DISTRO"
        
        # Install dependencies based on distribution
        case "$DISTRO" in
            ubuntu|lubuntu|debian)
                echo "Installing dependencies for Ubuntu/Debian..."
                sudo apt-get update
                sudo apt-get install -y curl build-essential libssl-dev pkg-config libgtk-3-dev \
                    libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf
                
                # Install Node.js if not present
                if ! command -v node &> /dev/null; then
                    curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
                    sudo apt-get install -y nodejs
                fi
                
                # Install Rust if not present
                if ! command -v cargo &> /dev/null; then
                    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                    source $HOME/.cargo/env
                fi
                ;;
                
            fedora)
                echo "Installing dependencies for Fedora..."
                sudo dnf install -y curl gcc gcc-c++ openssl-devel gtk3-devel \
                    webkit2gtk3-devel libappindicator-gtk3-devel librsvg2-devel patchelf
                
                # Install Node.js if not present
                if ! command -v node &> /dev/null; then
                    sudo dnf module install -y nodejs:18/default
                fi
                
                # Install Rust if not present
                if ! command -v cargo &> /dev/null; then
                    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                    source $HOME/.cargo/env
                fi
                ;;
                
            *)
                echo "Unsupported Linux distribution: $DISTRO"
                echo "Please install dependencies manually:"
                echo "- Rust (https://rustup.rs/)"
                echo "- Node.js"
                echo "- Build tools (gcc, g++, make)"
                echo "- WebKit2GTK and GTK3 development files"
                exit 1
                ;;
        esac
        ;;
        
    windows)
        echo "Setting up Windows test environment..."
        
        # Check for Rust
        if ! command -v cargo &> /dev/null; then
            echo "Rust is not installed. Please install from https://rustup.rs/"
            exit 1
        fi
        
        # Check for Node.js
        if ! command -v node &> /dev/null; then
            echo "Node.js is not installed. Please install from https://nodejs.org/"
            exit 1
        fi
        
        # Check for Visual Studio Build Tools
        if ! command -v cl &> /dev/null; then
            echo "Visual Studio Build Tools not found. Please install Visual Studio with C++ development tools."
            exit 1
        fi
        ;;
        
    *)
        echo "Unsupported platform: $PLATFORM"
        exit 1
        ;;
esac

echo "Environment setup complete."
echo "To build the application, run:"
echo "  npm install"
echo "  npm run tauri build"

echo "To run the tests, run:"
echo "  cd src-tauri && cargo test"
echo "  node scripts/test-idle-detection.js" 
