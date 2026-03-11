# MageBattle 法師對戰 🧙‍♂️⚔️

A 4-player turn-based card battle game built with Rust and React.

## 🚀 Quick Start

See **[Quick Start Guide](documents/setup/quick-start.md)** to get started in 5 minutes!

## 📚 Documentation

All documentation has been organized in the `documents/` folder:

### 🎯 Getting Started
- **[Quick Start Guide](documents/setup/quick-start.md)** - Get up and running in 5 minutes
- **[Installation Guide](documents/setup/installation.md)** - Detailed setup instructions

### 🎮 Gameplay
- **[Game Rules](documents/gameplay/game-rules.md)** - Complete game rules and mechanics
- **[Combat System](documents/gameplay/combat-system.md)** - Damage calculation and targeting
- **[Characters](documents/gameplay/characters.md)** - Character types and liberation skills
- **[Buffs & Debuffs](documents/gameplay/buffs-debuffs.md)** - Status effects guide

### 💻 Technical
- **[Architecture](documents/technical/architecture.md)** - System design and technology stack
- **[API Reference](documents/technical/api-reference.md)** - REST API documentation
- **[Proficiency System](documents/technical/proficiency-system.md)** - Implementation details
- **[Testing Guide](documents/technical/testing.md)** - How to run and write tests

### 🛠️ Development
- **[Project Status](documents/development/project-status.md)** - Current features and roadmap
- **[Contributing](documents/development/contributing.md)** - How to contribute

**Full Documentation**: See [documents/README.md](documents/README.md)

## ✨ Features

- 🎮 4-player 2v2 team battles
- ⚔️ 6 attribute types with unique proficiency effects
- 📋 60 spell cards with dual-sided effects
- 🌟 Character liberation skills
- 💀 Death and revival mechanics
- 🌐 Web-based multiplayer

## 🏗️ Technology Stack

- **Backend**: Rust + Axum
- **Frontend**: React + TypeScript
- **Architecture**: REST API with polling-based updates

## 📦 Project Structure

```
mage_battle/
├── documents/          # 📚 All documentation
│   ├── setup/         # Installation guides
│   ├── gameplay/      # Game rules
│   ├── technical/     # Technical docs
│   └── development/   # Contributing
├── src/               # Rust backend source
├── tests/             # Rust tests
├── frontend/          # React frontend
├── spells.json        # Spell card data
└── cards.json         # Card metadata
```

## 🎮 Quick Commands

```bash
# Start backend server
./start_server.sh       # Mac/Linux
start_server.bat        # Windows

# Start frontend (in new terminal)
./start_frontend.sh     # Mac/Linux
start_frontend.bat      # Windows

# Run tests
cargo test

# Build release
cargo build --release
```

## 📝 License

This project is an educational/learning project.

---

**Need help?** Check the [documentation](documents/README.md) or [troubleshooting guide](documents/setup/quick-start.md#troubleshooting).

Built with ❤️ using Rust & React
