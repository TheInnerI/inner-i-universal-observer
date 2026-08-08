#!/bin/bash
# ============================================================
# Inner I Universal Observer — All-in-one launcher
# ============================================================
# Starts the Observer Node + Control Center dashboard.
# One command to rule them all.
# ============================================================

set -e
ROOT="$(cd "$(dirname "$0")" && pwd)"

cleanup() {
    echo ""
    echo "Shutting down..."
    kill $NODE_PID 2>/dev/null
    kill $CC_PID 2>/dev/null
    exit 0
}
trap cleanup INT TERM

echo "============================================"
echo "  Inner I Universal Observer"
echo "  Starting all services..."
echo "============================================"
echo ""

# Check if Observer Node binary exists
if [ ! -f "$ROOT/target/debug/observer-node" ]; then
    echo "Building Observer Node..."
    cd "$ROOT" && cargo build -p observer-node 2>&1
fi

# Start Observer Node
echo "[1/2] Starting Observer Node on :7411..."
cd "$ROOT" && cargo run -p observer-node &
NODE_PID=$!
sleep 2

# Health check
if curl -s http://127.0.0.1:7411/health >/dev/null 2>&1; then
    echo "  ✓ Observer Node running"
else
    echo "  ⚠ Waiting for Observer Node..."
    sleep 3
fi

# Install deps if needed
if [ ! -d "$ROOT/apps/control-center/node_modules" ]; then
    echo "  Installing Control Center dependencies..."
    cd "$ROOT/apps/control-center" && npm install --silent 2>&1
fi

# Start Control Center
echo "[2/2] Starting Control Center on :7412..."
cd "$ROOT/apps/control-center" && npm run dev &
CC_PID=$!
sleep 3

echo ""
echo "============================================"
echo "  Inner I Universal Observer — Ready"
echo ""
echo "  Dashboard:   http://localhost:7412"
echo "  API:         http://127.0.0.1:7411"
echo "  Press Ctrl+C to stop all services"
echo "============================================"
echo ""

wait