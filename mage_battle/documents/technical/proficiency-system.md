# Proficiency System

Technical implementation details of the attribute proficiency system in MageBattle.

## Overview

The proficiency system is the core mechanism that grants passive bonuses based on attribute levels. It uses an "enchantment" pattern to selectively apply proficiencies to spells and attribute bolts.

## Architecture

### Key Components

1. **Proficiency Definitions** (`attribute.rs`)
   - Enum `MasteryEffect` defines all proficiency types
   - Method `get_mastery_effects()` checks attribute levels

2. **Enchantment System** (`game.rs`)
   - Vector of `AttributeType` passed through effect chain
   - Enables selective proficiency application

3. **Effect Handlers** (`game.rs`)
   - Different handlers for damage, healing, shield, etc.
   - Each matches enchantments to applicable proficiencies

## Code Locations

```
src/attribute.rs        Lines 154-181: Proficiency definitions
src/attribute.rs        Lines 183-237: Proficiency checking logic
src/game.rs             Lines 388-425: Spell execution
src/game.rs             Lines 429-745: Effect routing
src/game.rs             Lines 748-798: Damage proficiencies
src/game.rs             Lines 800-840: Heal proficiencies
src/game.rs             Lines 209-261: Attribute bolt implementation
src/player.rs           Lines 118-176: Damage reduction (Wood Lv5)
src/game.rs             Lines 112-147: Turn start passives (Fire Lv5)
```

## How Spells Trigger Proficiencies

### Flow Diagram

```
play_spell_card()
    ↓
Extract spell.requirements → enchantments = [Fire, Thunder, ...]
    ↓
execute_spell_effect(enchantments)
    ↓
apply_effect(effect, enchantments) for each effect
    ↓
Route to specific handler based on effect type
    ↓
Handler matches enchantments to proficiencies
    ↓
Apply matching bonuses
```

### Example: Thunder Spell with Lv5

```rust
// 1. Spell has requirements: [(Thunder, 2), (Fire, 1)]
let enchantments = vec![AttributeType::Thunder, AttributeType::Fire];

// 2. Execute spell effect with enchantments
execute_spell_effect(spell, targets, enchantments);

// 3. For damage effect, apply_damage_effect() is called
apply_damage_effect(target, base_damage: 10, enchantments);

// 4. Check proficiencies
if enchantments.contains(&AttributeType::Thunder) {
    if caster.thunder >= 3 { damage += 1; }
    if caster.thunder >= 5 { damage += 2; }
}
if enchantments.contains(&AttributeType::Fire) {
    if caster.fire >= 3 { damage += 1; }
}

// 5. Final damage: 10 + 3 (Thunder) + 1 (Fire) = 14
```

## Attribute Bolts Implementation

### Special Case

Attribute bolts cleverly reuse the spell proficiency system:

```rust
// play_attribute_bolt()
pub fn play_attribute_bolt(&mut self, card_id: CardId, attr_type: AttributeType, targets: Vec<PlayerId>) {
    // 1. Manually create enchantment
    let enchantments = vec![attr_type];

    // 2. Use IncreaseDamage(0) effect
    let effect = EffectType::IncreaseDamage(0);

    // 3. Apply effect (reuses damage proficiency system!)
    self.apply_effect(effect, caster_id, targets, enchantments);
}
```

### Why IncreaseDamage(0)?

- `IncreaseDamage(0)` means: damage = attribute_level + 0 + proficiency_bonuses
- Reuses existing damage proficiency logic
- Fire Lv3 adds +1
- Thunder Lv3/5 adds +1/+3
- Same proficiency handling as spell cards!

### Example: Fire Bolt

```rust
// Player has Fire Lv3, Thunder Lv5
// Uses Fire bolt (card_id=5, Fire level=3)

// 1. Enchantment created
enchantments = [Fire]

// 2. IncreaseDamage(0) effect
base_damage = 3 (Fire level) + 0 = 3

// 3. Apply proficiencies
// Fire Lv3: +1 (applies to ALL bolts)
// Thunder: +0 (not in enchantments)
final_damage = 3 + 1 = 4
```

## Proficiency Categories

### Active Proficiencies (Spell Casting)

Triggered during spell/bolt execution:

#### Fire Lv3
**Location**: `game.rs` in `apply_damage_effect()`
```rust
if enchantments.contains(&AttributeType::Fire) {
    if caster.attributes.fire >= 3 {
        damage += 1;
    }
}
```

**Applies To**:
- All attribute bolts (Fire in enchantments for Fire bolts)
- All Fire-enchanted spells

#### Thunder Lv3/5
**Location**: `game.rs` in `apply_damage_effect()`
```rust
if enchantments.contains(&AttributeType::Thunder) {
    if caster.attributes.thunder >= 3 {
        damage += 1;
    }
    if caster.attributes.thunder >= 5 {
        damage += 2;  // Total +3 with Lv3
    }
}
```

