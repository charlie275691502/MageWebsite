# Characters

Character types, initial attributes, and liberation skills in MageBattle.

## Character Overview

MageBattle features 4 unique character types, each with:
- Specific starting attributes
- One main attribute and one sub attribute
- A powerful liberation skill unlocked mid-game

## Character Types

### 🔥☠️ Flame Poison Mage (火毒法師)

**Character ID**: `FlamePoison`

**Initial Attributes**:
- Fire: 2
- Poison: 1
- All others: 0

**Main/Sub**:
- Main: Fire
- Sub: Poison

**Liberation Skill: Burning Out (燃燒殆盡)**

**Requirements**:
- Fire Lv4
- Poison Lv2

**Effect**:
- Fire damage +5 (applies to all Fire spells and Fire bolts)
- At each turn start, consume 1 Fire attribute point
- Duration: Permanent (until Fire reaches 0)

**Strategy**:
- Offensive powerhouse
- Maximize Fire damage quickly
- Plan ahead - skill ends when Fire reaches 0
- Can maintain by continuing to allocate Fire after liberation

**Example**:
```
Before liberation:
- Fire bolt (Lv4) = 4 + 1 (Fire Lv3) = 5 damage

After liberation:
- Fire bolt (Lv4) = 4 + 1 (Fire Lv3) + 5 (Burning Out) = 10 damage
- Next turn: Fire Lv4 → Lv3 (consumed)
```

---

### 🌳🌪️ Wood Wind Mage (木風法師)

**Character ID**: `WoodWind`

**Initial Attributes**:
- Wood: 2
- Wind: 1
- All others: 0

**Main/Sub**:
- Main: Wood
- Sub: Wind

**Liberation Skill: Guard Wood Carving (守護木雕)**

**Requirements**:
- Wood Lv4
- Wind Lv2

**Effect**:
- Self and teammate take -4 damage from all sources
- Applies to both spell and skill damage
- Duration: Until hit 3 times (per player)
- Each hit decrements the counter

**Strategy**:
- Defensive support
- Protects entire team
- Best used before major enemy attacks
- Stacks with Wood Lv5 damage reduction

**Example**:
```
After liberation:
- Teammate has Guard Wood Carving (3 hits left)
- Enemy deals 10 damage spell
- Damage: 10 - 4 (Guard) = 6 damage taken
- Guard Wood Carving: 3 → 2 hits remaining

Combined with Wood Lv5:
- Enemy deals 10 spell damage
- Damage: 10 - 1 (Wood Lv5) - 4 (Guard) = 5 damage
```

---

### ⚡☠️ Thunder Poison Mage (雷毒法師)

**Character ID**: `ThunderPoison`

**Initial Attributes**:
- Thunder: 2
- Poison: 1
- All others: 0

**Main/Sub**:
- Main: Thunder
- Sub: Poison

**Liberation Skill: Chain Lightning (鏈鎖電擊)**

**Requirements**:
- Thunder Lv4
- Poison Lv2

**Effect**:
- Deal 10 skill damage to ALL other players (enemies AND ally)
- Type: Instant (no duration)
- Damage type: Skill (not affected by Wood Lv5)

**Strategy**:
- High-risk, high-reward
- Damages ALL other players including teammate!
- Best used when enemies are low HP
- Coordinate with teammate to avoid killing them
- Massive damage when all 3 opponents are alive

**Example**:
```
Game state:
- P0 (you): 50 HP
- P1 (enemy): 30 HP
- P2 (teammate): 40 HP
- P3 (enemy): 25 HP

After Chain Lightning:
- P1: 30 → 20 HP
- P2: 40 → 30 HP (teammate also hit!)
- P3: 25 → 15 HP

Total damage dealt: 30 damage across 3 players
```

---

### 💧🌪️ Water Wind Mage (水風法師)

**Character ID**: `WaterWind`

**Initial Attributes**:
- Water: 2
- Wind: 1
- All others: 0

**Main/Sub**:
- Main: Water
- Sub: Wind

**Liberation Skill: Hurricane Eye (颶風之眼)**

**Requirements**:
- Water Lv4
- Wind Lv2

**Effect**:
- Regeneration buff: Heal 7 HP at each turn start
- Duration: 4 turns
- Healing cannot be prevented by Defense Invalidation

