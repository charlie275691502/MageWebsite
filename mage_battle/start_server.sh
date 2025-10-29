#!/bin/bash

echo "========================================"
echo " MageBattle Backend Server"
echo "========================================"
echo ""
echo "Starting Rust backend server..."
echo "Server will be available at http://localhost:3000"
echo ""
echo "Press Ctrl+C to stop the server"
echo ""

cargo run --release -- --web