**Applies To**:
- Thunder-enchanted spells
- Thunder attribute bolts

#### Water Lv3/5
**Location**: `game.rs` in `apply_heal_effect()`
```rust
if enchantments.contains(&AttributeType::Water) {
    if caster.attributes.water >= 3 && target_id == caster_id {
        heal_amount += 1;
    }
}
// After loop, if Lv5, heal teammate too
```

**Applies To**:
- Water-enchanted healing spells
- NOT attribute bolts (no healing in bolts)

#### Wood Lv3 (Special)
**Location**: `game.rs` in `execute_spell_effect()` (BEFORE other effects)
```rust
if spell_enchantments.contains(&AttributeType::Wood) {
    if caster.attributes.wood >= 3 {
        caster.hp -= 1;  // DIRECT damage, bypasses shield
        caster.shield += 1;
    }
}
```

**Applies To**:
- ANY Wood-enchanted spell
- Applied BEFORE spell effect resolves

**Unique Properties**:
- Only proficiency with a cost
- Bypasses all protections (DIRECT damage)
- Can kill caster before spell resolves

### Passive Proficiencies (Turn Start / Damage Taken)

Not tied to spell casting:

#### Fire Lv5
**Location**: `game.rs` in `handle_turn_start()`
```rust
if self.players[current].attributes.fire >= 5 {
    for enemy in self.get_enemies(current) {
        enemy.take_damage(1, DamageType::Skill);
    }
}
```

**Timing**: Turn Start phase, before actions
**Type**: Skill damage (bypasses Wood Lv5 reduction)

#### Wood Lv5
**Location**: `player.rs` in `take_damage()`
```rust
if damage_type == DamageType::Spell && self.attributes.wood >= 5 {
    damage = damage.saturating_sub(1);
}
```

**Timing**: When taking spell damage
**Applies To**: Spell damage only (not Skill or Direct)

#### Wind Lv2/5
**Location**: `game.rs` in effect handlers
```rust
// Wind Lv2: Apply Defense Invalidation debuff
if wind >= 2 {
    target.buffs.add(DefenseInvalidation);
}

// Wind Lv5: Allow free targeting
if wind >= 5 {
    // Override target restrictions
}
```

**Timing**: When casting Wind-enchanted spells

#### Poison Lv2/5
**Location**: `game.rs` in effect handlers
```rust
// Poison Lv2: Apply Confuse debuff
if poison >= 2 {
    target.buffs.add(Confuse);
}

// Poison Lv5: Apply Silent debuff
if poison >= 5 {
    target.buffs.add(Silent);
}
```

**Timing**: When casting Poison-enchanted spells

## Spell vs Attribute Bolt Differences

### Identical Behavior

Both apply these proficiencies:
- ✅ Fire Lv3 (+1 damage)
- ✅ Thunder Lv3/5 (+1/+3 damage)
- ✅ All passive proficiencies (Fire Lv5, Wood Lv5, Wind, Poison)

### Different Behavior

| Proficiency | Spell | Bolt | Reason |
|------------|-------|------|--------|
| Wood Lv3 | ✅ Triggers | ❌ No | Spells check enchantments; bolts use IncreaseDamage only |
| Water Lv3/5 | ✅ Triggers | ❌ No | Healing effects only; no healing in IncreaseDamage |

**Why the Differences?**
- Attribute bolts use `IncreaseDamage(0)` effect
- `IncreaseDamage` doesn't trigger healing or shield proficiencies
- Only damage proficiencies apply

## Damage Type Classification

Three damage types affect proficiency application:

### Spell Damage
**Used By**:
- Spell cards
- Attribute bolts

**Affected By**:
- ✅ Wood Lv5 (-1 reduction)
- ✅ Guard Wood Carving (-4 reduction)
- ✅ Shields
- ✅ Immune/Invincible

### Skill Damage
**Used By**:
- Fire Lv5 passive
- Liberation skills

**Affected By**:
- ❌ Wood Lv5 (NOT reduced)
- ✅ Guard Wood Carving (-4 reduction)
- ✅ Shields
- ✅ Immune/Invincible

**Key**: Bypasses Wood Lv5 spell damage reduction!

### Direct Damage
**Used By**:
- Wood Lv3 HP cost
- Special card effects

**Affected By**:
- ❌ Nothing (cannot be blocked or reduced)
- ❌ Bypasses shields
- ❌ Bypasses damage reductions
- ❌ Bypasses immunities

## Design Pattern: Enchantments

### Pattern Description

**Enchantments** are the central mechanism for selective proficiency triggering.

**Definition**:
- Enchantments = attribute types extracted from spell requirements
- Example: Spell with "F2T1" → enchantments = [Fire, Thunder]

