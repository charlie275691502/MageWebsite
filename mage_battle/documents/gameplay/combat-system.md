# Combat System

Detailed explanation of damage calculation, types, and mechanics in MageBattle.

## Damage Types

There are 3 distinct damage types that affect how damage is calculated and applied:

### 1. Spell Damage

**Source**: Spell cards and attribute bolts

**Affected By**:
- ✅ Shield (absorbed first)
- ✅ Wood Lv5 proficiency (-1 reduction)
- ✅ Guard Wood Carving buff (-4 reduction)
- ✅ Immune/Invincible buffs (completely blocked)

**Examples**:
- Casting a damage spell card
- Using attribute bolts

### 2. Skill Damage

**Source**: Character abilities and passive effects

**Affected By**:
- ✅ Shield (absorbed first)
- ❌ Wood Lv5 proficiency (NOT reduced)
- ✅ Guard Wood Carving buff (-4 reduction)
- ✅ Immune/Invincible buffs (completely blocked)

**Examples**:
- Fire Lv5 passive (1 damage to all enemies at turn start)
- Liberation skills that deal damage

**Key Difference**: Bypasses Wood Lv5 spell damage reduction!

### 3. Direct Damage

**Source**: Special costs and effects

**Affected By**:
- ❌ Shield (completely bypassed)
- ❌ Wood Lv5 proficiency (not affected)
- ❌ Guard Wood Carving buff (not affected)
- ❌ Immune/Invincible buffs (not affected)

**Examples**:
- Wood Lv3 proficiency HP cost (-1 HP)
- Special card effects that specify "direct"

**Key Difference**: Cannot be blocked or reduced by anything!

## Damage Calculation Flow

When damage is dealt, follow these steps:

### Step 1: Calculate Base Damage

Base damage from the spell/skill/effect.

### Step 2: Apply Caster's Proficiency Bonuses

**If attribute bolt or spell with matching attribute:**

- **Fire Lv3**: +1 to all attribute bolts
- **Thunder Lv3**: +1 to Thunder spells/bolts
- **Thunder Lv5**: +2 additional to Thunder spells/bolts (total +3)
- **Burning Out buff**: +5 to Fire spells/bolts

**Example:**
```
Base damage: 5 (Thunder bolt, Thunder level 5)
Thunder Lv3: +1
Thunder Lv5: +2
Fire Lv3: +1 (applies to ALL bolts)
Total after bonuses: 5 + 1 + 2 + 1 = 9
```

### Step 3: Apply Target's Damage Reduction

**For Spell Damage only:**
- **Wood Lv5**: -1 from all spell damage

**For Spell & Skill Damage:**
- **Guard Wood Carving buff**: -4 from all spell and skill damage
  - Decrements hit counter after applying
  - Expires after 3 hits

**Example:**
```
Damage after bonuses: 9
Target has Wood Lv5: -1
Target has Guard Wood Carving: -4
Final damage: 9 - 1 - 4 = 4
```

### Step 4: Apply Final Damage

```
Final Damage = max(0, damage after all modifiers)
```

Damage cannot go negative. Minimum is 0.

### Step 5: Apply to HP/Shield

