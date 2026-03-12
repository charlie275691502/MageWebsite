# Data Files Structure

This document explains how game data files are organized to avoid duplication.

## Single Source of Truth

The game data files (`cards.json` and `spells.json`) are stored **once** in the backend root directory:

```
mage_battle/
├── cards.json         # ← Source of truth
├── spells.json        # ← Source of truth
└── frontend/src/
    ├── cards.json     # → Symlink to ../../cards.json
    ├── spells.json    # → Symlink to ../../spells.json
    └── characters.json # Frontend-only file
```

## How It Works

- **Backend** (Rust): Reads directly from `cards.json` and `spells.json` in the root using `include_str!("../cards.json")`
- **Frontend** (React): Imports through symbolic links in `src/` directory
- **Version Control**: Git tracks the symlinks, so they work across different machines

## Updating Game Data

To update cards or spells:

1. Edit `cards.json` or `spells.json` in the **backend root directory**
2. Changes automatically affect both backend and frontend
3. No need to update files in two places!

## Build Process

- Backend: `cargo build` - Embeds JSON files at compile time
- Frontend: `npm run build` - Follows symlinks during bundling
- Both processes work seamlessly with this structure

## Character Data

`characters.json` is frontend-only as it's only used for UI display in the lobby/waiting room.
