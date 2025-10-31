# CRITICAL CHANGES NEEDED - Battle_Logic.txt Updated

## 🚨 MAJOR BREAKING CHANGES DETECTED

The updated Battle_Logic.txt contains **MAJOR** changes that require significant code updates.

---

## 1. ⚠️ SHIELD MECHANICS - COMPLETE REVERSAL

### OLD RULE (Lines 181):
```
Example: 10 damage, 7 shield → 0 shield, -3 HP
```
Shield absorbs first, **remaining damage goes to HP**

### NEW RULE (Lines 180-186):
```
- Decrease Shield equals to the damage
- The surplus damage from the shield DOES NOT deal to HP
- Example: 50HP, 7 shield, took 5 damage =>50HP, 2 shield
- Example: 50HP, 7 shield, took 10 damage =>50HP, 0 shield
```

**NEW BEHAVIOR**: Shield blocks ALL damage. If damage > shield, shield goes to 0, but **HP is NOT affected**!

### CURRENT CODE (player.rs:144-152):
```rust
let actual_damage = if damage > self.shield {
    let remaining = damage - self.shield;
    self.shield = 0;
    self.hp -= remaining as i32;  // ❌ THIS IS WRONG NOW!
    remaining
} else {
    self.shield -= damage;
    0
};
```

### REQUIRED FIX:
```rust
// Shield absorbs ALL damage, no overflow to HP
if damage >= self.shield {
    self.shield = 0;
} else {
    self.shield -= damage;
}
// HP is never affected by normal damage anymore (only by direct damage)
```

---

## 2. ⚠️ ATTRIBUTE BOLT TARGETING - CHANGED

### OLD RULE (Line 432-436):
```
ALWAYS targets left neighbor (previous in turn order)
P0's left neighbor = P3
```

### NEW RULE (Lines 200-201, 438-443):
```
DEFAULT TARGETING (for spells if not specified):
  - MUST target furthest alived enemy
```

Attribute bolts now target **furthest alive enemy**, NOT left neighbor!

### CURRENT CODE (game.rs):
Uses `left_player_id()` for attribute bolts

### REQUIRED FIX:
Change attribute bolt targeting to use **furthest alive enemy** logic

---

## 3. ⚠️ ATTRIBUTE BOLTS ARE NOW SPELLS

### NEW RULE (Line 150):
```
ATTRIBUTE BOLT:
  - Count as spell. It means it deal spell damage
```

**IMPLICATIONS:**
- Attribute bolts now deal "spell damage"
- Wood Lv5 reduction should apply to attribute bolts (-1 damage)
- Guard Wood Carving should reduce attribute bolt damage (-4 damage)

### CURRENT CODE:
Attribute bolts use `from_spell=true` or `false`?

### REQUIRED FIX:
- Ensure attribute bolts pass `from_spell=true` to `take_damage()`
- Wood Lv5 should reduce attribute bolt damage

---

## 4. ⚠️ FIRE LV3 - ONLY ATTRIBUTE BOLTS

### OLD RULE:
```
Lv3: All attribute bolts and damage spells +1 damage
```

### NEW RULE (Line 170):
```
Fire Lv3: +1 to if the spell is attribute bolt
```

Fire Lv3 now **ONLY** applies to attribute bolts, NOT all damage!

### CURRENT CODE (game.rs):
Applies Fire Lv3 to all damage

### REQUIRED FIX:
Only apply Fire Lv3 bonus when casting attribute bolts

---

## 5. ⚠️ WOOD LV3 - ONLY "WOOD SPELL"

### OLD RULE:
```
When using Wood attribute bolt or Wood spell card
```

### NEW RULE (Line 72):
```
When using Wood spell
```

**UNCLEAR**: Does "Wood spell" include attribute bolts? Given that bolts are now spells (line 150), probably yes, but needs clarification.

### CURRENT CODE:
Checks for Wood-enchanted spells OR Wood attribute bolts

### REQUIRED ACTION:
- If "Wood spell" includes attribute bolts: keep current logic
- If NOT: only trigger on spell cards with Wood requirement

---

## 6. ⚠️ WOOD LV5 - ONLY SPELL DAMAGE

