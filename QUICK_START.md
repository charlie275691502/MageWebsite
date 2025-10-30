# MageBattle - Quick Start Guide

## 🚀 Start Playing in 3 Steps

### Step 1: Start the Backend Server

```bash
cd mage_battle
cargo run --release -- --web
```

Wait for:
```
🚀 啟動 MageBattle Web 服務器...
📍 服務器地址: http://localhost:3000
⛔ 按 Ctrl+C 停止服務器
```

**Important**: The backend MUST be running for the game to work!

### Step 2: Start the Frontend (Development Mode)

Open a new terminal:

```bash
cd mage_battle/frontend
npm start
```

Browser will automatically open to: http://localhost:3001

### Step 3: Create or Join a Game

**Player 1 (Host)**:
1. You'll see a random username already filled (e.g., "SwiftMage321")
2. Click "創建房間" (Create Room)
3. Share the 6-digit room code with other players
4. Select your character (火毒/木風/雷毒/水風)
5. Select your team (A, B, C, or D)
6. Click "準備" (Ready)
7. Wait for at least one more player
8. Click "開始遊戲" (Start Game)

**Player 2+ (Joiners)**:
1. You'll see a random username already filled
2. Enter the room code from Player 1
3. Click "加入房間" (Join Room)
4. Select your character
5. Select a DIFFERENT team than Player 1 (at least 2 teams required)
6. Click "準備" (Ready)
7. Wait for host to start

## 🎮 Gameplay Quick Reference

### Left Panel - Players List
- Shows all players with HP, attributes, and status
- **Your turn to allocate**: Buttons appear on your row
- **Current player**: Green glowing border
- **You**: Blue background

### Right Panel - Cards
- **Your hand**: Click cards to select
- **Hover 1+ second**: Shows detailed spell info popup
- **Actions**: Choose Top Spell, Bottom Spell, or Attribute Bolt
- **Auto-draw**: Card automatically drawn after playing

### Turn Phases

1. **AllocateAttribute** (分配屬性點)
   - Click attribute buttons on your player row (left panel)
   - Choose wisely!

2. **PlayCard** (出牌)
   - Click a card from your hand (right panel)
   - Choose action:
     - **Top Spell**: Use top spell on card
     - **Bottom Spell**: Use bottom spell on card
     - **Attribute Bolt**: Use card as attack
   - Card automatically drawn

3. **Next player's turn**

## 🎯 Game Rules Quick Summary

- **Teams**: At least 2 different teams (A/B/C/D)
- **Players**: 2-4 players
- **Win Condition**: Eliminate opposing team(s)
- **HP**: Starts at 25, dies at 0
- **Attributes**: 6 types (Fire, Wood, Thunder, Water, Wind, Poison)
- **Cards**: 60 unique spell cards from Excel data

## 🐛 Troubleshooting

### "Failed to fetch" or Connection Errors
→ Make sure backend is running on port 3000
→ Check: http://localhost:3000/api/game/test_id

### "Room not found"
→ Room codes expire when all players leave
→ Create a new room

### Page doesn't load
→ Make sure frontend is running: `npm start`
→ Clear browser cache (Ctrl+F5)

### Build errors
→ Backend: `cd mage_battle && cargo clean && cargo build --release`
→ Frontend: `cd mage_battle/frontend && rm -rf node_modules && npm install`

## 📝 Tips

1. **Testing solo**: Open multiple browser tabs/windows
2. **Quick reset**: Click "重置" button to clear all data
3. **Room codes**: 6 digits, easy to share
4. **Card info**: Hover over cards to see detailed spell effects
5. **Team strategy**: Coordinate with teammates!

## 🎨 UI Features

### New in This Version
- ✅ Random usernames (no typing needed!)
- ✅ Team A/B/C/D selection
- ✅ Clean table-based player list
- ✅ Left-right game layout
- ✅ Attribute buttons on player rows
- ✅ Auto-draw cards
- ✅ Card hover tooltips

### Color Coding
- **Team A**: Blue
- **Team B**: Red
- **Team C**: Green
- **Team D**: Gold
- **Current turn**: Green glow
- **You**: Blue highlight
- **Host**: Gold background + 👑

## 🎯 Game Flow Diagram

```
Lobby → Waiting Room → Game

Lobby:
  ├─ Create Room → Get Room Code
  └─ Join Room → Enter Code

Waiting Room:
  ├─ Select Character (4 options)
  ├─ Select Team (A/B/C/D)
  ├─ Click Ready
  └─ Host Starts → Game Begins

Game Turn:
  ├─ Phase 1: Allocate Attribute
  │   └─ Click button on your row
  ├─ Phase 2: Play Card
  │   ├─ Click card
  │   ├─ Choose action
  │   └─ Auto-draw new card
  └─ Repeat until win/loss
```

## 🚀 Production Deployment (Optional)

### Build Frontend for Production
```bash
cd mage_battle/frontend
npm run build
```

### Run Backend Only
```bash
cd mage_battle
cargo run --release -- --web
```

Backend can serve the built frontend from `frontend/build/` (if configured).

## 📚 More Info

- Implementation details: See `COMPLETED_FEATURES.md`
- Card data: 60 cards from `數值企劃.xlsx`
- API docs: Check backend code in `src/web_server.rs`

---

**Enjoy playing MageBattle! 🧙‍♂️✨**
