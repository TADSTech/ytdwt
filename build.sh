#!/bin/bash

echo "Building ytdwt in release mode..."
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✓ Build successful!"
    echo ""
    echo "Binary location: target/release/ytdwt"
    echo "Binary size: $(du -h target/release/ytdwt | cut -f1)"
    echo ""
    echo "To install system-wide, run:"
    echo "  sudo cp target/release/ytdwt /usr/local/bin/"
    echo ""
    echo "Or run directly:"
    echo "  ./target/release/ytdwt"
else
    echo "✗ Build failed!"
    exit 1
fi