### OLD RULE:
```
Self and teammate receive -1 damage from all card/spell damage
```

### NEW RULE (Line 75):
```
Self and teammate receive -1 spell damage from all spell damages
```

**CLARIFICATION**: Now explicitly says "spell damage from all spell damages" (redundant but clear)

### CURRENT CODE (player.rs:125):
```rust
if from_spell && self.attributes.wood >= 5 && damage > 0 {
    damage = damage.saturating_sub(1);
}
```

### REQUIRED FIX:
Probably correct already, but verify attribute bolts are considered "spell damage"

---

## 7. ⚠️ THUNDER PROFICIENCY - ONLY SPELLS

### OLD RULE:
```
Thunder attribute spells/bolts
```

### NEW RULE (Lines 78-79):
```
Lv3: Thunder spells +1 damage
Lv5: Thunder spells +2 additional damage
```

Removed mention of "/bolts" - but since bolts are now spells (line 150), this should still apply to bolts.

### CURRENT CODE:
Applies to thunder-enchanted spells and bolts

### REQUIRED ACTION:
Verify logic still correct with bolts being "spells"

---

## 8. ⚠️ BURNING OUT - ONLY FIRE SPELLS

### OLD RULE:
```
Fire damage +5
```

### NEW RULE (Line 173, 238):
```
Burning Out buff: +5 to fire spells
Fire damage +5  (in character section)
```

Changed from "fire damage" to "fire spells"

### REQUIRED FIX:
Only apply Burning Out bonus to Fire-enchanted spells, not all fire damage

---

## 9. ⚠️ GUARD WOOD CARVING - SPELL AND SKILL DAMAGE

### OLD RULE:
```
Take -4 damage from all sources
```

### NEW RULE (Line 176):
```
Guard Wood Carving buff: -4 from spell and skill damage
```

Now only reduces **spell and skill damage**, NOT direct damage!

### CURRENT CODE (player.rs:130):
```rust
if self.buffs.has(crate::buff::BuffType::GuardWoodCarving) {
    damage = damage.saturating_sub(4);
```

### REQUIRED FIX:
Add damage type parameter, only apply reduction to spell/skill damage

---

## 10. ✅ DAMAGE TYPES - NEW DEFINITIONS

### NEW SECTION (Lines 162-166):
```
DAMAGE TYPES:
  - Spell damage: The damage caused by spells
  - Skill damage: The damage caused by skills
  - Direct damage: The damage specified on description
```

**ACTION REQUIRED:**
Create a damage type enum:
```rust
pub enum DamageType {
    Spell,
    Skill,
    Direct,
}
```

Update `take_damage()` signature:
```rust
pub fn take_damage(&mut self, damage: u32, damage_type: DamageType) -> u32
```

---

## 📋 IMPLEMENTATION PRIORITY

### CRITICAL (Must fix immediately):
1. **Shield mechanics** - Complete reversal, fundamentally breaks damage system
2. **Attribute bolt targeting** - Changes core gameplay (left neighbor → furthest enemy)
3. **Damage type system** - Foundation for all other changes

### HIGH PRIORITY:
4. **Fire Lv3** - Only attribute bolts (not all damage)
5. **Attribute bolts as spells** - Affects Wood/Thunder proficiency
6. **Guard Wood Carving** - Only spell/skill damage

### MEDIUM PRIORITY:
7. **Burning Out** - Fire spells only
8. **Wood Lv3** - Clarify "Wood spell" includes bolts or not
9. **Update all tests** - Many tests will fail with new rules

---

## 🧪 TESTING STRATEGY

1. Update damage type system first
2. Fix shield mechanics
3. Fix attribute bolt targeting
4. Run existing tests (expect many failures)
5. Update tests one by one to match new rules
6. Add new tests for edge cases

---

## ⏱️ ESTIMATED EFFORT

- **Damage type refactor**: 1-2 hours
- **Shield mechanics fix**: 30 minutes
- **Attribute bolt targeting**: 1 hour
- **Proficiency updates**: 1-2 hours
- **Test updates**: 2-3 hours

**TOTAL**: ~6-9 hours of work

This is a MAJOR rewrite of core game mechanics!
