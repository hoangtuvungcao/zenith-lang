#!/bin/bash
# Serve Zenith Web Demo Locally

set -e

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Zenith Web Server"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if dist exists
if [ ! -d "web/dist" ]; then
    echo "❌ web/dist not found. Run ./scripts/build-web.sh first"
    exit 1
fi

PORT=8080

echo "Starting local server on port $PORT..."
echo ""

# Try trunk serve (with hot reload)
if command -v trunk &> /dev/null; then
    echo "Using trunk serve (with hot reload)..."
    echo ""
    echo "  📡 Server: http://localhost:$PORT"
    echo "  🔄 Hot reload enabled"
    echo "  ⏹  Press Ctrl+C to stop"
    echo ""
    cd web
    trunk serve --port $PORT --open
else
    # Fallback to Python HTTP server
    echo "Using Python HTTP server (no hot reload)..."
    echo ""
    echo "  📡 Server: http://localhost:$PORT"
    echo "  ⏹  Press Ctrl+C to stop"
    echo ""
    
    cd web/dist
    
    if command -v python3 &> /dev/null; then
        python3 -m http.server $PORT
    elif command -v python &> /dev/null; then
        python -m http.server $PORT
    else
        echo "❌ No web server available"
        echo "Install trunk: cargo install trunk"
        echo "Or use Python: apt install python3"
        exit 1
    fi
fi
