# Character Images Guide

## Where to Place Character Images

All character images should be placed in the `/public/characters/` folder with separate subdirectories for avatars and full images.

### Directory Structure

```
/public/
  └── characters/
      ├── avatar/
      │   ├── flame_poison_avatar.png      (Avatar for FlamePoison)
      │   ├── wood_wind_avatar.png         (Avatar for WoodWind)
      │   ├── thunder_poison_avatar.png    (Avatar for ThunderPoison)
      │   └── water_wind_avatar.png        (Avatar for WaterWind)
      └── full/
          ├── flame_poison_full.png        (Full image for FlamePoison)
          ├── wood_wind_full.png           (Full image for WoodWind)
          ├── thunder_poison_full.png      (Full image for ThunderPoison)
          └── water_wind_full.png          (Full image for WaterWind)
```

### Image Requirements

#### Avatar Images (Grid Display)
- **Size:** 100x100 pixels (or square aspect ratio)
- **Format:** PNG or JPG
- **Purpose:** Displayed in the 10x3 character selection grid
- **Style:** Portrait/icon style, clear character face/design

#### Full Images (Detail Section)
- **Size:** 300x400 pixels (3:4 aspect ratio recommended)
- **Format:** PNG or JPG
- **Purpose:** Displayed in the character detail panel on the left side
- **Style:** Full character art showing more details

### Adding New Characters

To add new characters to the game:

1. **Add character images to the appropriate folders:**
   - `{character_id}_avatar.png` - Place in `/public/characters/avatar/`
   - `{character_id}_full.png` - Place in `/public/characters/full/`

2. **Update `/src/characters.json`:**

**Important:** The `avatar` and `fullImage` paths must include the subdirectory (`avatar/` or `full/`).

```json
{
  "characters": [
    {
      "id": "YourCharacterId",
      "name": "Character_Internal_Name",
      "title": "角色顯示名稱",
      "avatar": "avatar/your_character_avatar.png",
      "fullImage": "full/your_character_full.png",
      "primaryAttributes": ["Fire", "Wood"],
      "description": "角色描述文字",
      "skills": [
        {
          "name": "技能名稱",
          "type": "liberation",
          "description": "技能描述"
        }
      ],
      "proficiencies": {
        "Fire": "火焰專精描述",
        "Wood": "木屬性專精描述"
      }
    }
  ]
}
```

3. **Update Rust backend** (if character requires backend changes):
   - Add character enum variant in `src/character.rs`
   - Update character implementation logic

### Current Characters

The system currently has these 4 characters configured:

1. **FlamePoison** (火毒法師)
   - Avatar: `/public/characters/avatar/flame_poison_avatar.png`
   - Full: `/public/characters/full/flame_poison_full.png`

2. **WoodWind** (木風法師)
   - Avatar: `/public/characters/avatar/wood_wind_avatar.png`
   - Full: `/public/characters/full/wood_wind_full.png`

3. **ThunderPoison** (雷毒法師)
   - Avatar: `/public/characters/avatar/thunder_poison_avatar.png`
   - Full: `/public/characters/full/thunder_poison_full.png`

4. **WaterWind** (水風法師)
   - Avatar: `/public/characters/avatar/water_wind_avatar.png`
   - Full: `/public/characters/full/water_wind_full.png`

### Fallback Behavior

If an image is missing, the system will automatically generate a placeholder SVG with the character's title. However, for best user experience, all images should be provided.

### Grid Layout

The character grid displays:
- **10 columns** with rows adjusted based on the number of characters available
- Current playable characters are displayed first
- The grid size automatically adjusts to fit all characters without showing placeholder slots

### Tips

- Keep file sizes reasonable (avatars < 50KB, full images < 200KB)
- Use consistent art style across all characters
- Ensure images are clear and recognizable at small sizes
- Test images in both light and dark backgrounds
- Consider adding transparent backgrounds (PNG) for better blending
