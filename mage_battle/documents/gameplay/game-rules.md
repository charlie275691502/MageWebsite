# Game Rules

Complete rules for MageBattle - a 4-player turn-based card battle game.

## Game Structure

### Players & Teams

- **Total Players**: 4 (Player 0, 1, 2, 3)
- **Teams**: 2v2 format
  - Team A: Player 0 & Player 2
  - Team B: Player 1 & Player 3
- **Turn Order**: Sequential (0 → 1 → 2 → 3 → 0...)
- **Seating**: Players sit in a circle (P0 → P1 → P2 → P3 → back to P0)

### Win Condition

Eliminate both players of the opposing team.

## Player Stats

### HP (Health Points)

- **Initial**: 50 HP
- **Maximum**: 50 HP (can be healed over max)
- **Death**: Occurs when HP reaches 0

### Shield

- **Initial**: 0
- **Maximum**: No limit
- **Function**: Absorbs damage before HP
- **Important**: Shield does NOT absorb remaining damage (surplus damage is lost)
- **Example 1**: 50 HP, 7 shield, takes 5 damage → 50 HP, 2 shield
- **Example 2**: 50 HP, 7 shield, takes 10 damage → 50 HP, 0 shield (NOT 47 HP)
- **Exception**: Shield cannot block Direct damage type

### Death & Revival

**When HP = 0:**
- Player dies immediately
- All cards in hand are discarded
- All buffs/debuffs are removed
- Cannot act during death

