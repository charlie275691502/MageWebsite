# 🚀 MageBattle Quick Start Guide

## Prerequisites

- **Rust** (1.70+): https://rustup.rs/
- **Node.js** (16+): https://nodejs.org/
- **npm** (comes with Node.js)

## 🎮 Quick Start (5 minutes)

### Step 1: Install Frontend Dependencies

```bash
cd mage_battle/frontend
npm install
```

This will take 2-3 minutes to download all React dependencies.

### Step 2: Start the Backend Server

**Windows:**
```bash
cd mage_battle
start_server.bat
```

**Mac/Linux:**
```bash
cd mage_battle
chmod +x start_server.sh
./start_server.sh
```

**Or manually:**
```bash
cd mage_battle
cargo run --release -- --web
```

The server will compile (first time takes 3-5 minutes) and start on **http://localhost:3000**

### Step 3: Start the Frontend

Open a **NEW terminal/command prompt**:

**Windows:**
```bash
cd mage_battle
start_frontend.bat
```

**Mac/Linux:**
```bash
cd mage_battle
chmod +x start_frontend.sh
./start_frontend.sh
```

**Or manually:**
```bash
cd mage_battle/frontend
npm start
```

The frontend will start on **http://localhost:3001** and should automatically open in your browser.

## 🎯 How to Play (4 Players)

### Setup Phase

1. **First player** goes to http://localhost:3001
2. Fill in player names and select characters
3. Click "創建遊戲" (Create Game)

### Join Game (Other 3 Players)

4. **Players 2, 3, 4** each open a **NEW browser tab** (or use different browsers/devices)
5. Go to http://localhost:3001
6. The game will show "選擇你的角色" (Select Your Character)
7. Each player clicks on their character card

### Playing

Each player's tab will show:
- ✅ **Their turn**: Action buttons appear
- ⏳ **Others' turn**: "Waiting..." message

**Turn Structure (3 phases):**
1. **Allocate Attribute** - Click an attribute button (火/木/雷/水/風/毒)
2. **Play Card** - Use Attribute Bolt to attack enemies
3. **Draw Card** - Click "抽一張卡" to end turn

**Win Condition:**
- Defeat both players on the opposing team
- Team A (Players 1 & 3) vs Team B (Players 2 & 4)

## 🔧 Troubleshooting

### Backend won't start
```bash
# Make sure you're in the mage_battle directory
cd mage_battle

# Clean and rebuild
cargo clean
cargo build --release
```

### Frontend won't start
```bash
# Make sure you're in the frontend directory
cd mage_battle/frontend

# Delete node_modules and reinstall
rm -rf node_modules package-lock.json  # or delete manually on Windows
npm install
```

### Port already in use
```bash
# Backend (port 3000)
# Windows: netstat -ano | findstr :3000
# Mac/Linux: lsof -i :3000

# Frontend (port 3001)
# Change port by setting PORT environment variable
# Windows: set PORT=3002 && npm start
# Mac/Linux: PORT=3002 npm start
```

### Can't connect to backend
1. Make sure backend is running (check terminal for errors)
2. Make sure it says "Server running on http://localhost:3000"
3. Try accessing http://localhost:3000/api/game/new directly

## 📱 Multi-Device Play

You can also play with friends on the same network:

1. Find your computer's IP address:
   - Windows: `ipconfig` (look for IPv4 Address)
   - Mac/Linux: `ifconfig` or `ip addr`

2. Other players visit: `http://YOUR_IP:3001`

3. Make sure firewall allows connections on ports 3000 and 3001

## 🎨 Game Features

- ✅ 4 player support (2v2)
- ✅ 6 attribute types with mastery effects
- ✅ Character liberation skills
- ✅ Death and revival mechanics
- ✅ Shield and buff system
- ✅ Real-time game state updates

## 📝 Notes

- Game state auto-refreshes every 2 seconds
- Each player only sees their own hand cards
- Game ID is stored in localStorage (persists across page refreshes)
- Use "重置遊戲" (Reset Game) button to start a new game

## 🐛 Known Issues

- Card data not yet loaded (only attribute bolt attacks work)
- Some effects not fully implemented
- No AI opponent yet

Enjoy the game! 🎮⚔️🧙‍♂️
