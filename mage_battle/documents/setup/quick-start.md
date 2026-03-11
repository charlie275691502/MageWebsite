# Quick Start Guide

Get MageBattle running in 5 minutes!

## 🚀 Quick Start (2 Terminals Required)

### Terminal 1 - Backend Server

```bash
cd mage_battle
start_server.bat          # Windows
# OR
./start_server.sh         # Mac/Linux
```

Wait for: "🚀 Server running on http://localhost:3000"

### Terminal 2 - Frontend

```bash
cd mage_battle
start_frontend.bat        # Windows
# OR
./start_frontend.sh       # Mac/Linux
```

Browser will open at: http://localhost:3001

## 🎮 How to Play

### Setup Phase (4 Players)

1. **Player 1**:
   - Open http://localhost:3001
   - Enter player names
   - Select characters for each player
   - Click "創建遊戲" (Create Game)
   - Click on "玩家1" to control that character

2. **Players 2, 3, 4**:
   - Each opens a **NEW browser tab** (Ctrl+T / Cmd+T)
   - Go to http://localhost:3001
   - Click on their character (玩家2, 玩家3, or 玩家4)

### Playing Your Turn

When it's your turn, you'll see action buttons:

#### Phase 1: Allocate Attribute
Click one of the six attribute buttons:
- 🔥 火 (Fire)
- 🌳 木 (Wood)
- ⚡ 雷 (Thunder)
- 💧 水 (Water)
- 🌪️ 風 (Wind)
- ☠️ 毒 (Poison)

#### Phase 2: Play Card
- Select an enemy target
- Choose an attribute bolt to attack
- OR use Liberation Skill (if available)

#### Phase 3: Draw Card
- Click "抽一張卡" to end your turn

### Win Condition
- **Team A** (Players 1 & 3) vs **Team B** (Players 2 & 4)
- Defeat both players on the opposing team to win!

## 🔧 Troubleshooting

### Backend won't start
```bash
cd mage_battle
cargo clean
cargo build --release
```

### Frontend won't start
```bash
cd mage_battle/frontend
rm -rf node_modules package-lock.json
npm install
```

### Port already in use

**Backend (port 3000):**
```bash
# Windows
netstat -ano | findstr :3000

# Mac/Linux
lsof -i :3000
```

**Frontend (port 3001):**
```bash
# Change port
# Windows: set PORT=3002 && npm start
# Mac/Linux: PORT=3002 npm start
```

### Can't connect to backend
1. Check backend terminal for errors
2. Verify it says "Server running on http://localhost:3000"
3. Try accessing http://localhost:3000/api/game/new directly

## 📱 Multi-Device Play

Play with friends on the same network:

1. **Find your IP address:**
   - Windows: `ipconfig` (look for IPv4 Address)
   - Mac/Linux: `ifconfig` or `ip addr`

2. **Other players visit:**
   ```
   http://YOUR_IP:3001
   ```

3. **Firewall:**
   - Allow connections on ports 3000 and 3001

## 📝 Notes

- Game state auto-refreshes every 2 seconds
- Each player only sees their own hand cards
- Game ID is stored in localStorage (persists across refreshes)
- Use "重置遊戲" (Reset Game) button to start a new game

## 🎯 Next Steps

- Read the [Complete Game Rules](../gameplay/game-rules.md)
- Learn about [Character Abilities](../gameplay/characters.md)
- Understand the [Combat System](../gameplay/combat-system.md)

---

Need more help? Check the [Installation Guide](installation.md) for detailed setup instructions.