**Death Counter:**
- Tracks turns dead (increments on player's turn)

**Revival:**
- After 2 turns dead, player revives automatically
- Revives with 50% HP (25 HP)

## Attributes & Proficiency

### Attribute Types

**Main Attributes** (4):
- 🔥 Fire (F)
- 🌳 Wood (W)
- ⚡ Thunder (T)
- 💧 Water (Wa)

**Sub Attributes** (2):
- 🌪️ Wind (Wi)
- ☠️ Poison (Po)

### Attribute Levels

- **Range**: 0 to 5
- **Allocation**: 1 point per turn
- **Maximum**: Cannot exceed level 5 in any single attribute
- **Full Cap**: If every attribute reaches level 5, skip attribute allocation phase

### Attribute Proficiency Bonuses

Passive bonuses unlocked at certain attribute levels:

#### 🔥 Fire
- **Lv3**: All attribute bolts +1 damage
- **Lv5**: At turn start, deal 1 skill damage to all enemies
  - Bypasses Wood Lv5 reduction

#### 🌳 Wood
- **Lv3**: When using Wood spell:
  - Cost: -1 HP (DIRECT, bypasses shield entirely)
  - Benefit: +1 Shield
  - Example: 45 HP, 1 shield → trigger Wood Lv3 → 44 HP, 2 shield
- **Lv5**: Self and teammate receive -1 spell damage from all sources

#### ⚡ Thunder
- **Lv3**: Thunder spells +1 damage
- **Lv5**: Thunder spells +2 additional damage (total +3 with Lv3)

#### 💧 Water
- **Lv3**: When using Water spell, heal self +1 HP
- **Lv5**: When using Water spell, also heal teammate +1 HP

#### 🌪️ Wind
- **Lv2**: Targets hit by Wind spells cannot heal or gain shield this round
- **Lv5**: Wind spells can target any player (not restricted to default targeting)

#### ☠️ Poison
- **Lv2**: Targets hit by Poison spells get Confuse debuff (play card before allocating next turn)
- **Lv5**: Targets hit by Poison spells get Silent debuff (can only use attribute bolts next turn)

## Turn Structure

Each player's turn consists of 5 phases:

### 1. Turn Start Phase

Automatic effects trigger:
- Death counter increments (if dead)
- Revival check (if dead for 2 turns)
- Fire Lv5 effect (1 damage to all enemies)
- Health Drain effect (if applicable)
- Regeneration effect (if applicable)
- Burning Out effect (consumes 1 Fire attribute point)

### 2. Allocate Attribute Phase

- Current player allocates 1 attribute point
- Choose from: Fire, Wood, Thunder, Water, Wind, Poison
- Cannot exceed level 5
- **Special**: If Confused, this phase happens AFTER Play Card

### 3. Play Card Phase

Current player **must** play exactly 1 card:

**Option A: Cast a Spell**
- Choose top or bottom side of a card
- Must meet attribute requirements
- Select targets based on spell

**Option B: Use Attribute Bolt**
- Discard any card
- Use one of your attributes as an attack
- Damage = attribute level
- Target = furthest alive enemy (automatic)

**Option C: Use Liberation Skill**
- Must meet liberation requirements
- Can only be used once per game
- See [Characters](characters.md) for details

Cannot skip this phase!

### 4. Draw Card Phase

- Draw 1 card from deck
- If deck is empty, shuffle discard pile into deck
- If player is liberated, draw +1 additional card (total 2)

### 5. Turn End Phase

Automatic cleanup:
- Buff/debuff durations tick down
- Expired effects are removed
- Next player's turn begins

## Cards & Spells

### Card Structure

- Each card has 1 or 2 spell sides
- **Top Spell**: Always present
- **Bottom Spell**: Optional (some cards are single-sided)
- Cards stored in spells.json

### Spell Requirements

- Format example: "F2W1" means Fire Lv2 AND Wood Lv1 required
- Must meet ALL requirements to cast
- Requirements checked at casting time

### Spell Effects

Common spell effects:
- **Damage**: Deal X damage to target(s)
- **Heal**: Restore X HP to target(s)
- **Shield**: Grant X shield to target(s)
- **Buffs/Debuffs**: Apply status effects
- **Special**: Various unique effects (see cards)

### Attribute Bolts

Special attack using attributes:

- **Requirements**:
  - 1 any card (discarded)
  - Attribute level ≥ 1
- **Damage**: Equal to attribute level
- **Target**: Furthest alive enemy (automatic)
- **Type**: Counts as spell damage
- **Enchantment**: Considered to have the bolt's attribute for proficiency bonuses
- **Proficiency**:
  - Fire Lv3 adds +1 damage to ALL attribute bolts
  - Thunder Lv3/5 adds +1/+3 to Thunder bolts specifically

**Example:**
- Fire level 3, Thunder level 5
- Use Fire bolt → Damage = 3 (Fire level) + 1 (Fire Lv3 proficiency) = 4
- Use Thunder bolt → Damage = 5 (Thunder level) + 1 (Fire Lv3 proficiency) + 3 (Thunder Lv5 proficiency) = 9

## Targeting Rules

### Default Targeting

For most spells (if not specified otherwise):
- **Must target furthest alive enemy**
- **Cannot target**: Dead players

### Furthest Alive Enemy

Definition: Alive enemy with maximum distance going forward in turn order.

**Example (for Player 0):**
- P1 (distance 1) - enemy
- P2 (distance 2) - teammate
- P3 (distance 3) - enemy
- **Furthest alive enemy** = P3

**Example (for Player 0, P3 is dead):**
- P1 (distance 1) - enemy
- P2 (distance 2) - teammate
- P3 (distance 3) - enemy (DEAD)
- **Furthest alive enemy** = P1

### Attribute Bolt Targeting

- **Always targets** furthest alive enemy
- Cannot choose target manually
- Wind Lv5 does NOT affect attribute bolt targeting

### Spell Card Targeting

Varies by spell specification:
- **Default**: Furthest alive enemy
- **Self**: The caster
- **Ally**: Caster or teammate
- **Enemy**: Any living enemy
- **All**: Multiple targets
- **Wind Lv5 Override**: Allows free target selection for Wind spells

## Deck & Hand Management

### Starting Setup
- **Deck Size**: 60 cards total (shared)
- **Starting Hand**: 5 cards per player

### During Game
- **Draw**: 1 card per turn (2 if liberated)
- **Discard Pile**: Used cards go here
- **Reshuffle**: When deck empty, shuffle discard pile into deck

### Death & Cards
- When player dies, all hand cards discarded
- When player revives, they draw 5 new cards

## Special Rules

### Proficiency Stacking

Multiple proficiency bonuses stack additively.

**Example:**
- Spell with Fire & Thunder requirements
- Caster has Fire Lv3 and Thunder Lv5
- Base damage: 10
- Fire Lv3: +1
- Thunder Lv3: +1
- Thunder Lv5: +2
- **Total**: 10 + 1 + 1 + 2 = 14 damage

### Spell Enchantments

A spell is "enchanted" with attributes based on its requirements:
- Example: "F2W1" spell is enchanted with Fire and Wood
- Enchantments determine which proficiency bonuses apply
- Wood Lv3 cost triggers for ANY Wood-enchanted spell

### Liberation Skills

- Each character has 1 unique liberation skill
- Requires specific attribute levels
- Can only be used once per game
- Replaces "Play Card" phase
- After using liberation, character gains +1 card draw permanently

See [Characters](characters.md) for specific liberation skills.

## Example Turn

**Player 0's turn:**

1. **Turn Start**:
   - Fire Lv5 triggers: 1 damage to P1 and P3
   - No other effects

2. **Allocate Attribute**:
   - Chooses to add 1 point to Thunder
   - Thunder: 3 → 4

3. **Play Card**:
   - Uses Thunder bolt (discard card 15)
   - Damage: 4 (Thunder level) + 1 (Fire Lv3) + 3 (Thunder Lv5) = 8
   - Target: P3 (furthest enemy)
   - P3 takes 8 damage

4. **Draw Card**:
   - Draws card from deck
   - Turn ends

5. **Turn End**:
   - Buffs tick down
   - Next turn: Player 1

## Next Topics

- [Combat System](combat-system.md) - Detailed damage calculation
- [Characters](characters.md) - Character types and liberation skills
- [Buffs & Debuffs](buffs-debuffs.md) - Status effects

---

For implementation details, see [Technical Documentation](../technical/README.md).