See [Shield Mechanics](#shield-mechanics) below.

## Shield Mechanics

### How Shield Works

1. **Damage Application Order**:
   - Shield absorbs damage first
   - Remaining damage goes to HP
   - **Important**: Surplus damage from shield does NOT carry over!

2. **Examples**:

**Example 1: Damage less than shield**
```
Current: 50 HP, 7 Shield
Takes: 5 damage
Result: 50 HP, 2 Shield
```

**Example 2: Damage equals shield**
```
Current: 50 HP, 7 Shield
Takes: 7 damage
Result: 50 HP, 0 Shield
```

**Example 3: Damage more than shield (IMPORTANT!)**
```
Current: 50 HP, 7 Shield
Takes: 10 damage
Result: 50 HP, 0 Shield (NOT 47 HP!)
```

Shield "breaks" and absorbs ALL remaining shield value, but excess damage is LOST.

### Shield Properties

- **No Maximum**: Shield can stack indefinitely
- **No Natural Decay**: Shield persists until hit or removed
- **Gaining Shield**: Multiple sources stack additively
- **Bypassed By**: Direct damage type

### Shield Interactions

**Can Block**:
- ✅ Spell damage
- ✅ Skill damage

**Cannot Block**:
- ❌ Direct damage (Wood Lv3 cost, special effects)

**Affected By**:
- Defense Invalidation debuff (cannot gain shield)

## Special Cases

### Wood Lv3 HP Cost

This is the ONLY proficiency that costs HP:

**Trigger**: When using ANY Wood-enchanted spell or bolt

**Effect**:
- -1 HP (DIRECT damage type)
- +1 Shield

**Important Properties**:
- HP cost is DIRECT damage (bypasses shield)
- HP cost applied BEFORE spell effect
- Can cause death before spell resolves
- NOT affected by damage reductions

**Example**:
```
Before: 45 HP, 1 Shield, cast Wood spell
Wood Lv3 cost: -1 HP (DIRECT)
After cost: 44 HP, 2 Shield
Then spell effect resolves
```

**Death Example**:
```
Before: 1 HP, 0 Shield, cast Wood spell
Wood Lv3 cost: -1 HP (DIRECT)
After cost: 0 HP → Player dies
Spell effect does NOT resolve (caster dead)
```

### Burning Out Fire Damage Bonus

**Effect**: +5 damage to all Fire spells and Fire bolts

**Trigger**: When casting Fire-enchanted spell/bolt

**Stacks With**:
- Fire Lv3 (+1)
- Thunder Lv3/5 (if also Thunder-enchanted)

**Example**:
```
Base damage: 10 (Fire & Thunder spell)
Fire Lv3: +1
Thunder Lv5: +3
Burning Out: +5
Total: 19 damage
```

### Immunity Effects

**Immune and Invincible buffs block**:
- All damage (Spell, Skill, and Direct)
- All negative debuffs

**Cannot Block**:
- Self-inflicted costs (Wood Lv3)
- Positive buff durations expiring

## Damage Calculation Examples

### Example 1: Simple Thunder Bolt

**Setup**:
- Caster: Thunder Lv5, Fire Lv3
- Target: 50 HP, 0 Shield, no reductions

**Calculation**:
```
1. Base: 5 (Thunder level)
2. Bonuses: +1 (Fire Lv3), +3 (Thunder Lv5) = 9
3. Reductions: None
4. Final: 9 damage
5. Result: Target → 41 HP
```

### Example 2: Fire Spell vs Wood Defender

**Setup**:
- Caster: Fire Lv5, Burning Out buff
- Spell: 10 base Fire damage
- Target: 50 HP, 0 Shield, Wood Lv5

**Calculation**:
```
1. Base: 10
2. Bonuses: +1 (Fire Lv3), +5 (Burning Out) = 16
3. Reductions: -1 (Wood Lv5) = 15
4. Final: 15 damage
5. Result: Target → 35 HP
```

### Example 3: Attack with Shield and Guard Buff

**Setup**:
- Caster: Thunder Lv5
- Spell: 10 base Thunder damage
- Target: 50 HP, 3 Shield, Guard Wood Carving, Wood Lv5

**Calculation**:
```
1. Base: 10
2. Bonuses: +3 (Thunder Lv5) = 13
3. Reductions: -1 (Wood Lv5), -4 (Guard) = 8
4. Shield: 8 - 3 = 5 damage to HP
5. Result: Target → 45 HP, 0 Shield
   Guard Wood Carving: 3 hits → 2 hits remaining
```

### Example 4: Wood Lv3 Cost + Spell

**Setup**:
- Caster: Wood Lv3, 45 HP, 1 Shield
- Spell: Wood-enchanted heal 10 HP

**Calculation**:
```
1. Wood Lv3 cost applies FIRST:
   - HP: 45 → 44 (DIRECT, bypasses shield)
   - Shield: 1 → 2 (+1 from Wood Lv3)

2. Spell effect applies:
   - Heal 10 HP: 44 → 50 HP (capped at max)

3. Final: 50 HP, 2 Shield
```

### Example 5: Fire Lv5 Passive

**Setup**:
- Player: Fire Lv5
- Enemies: P1 (50 HP, Wood Lv5), P3 (50 HP, 0 Shield)

**Calculation (Turn Start)**:
```
P1:
1. Damage type: Skill (NOT Spell)
2. Base: 1
3. Reductions: None (Wood Lv5 doesn't affect Skill damage)
4. Final: 1 damage
5. Result: P1 → 49 HP

P3:
1. Same as P1
2. Result: P3 → 49 HP
```

## Targeting Priority

When multiple valid targets exist, follow these rules:

### Spell Card Targeting

Depends on spell's TargetPool:
- **Default**: Furthest alive enemy (required)
- **Ally**: Can choose self or teammate
- **Enemy**: Can choose any alive enemy
- **Self**: Always self
- **All**: Hits all in category

**Wind Lv5 Override**:
- Wind spells can target anyone (ignores restrictions)

### Attribute Bolt Targeting

- **Always** targets furthest alive enemy
- **Cannot** be manually changed
- **Not affected** by Wind Lv5

### Furthest Alive Enemy Logic

Go forward in turn order, pick alive enemy with max distance.

**Example (Player 0 perspective)**:
```
P0 (you) → P1 (distance 1, enemy) → P2 (distance 2, teammate) → P3 (distance 3, enemy)

Furthest alive enemy = P3

If P3 is dead:
Furthest alive enemy = P1
```

## Next Topics

- [Game Rules](game-rules.md) - Core game mechanics
- [Buffs & Debuffs](buffs-debuffs.md) - Status effects
- [Characters](characters.md) - Liberation skills and special abilities

---

For implementation details, see [Technical Documentation](../technical/README.md).
