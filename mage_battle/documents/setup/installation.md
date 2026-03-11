# Installation Guide

Complete guide to installing and setting up MageBattle.

## Prerequisites

### Required Software

- **Rust** (1.70+)
  - Download from: https://rustup.rs/
  - Verify: `rustc --version`

- **Node.js** (16+)
  - Download from: https://nodejs.org/
  - Verify: `node --version`

- **npm** (comes with Node.js)
  - Verify: `npm --version`

### System Requirements

- **OS**: Windows 10+, macOS 10.15+, or Linux
- **RAM**: 4GB minimum, 8GB recommended
- **Disk Space**: 2GB for dependencies

## Installation Steps

### 1. Clone or Extract Project

```bash
cd /path/to/projects
# If from git:
git clone <repository-url>
# Or extract the zip file
```

### 2. Install Frontend Dependencies

```bash
cd mage_battle/frontend
npm install
```

This takes 2-3 minutes to download React and dependencies.

### 3. Build Backend (First Time)

```bash
cd mage_battle
cargo build --release
```

First build takes 3-5 minutes to compile Rust dependencies.

## Running the Game

### Option 1: Using Scripts (Recommended)

#### Windows

Open 2 separate Command Prompts:

**Terminal 1:**
```cmd
cd mage_battle
start_server.bat
```

**Terminal 2:**
```cmd
cd mage_battle
start_frontend.bat
```

#### Mac/Linux

Open 2 separate terminals:

**Terminal 1:**
```bash
cd mage_battle
chmod +x start_server.sh
./start_server.sh
```

**Terminal 2:**
```bash
cd mage_battle
chmod +x start_frontend.sh
./start_frontend.sh
```

### Option 2: Manual Commands

**Terminal 1 - Backend:**
```bash
cd mage_battle
cargo run --release -- --web
```

**Terminal 2 - Frontend:**
```bash
cd mage_battle/frontend
npm start
```

## Verification

### Backend Check
You should see:
```
🚀 Server running on http://localhost:3000
```

Test by visiting: http://localhost:3000/api/game/new

### Frontend Check
Browser should auto-open at: http://localhost:3001

You should see the game lobby interface.

## Common Issues

### Issue: `cargo: command not found`

**Solution:**
- Rust not installed or not in PATH
- Install from https://rustup.rs/
- Restart terminal after installation

### Issue: `npm: command not found`

**Solution:**
- Node.js not installed or not in PATH
- Install from https://nodejs.org/
- Restart terminal after installation

### Issue: Port 3000 already in use

**Solution:**
```bash
# Find and kill the process using port 3000
# Windows:
netstat -ano | findstr :3000
taskkill /PID <PID> /F

# Mac/Linux:
lsof -i :3000
kill -9 <PID>
```

### Issue: Port 3001 already in use

**Solution:**
```bash
# Use a different port
cd mage_battle/frontend

# Windows:
set PORT=3002 && npm start

# Mac/Linux:
PORT=3002 npm start
```

### Issue: Build fails on Windows

**Solution:**
- Install Visual Studio Build Tools
- Download from: https://visualstudio.microsoft.com/downloads/
- Select "Desktop development with C++"

### Issue: `error: linker 'cc' not found`

**Solution (Linux):**
```bash
# Debian/Ubuntu:
sudo apt install build-essential

# Fedora/RHEL:
sudo dnf install gcc

# Arch:
sudo pacman -S base-devel
```

### Issue: Frontend shows "Cannot connect to backend"

**Check:**
1. Backend is running (check Terminal 1)
2. Backend shows no errors
3. Try accessing http://localhost:3000/api/game/new
4. Check browser console (F12) for error messages

### Issue: Slow compilation

**Solution:**
```bash
# Use fewer CPU cores if overheating
cargo build --release -j 2

# Or use debug build (faster to compile, slower to run)
cargo build
cargo run -- --web
```

## Development Setup

### For Code Development

**Install additional tools:**
```bash
# Rust formatter
rustup component add rustfmt

# Rust linter
rustup component add clippy
```

**Run tests:**
```bash
cd mage_battle
cargo test
```

**Run specific test:**
```bash
cargo test test_name
```

### For Frontend Development

**Install React DevTools:**
- Chrome: https://chrome.google.com/webstore
- Firefox: https://addons.mozilla.org/firefox

**Hot reload:**
Frontend automatically reloads on file changes when running `npm start`.

## Updating

### Update Rust Dependencies
```bash
cd mage_battle
cargo update
cargo build --release
```

### Update Frontend Dependencies
```bash
cd mage_battle/frontend
npm update
```

### Update Rust Toolchain
```bash
rustup update
```

## Uninstalling

### Remove Project
```bash
# Just delete the project folder
rm -rf /path/to/mage_battle  # Mac/Linux
# or manually delete on Windows
```

### Remove Rust (optional)
```bash
rustup self uninstall
```

### Remove Node.js (optional)
Follow your OS-specific uninstallation process.

## Next Steps

- Read the [Quick Start Guide](quick-start.md)
- Learn the [Game Rules](../gameplay/game-rules.md)
- Check [Project Status](../development/project-status.md)

---

Having trouble? Check [Troubleshooting](quick-start.md#troubleshooting) or open an issue on GitHub.
