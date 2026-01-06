#!/bin/bash
set -e

# Build first
./build_web.sh

# Serve locally
echo "🌐 Starting local server at http://localhost:8081"
echo "Press Ctrl+C to stop"

if command -v python3 &> /dev/null; then
    python3 -m http.server -d dist 8080
elif command -v python &> /dev/null; then
    cd dist && python -m SimpleHTTPServer 8080
else
    echo "❌ Python not found. Please install Python or use another static file server."
    echo "   Alternatively, use: cargo install basic-http-server && basic-http-server dist"
    exit 1
fi
