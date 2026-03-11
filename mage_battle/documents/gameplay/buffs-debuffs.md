# Buffs & Debuffs

Complete guide to status effects in MageBattle.

## Overview

Buffs and debuffs are temporary status effects that modify player abilities and stats. They have various durations and can stack or override each other.

## Positive Buffs

### 🛡️ Immune (免疫)

**Effect**:
- Immune to all damage (Spell, Skill, and Direct)
- Immune to all negative debuffs
- New debuffs cannot be applied

**Duration**: Varies by source

**Notes**:
- Does NOT prevent self-inflicted costs (Wood Lv3)
- Does NOT prevent existing buffs from expiring

**Source**: Spell cards

---

### ⚡ Invincible (化身)

**Effect**:
- Same as Immune
- Immune to all damage
- Immune to all negative debuffs

**Duration**: Until next player completes their turn (UntilNextPlayer)

**Notes**:
- Shorter duration than Immune
- Otherwise identical effect

**Source**: Spell cards

---

### 💚 Regeneration (再生)

**Effect**:
- Heal 7 HP at turn start (before any actions)
- Healing cannot be prevented by Defense Invalidation

**Duration**: Turns-based (varies)

**Timing**:
- Triggers during Turn Start phase
- Applies before Fire Lv5 damage
- Applies before actions

**Source**:
- Hurricane Eye (Water Wind liberation): 4 turns
- Spell cards: varies

---

### 🔥 Burning Out (燃燒殆盡)

**Effect**:
- Fire damage +5 (all Fire spells and bolts)
- Cost: Consume 1 Fire attribute at turn start

**Duration**: Permanent (until Fire = 0)

**Mechanics**:
- If Fire reaches 0, buff is removed
- Can be maintained by allocating Fire
- Stacks with other Fire bonuses

**Source**: Flame Poison liberation skill

**Example**:
```
Fire Lv4 with Burning Out:
- Fire bolt damage: 4 + 1 (Fire Lv3) + 5 (Burning Out) = 10
- Turn start: Fire 4 → 3 (consumed)
- Next bolt: 3 + 1 + 5 = 9 damage
```

---

### 🌳 Guard Wood Carving (守護木雕)

**Effect**:
- Reduce damage taken by 4 from all sources
- Applies to both spell and skill damage
- Each instance of damage decrements hit counter

**Duration**: Until hit 3 times (UntilHit)

**Mechanics**:
- When target takes damage, counter decrements
- After 3 hits, buff expires
- Each player has independent counter if applied to multiple targets

**Source**: Wood Wind liberation skill (affects self and teammate)

**Example**:
```
Hit 1: 10 damage → 6 damage (Guard 3 → 2)
Hit 2: 15 damage → 11 damage (Guard 2 → 1)
Hit 3: 8 damage → 4 damage (Guard 1 → 0, buff expires)
Hit 4: 10 damage → 10 damage (no guard)
```

---

### 💉 Health Drain (生命汲取)

**Effect**:
- At your turn start, deal 1 damage to target
- Heal self 1 HP
- Target gets Health Drain Target debuff

**Duration**: Varies

**Data**: Stores target player ID

**Notes**:
- Positive buff for caster
- Negative debuff for target
- Simultaneous damage and healing

**Source**: Spell cards

## Negative Debuffs

### 😵 Paralysis (癱瘓)

**Effect**:
- Cannot take any actions
- Turn is skipped entirely

**Duration**: Turns-based

**Blocked By**:
- Immune buff
- Invincible buff

**Source**: Spell cards

---

### 🔒 Seal (封印)

**Effect**:
- Cannot use liberation skill
- All other actions allowed

**Duration**: Turns-based

**Blocked By**:
- Immune buff
- Invincible buff

**Source**: Spell cards

**Notes**:
- Can still play cards and use attribute bolts
- Only prevents liberation

---

### 🤐 Silent (沈默)

**Effect**:
- Can only use attribute bolts
- Cannot cast spell cards
- Cannot use liberation skill

**Duration**: Turns-based

**Blocked By**:
- Immune buff
- Invincible buff

**Source**:
- Poison Lv5 proficiency (targets hit by Poison spells)
- Spell cards

**Notes**:
- Still must play a card (discarded for attribute bolt)
- Attribute allocation still works

---

### ⚠️ Master Disable (元素剝離)

**Effect**:
- Attribute proficiency bonuses do not apply
- All passive proficiencies disabled

**Duration**: Turns-based

**Blocked By**:
- Immune buff
- Invincible buff

**Affected Proficiencies**:
- Fire Lv3 (+1 bolt damage)
- Fire Lv5 (turn start damage)
- Wood Lv3 (shield gain, but HP cost still applies)
- Wood Lv5 (damage reduction)
- Thunder Lv3/5 (damage bonuses)
- Water Lv3/5 (healing bonuses)
- Wind Lv2/5 (targeting/debuffs)
- Poison Lv2/5 (debuffs)

**Source**: Spell cards

**Important**: Wood Lv3 HP cost (-1 HP) STILL applies!

---

### 🚫 Defense Invalidation (防禦崩解)

**Effect**:
- Cannot heal HP
- Cannot gain shield
- All healing effects are blocked

**Duration**: Turns-based

**Blocked By**:
- Immune buff
- Invincible buff

**Source**:
- Wind Lv2 proficiency (targets hit by Wind spells)
- Spell cards

**Example**:
```
With Defense Invalidation:
- Heal 10 HP → No effect
- Gain 5 shield → No effect
- Water Lv3 healing → Blocked
- Regeneration buff → Blocked
```

---

### 😵‍💫 Confuse (混亂)

