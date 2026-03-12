# Character Images Folder

Place all character images in organized subdirectories.

## Directory Structure

```
/public/characters/
  ├── avatar/          (Avatar images for grid display)
  └── full/            (Full images for detail panel)
```

## Required Images for Current Characters

### Avatar Images (100x100px, for grid display) - Place in `avatar/`:
- `flame_poison_avatar.png`
- `wood_wind_avatar.png`
- `thunder_poison_avatar.png`
- `water_wind_avatar.png`

### Full Images (300x400px, for detail panel) - Place in `full/`:
- `flame_poison_full.png`
- `wood_wind_full.png`
- `thunder_poison_full.png`
- `water_wind_full.png`

## Adding More Characters

For each new character, add:
1. `{character_id}_avatar.png` - Place in `/public/characters/avatar/`
2. `{character_id}_full.png` - Place in `/public/characters/full/`

Then update `/src/characters.json` with the new character data.

See `/CHARACTER_IMAGES_GUIDE.md` for detailed instructions.