**Strategy**:
- Sustained healing support
- Excellent for surviving extended battles
- Combines well with Water Lv3/5 for additional healing
- Total healing: 7 × 4 = 28 HP over 4 turns

**Example**:
```
Turn 1 start: 30 HP → 37 HP (Regeneration)
Turn 2 start: 37 HP → 44 HP (Regeneration)
Turn 3 start: 44 HP → 50 HP (Regeneration, capped at max)
Turn 4 start: 50 HP → 50 HP (Regeneration, already at max)
Buff expires after Turn 4
```

## Liberation Mechanics

### General Rules

1. **One-Time Use**: Each liberation can only be used once per game
2. **Replaces Play Card Phase**: Using liberation counts as your card play action
3. **Still Draw**: You still draw cards after using liberation
4. **Permanent Bonus**: After liberation, gain +1 card draw per turn (draw 2 instead of 1)
5. **Cannot Be Sealed**: Must have both required attribute levels AND not have Seal debuff

### Liberation Timing

Liberation can be used during your Play Card phase when:
- ✅ You meet the attribute requirements
- ✅ You don't have Seal debuff
- ✅ You haven't used liberation yet this game
- ✅ You're alive

### After Liberation

**Permanent Changes**:
- +1 card draw per turn (2 total instead of 1)
- Marked as "liberated" for the rest of the game
- Cannot use liberation again

**Liberation-Specific Effects**:
- Depend on character (see above)
- Some are permanent (Burning Out)
- Some have durations (Hurricane Eye, Guard Wood Carving)
- Some are instant (Chain Lightning)

## Character Selection Strategy

### Offensive Team Composition

**Flame Poison + Thunder Poison**
- Pros:
  - High burst damage
  - Multiple damaging liberations
  - Fire Lv5 passive synergy
- Cons:
  - Chain Lightning hits teammate
  - Requires coordination
  - Less defensive options

### Defensive Team Composition

**Wood Wind + Water Wind**
- Pros:
  - Strong damage reduction
  - Sustained healing
  - Good survivability
- Cons:
  - Lower damage output
  - Slower game pace
  - Vulnerable to burst damage

### Balanced Team Composition

**Flame Poison + Wood Wind** or **Thunder Poison + Water Wind**
- Pros:
  - Mix of offense and defense
  - Flexible strategy
  - Can adapt to opponent
- Cons:
  - Less specialized
  - Requires good coordination

## Attribute Synergies

### Fire-based Characters (Flame Poison)

**Natural Synergies**:
- Fire Lv3: Boosts all attribute bolts
- Fire Lv5: Passive damage to enemies
- Burning Out: Massive damage boost

**Recommended Build**:
1. Rush Fire to Lv4 for liberation
2. Get Poison to Lv2
3. Use liberation early
4. Continue allocating Fire for sustainability
5. Add Thunder for additional damage proficiency

### Wood-based Characters (Wood Wind)

**Natural Synergies**:
- Wood Lv3: Shield generation
- Wood Lv5: Team damage reduction
- Guard Wood Carving: Additional team protection

**Recommended Build**:
1. Get Wood to Lv4
2. Add Wind to Lv2 for liberation
3. Use liberation before major fights
4. Get Wood Lv5 for double damage reduction
5. Add Water for healing

### Thunder-based Characters (Thunder Poison)

**Natural Synergies**:
- Thunder Lv3/5: Massive spell damage
- Fire Lv3: Bolt damage boost
- Chain Lightning: AOE damage

**Recommended Build**:
1. Rush Thunder to Lv4-5
2. Get Poison to Lv2 for liberation
3. Add Fire Lv3 for bolt boost
4. Use liberation when enemies are grouped and low HP
5. Coordinate with teammate first!

### Water-based Characters (Water Wind)

**Natural Synergies**:
- Water Lv3/5: Self and team healing
- Hurricane Eye: Sustained regeneration
- Wind Lv2/5: Control effects

**Recommended Build**:
1. Get Water to Lv4
2. Add Wind to Lv2
3. Get Water Lv5 for team healing
4. Use liberation mid-fight for sustained healing
5. Add Wood for defense

## Next Topics

- [Game Rules](game-rules.md) - Basic game mechanics
- [Combat System](combat-system.md) - Damage calculation
- [Buffs & Debuffs](buffs-debuffs.md) - Status effects

---

For implementation details, see [Technical Documentation](../technical/README.md).