**Effect**:
- Turn order changed: Play Card BEFORE Allocate Attribute
- Normal order: Allocate → Play → Draw
- Confused order: Play → Allocate → Draw

**Duration**: Turns-based (affects next turn)

**Blocked By**:
- Immune buff
- Invincible buff

**Source**:
- Poison Lv2 proficiency (targets hit by Poison spells)
- Spell cards

**Strategic Impact**:
- Cannot allocate attribute before playing card
- May prevent meeting spell requirements
- Forces use of attribute bolts if spell requirements not met

---

### 🩸 Health Drain Target (寄主)

**Effect**:
- Take 1 damage at drainer's turn start
- Drainer heals 1 HP simultaneously

**Duration**: Matches Health Drain buff duration

**Linked To**: Player with Health Drain buff

**Notes**:
- Applied automatically when Health Drain buff is applied
- Cannot be blocked by Immune (already applied)
- Expires when drainer's Health Drain expires

## Duration Types

### Permanent

- **Lasts**: Indefinitely until manually removed or condition met
- **Examples**: Burning Out (until Fire = 0)

### Turns (N)

- **Lasts**: N of the affected player's turns
- **Decrements**: At turn end
- **Expiration**: When counter reaches 0

**Example**:
```
Buff: Regeneration (Turns 3)
Turn 1: Triggers, counter 3 → 2
Turn 2: Triggers, counter 2 → 1
Turn 3: Triggers, counter 1 → 0, buff expires
```

### UntilHit (N)

- **Lasts**: Until hit N times
- **Decrements**: On each damage instance
- **Expiration**: When counter reaches 0

**Example**:
```
Buff: Guard Wood Carving (UntilHit 3)
Hit 1: Effect applies, counter 3 → 2
Hit 2: Effect applies, counter 2 → 1
Hit 3: Effect applies, counter 1 → 0, buff expires
```

**Notes**:
- Each separate damage counts as one hit
- AOE hitting multiple times in one action = 1 hit per target

### UntilNextPlayer

- **Lasts**: Until the next player completes their turn
- **Expiration**: After next player's Turn End phase

**Example**:
```
P0's turn: P0 gains Invincible (UntilNextPlayer)
P1's turn: P0 still has Invincible
P1 turn end: P0 loses Invincible
P2's turn: P0 no longer has Invincible
```

## Buff/Debuff Interactions

### Immunity Blocking

Immune and Invincible prevent:
- ✅ All damage
- ✅ New negative debuffs from being applied

Do NOT prevent:
- ❌ Existing debuffs from continuing
- ❌ Self-inflicted costs (Wood Lv3)
- ❌ Positive buff durations expiring

### Stacking Rules

**Multiple Same Buffs**:
- Duration is updated to the longer one
- Example: Regeneration (2 turns) + Regeneration (4 turns) = Regeneration (4 turns)

**Multiple Different Buffs**:
- All apply simultaneously
- Example: Immune + Regeneration (both active)

**Multiple Debuffs**:
- All apply simultaneously
- Example: Can be Paralyzed + Sealed + Silent simultaneously

### Cleansing

**Death Cleanse**:
- Upon death, ALL buffs and debuffs removed
- Player starts fresh upon revival

**No Cleanse Spells** (currently):
- No spell cards that remove debuffs
- Must wait for duration to expire

## Common Scenarios

### Scenario 1: Immune vs Health Drain Target

```
Q: Player has Immune buff and Health Drain Target debuff. Do they take damage?
A: Yes. Immune only prevents NEW debuffs. Existing debuffs still apply.
```

### Scenario 2: Defense Invalidation vs Regeneration

```
Q: Player has both Defense Invalidation and Regeneration. Does healing work?
A: No. Defense Invalidation blocks all healing, including Regeneration.
```

### Scenario 3: Master Disable vs Wood Lv3

```
Q: Player has Master Disable. Does Wood Lv3 cost still apply?
A: HP cost (-1 HP): YES, still applies
   Shield gain (+1 shield): NO, blocked by Master Disable
```

### Scenario 4: Guard Wood Carving + Wood Lv5

```
Q: Do damage reductions stack?
A: Yes. -4 from Guard + -1 from Wood Lv5 = -5 total
```

### Scenario 5: Burning Out Fire = 0

```
Q: What happens when Fire reaches 0 with Burning Out?
A: Burning Out buff is removed immediately. No more +5 damage bonus.
```

## Strategic Tips

### Using Buffs Effectively

1. **Immune/Invincible**: Save for enemy's powerful attacks
2. **Regeneration**: Use early for sustained value
3. **Burning Out**: Rush early, maintain Fire allocation
4. **Guard Wood Carving**: Coordinate with teammate, use before fights

### Dealing with Debuffs

1. **Paralysis**: Most devastating, hard to counter
2. **Silent**: Keep attribute bolts as backup
3. **Confuse**: Plan attributes ahead
4. **Master Disable**: Play cards with low requirements
5. **Defense Invalidation**: Avoid healing spells, focus on avoiding damage

### Debuff Combos

Effective debuff combinations:
- **Silent + Seal**: Prevents liberation and spell cards
- **Confuse + high requirement spells**: Forces weak attribute bolts
- **Defense Invalidation + sustained damage**: Prevents recovery
- **Master Disable + attribute-dependent builds**: Removes their advantage

## Next Topics

- [Game Rules](game-rules.md) - Core game mechanics
- [Combat System](combat-system.md) - Damage calculation
- [Characters](characters.md) - Liberation skills that apply buffs

---

For implementation details, see [Proficiency System](../technical/proficiency-system.md).