**Flow**:
1. Extract enchantments from spell requirements
2. Pass enchantments through effect chain
3. Effect handlers receive enchantments
4. Handlers match enchantments to proficiencies
5. Apply matching proficiency bonuses

**Benefits**:
- ✅ Multi-attribute spells work naturally
- ✅ Attribute bolts use same system
- ✅ Easy to add new proficiencies
- ✅ No spell-specific hardcoding

### Example: Multi-Attribute Spell

```rust
// Spell: "Fire Lv2, Thunder Lv2" (F2T2)
// Player: Fire Lv3, Thunder Lv5, Burning Out buff

// 1. Extract enchantments
enchantments = [Fire, Thunder]

// 2. Base damage: 10

// 3. Apply Fire proficiencies
if Fire in enchantments && fire >= 3 { damage += 1; }
if has Burning Out buff { damage += 5; }

// 4. Apply Thunder proficiencies
if Thunder in enchantments && thunder >= 3 { damage += 1; }
if Thunder in enchantments && thunder >= 5 { damage += 2; }

// 5. Final: 10 + 1 (Fire Lv3) + 5 (Burning Out) + 3 (Thunder Lv5) = 19
```

## Special Cases

### Wood Lv3 Exception

**Why Special?**
- Only proficiency applied in `execute_spell_effect()` (before other effects)
- HP cost must apply BEFORE healing effects
- Can kill caster, preventing spell from resolving

**Code Location**:
```rust
// game.rs lines 402-415
if spell_enchantments.contains(&AttributeType::Wood) {
    if caster.wood >= 3 {
        caster.hp -= 1;  // Bypass everything
        caster.shield += 1;

        if caster.hp <= 0 {
            caster.die();
            return; // Spell doesn't resolve!
        }
    }
}
```

### Burning Out Fire Damage

**Unique Properties**:
- Permanent buff (until Fire = 0)
- Consumes 1 Fire per turn
- +5 damage (largest damage bonus)
- Applies to Fire spells AND Fire bolts
- Stacks with Fire Lv3

**Sustainability**:
```
Turn 1: Fire 4, Burning Out active
Turn 2: Fire 4 → 3 (consumed), still active
Turn 3: Fire 3 → 2 (consumed), still active
Turn 4: Fire 2 → 1 (consumed), still active
Turn 5: Fire 1 → 0 (consumed), Burning Out REMOVED
```

Can maintain by allocating Fire each turn.

## Testing

### Test Coverage

**File**: `tests/proficiency.rs`

**Tests**:
- Fire Lv3 bolt damage boost ✅
- Fire Lv5 turn start damage ✅
- Thunder Lv3/5 damage bonuses ✅
- Wood Lv3 HP cost and shield ✅
- Wood Lv5 damage reduction ✅
- Water Lv3/5 healing bonuses ✅
- Wind Lv2/5 debuffs and targeting ✅
- Poison Lv2/5 debuffs ✅
- Burning Out buff ✅
- Multi-attribute proficiency stacking ✅

**Coverage**: Comprehensive for all proficiency types

### Running Tests

```bash
# All proficiency tests
cargo test proficiency

# Specific test
cargo test test_fire_lv3_bolt_bonus

# With output
cargo test proficiency -- --nocapture
```

## Common Questions

### Q: Why doesn't Wood Lv3 affect attribute bolts?

**A**: Attribute bolts use `IncreaseDamage(0)` effect, which doesn't check for Wood enchantments. The shield gain is part of `execute_spell_effect()`, which isn't called for bolts.

### Q: How do multi-attribute spells work?

**A**: All enchantments are checked. A Fire+Thunder spell gets Fire AND Thunder proficiency bonuses if the caster has them.

### Q: Can proficiencies stack infinitely?

**A**: Yes, but practically limited:
- Fire Lv3 (+1) + Burning Out (+5) + Thunder Lv5 (+3) = +9 max for Fire+Thunder spell
- Multiple proficiencies from same attribute don't stack (Lv3 and Lv5 are additive, not multiplicative)

### Q: Why is Wood Lv3 HP cost not affected by immunities?

**A**: It's a self-inflicted cost (Direct damage type), not an attack. Same reason it bypasses shield.

## Future Enhancements

Potential improvements:
1. **Proficiency Tooltips**: Show active proficiencies in UI
2. **Damage Breakdown**: Display damage calculation step-by-step
3. **Proficiency Synergies**: New combinations of effects
4. **Conditional Proficiencies**: Activate under specific conditions
5. **Negative Proficiencies**: Costs for powerful effects

## Next Topics

- [Combat System](../gameplay/combat-system.md) - How proficiencies affect combat
- [Game Rules](../gameplay/game-rules.md) - Proficiency rules
- [Architecture](architecture.md) - Overall system design

---

For gameplay effects, see [Gameplay Documentation](../gameplay/README.md).
