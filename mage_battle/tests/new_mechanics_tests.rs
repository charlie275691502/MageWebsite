// Unit tests for NEW battle mechanics from updated Battle_Logic.txt

use mage_battle::attribute::AttributeType;
use mage_battle::character::CharacterType;
use mage_battle::damage::DamageType;
use mage_battle::game::{Game, TurnPhase};

/// TEST 1: Shield mechanics - NO overflow to HP
/// NEW RULE (Lines 180-186): "The surplus damage from the shield DOES NOT deal to HP"
///
/// Example: 50HP, 7 shield, took 10 damage =>50HP, 0 shield (NOT 47HP)
#[test]
fn test_shield_blocks_all_damage_no_overflow() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
    ];

    let mut game = Game::new(names, chars);

    // Setup: P0 has 50 HP and 7 shield
    game.players[0].hp = 50;
    game.players[0].shield = 7;

    // Apply 10 damage (more than shield)
    game.players[0].take_damage(10, DamageType::Spell);

    // NEW BEHAVIOR: Shield blocks all damage, NO overflow to HP
    assert_eq!(game.players[0].shield, 0, "Shield should be depleted");
    assert_eq!(game.players[0].hp, 50, "HP should be UNCHANGED (shield blocks all)");
}

#[test]
fn test_shield_blocks_partial_damage() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
    ];

    let mut game = Game::new(names, chars);

    // Setup: P0 has 50 HP and 7 shield
    game.players[0].hp = 50;
    game.players[0].shield = 7;

    // Apply 5 damage (less than shield)
    game.players[0].take_damage(5, DamageType::Spell);

    // Shield absorbs the damage
    assert_eq!(game.players[0].shield, 2, "Shield should be 7 - 5 = 2");
    assert_eq!(game.players[0].hp, 50, "HP should be UNCHANGED");
}

/// TEST 2: Direct damage bypasses shield
/// Direct damage (like Wood Lv3 cost) should bypass shield entirely
#[test]
fn test_direct_damage_bypasses_shield() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
    ];

    let mut game = Game::new(names, chars);

    // Setup: P0 has 50 HP and 10 shield
    game.players[0].hp = 50;
    game.players[0].shield = 10;

    // Apply 5 direct damage (should bypass shield)
    game.players[0].take_damage(5, DamageType::Direct);

    // Shield should be untouched, HP should decrease
    assert_eq!(game.players[0].shield, 10, "Shield should be UNCHANGED");
    assert_eq!(game.players[0].hp, 45, "HP should be 50 - 5 = 45");
}

/// TEST 3: Attribute bolts target furthest alive enemy
/// NEW RULE (Lines 200-201): Attribute bolts target "furthest alived enemy"
///
/// Turn order: P0 → P1 → P2 → P3
/// P0's enemies: P1 (distance 1), P3 (distance 3)
/// Furthest enemy: P3
#[test]
fn test_attribute_bolt_targets_furthest_enemy() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
    ];

    let mut game = Game::new(names, chars);

    // Setup: P0's turn, Wood Lv3
    game.current_player_index = 0;
    game.players[0].attributes.wood = 3;
    game.players[0].hp = 50;
    game.players[0].shield = 0;

    // All players at full HP
    for i in 0..4 {
        game.players[i].hp = 50;
        game.players[i].shield = 0;
    }

    // Set turn phase to PlayCard and add a card to hand
    game.turn_phase = TurnPhase::PlayCard;
    game.players[0].draw_card(1); // Add card ID 1 to hand
    let card_id = game.players[0].hand[0];

    // P0 uses Wood attribute bolt
    let result = game.play_attribute_bolt(card_id, AttributeType::Wood);
    assert!(result.is_ok(), "Attribute bolt should succeed");

    // P0's enemies: P1 (distance 1), P3 (distance 3)
    // Furthest enemy is P3
    assert_eq!(game.players[3].hp, 47, "P3 (furthest enemy) should take 3 damage");

    // Other players should be unaffected (except P0's Wood Lv3 cost)
    assert_eq!(game.players[0].hp, 49, "P0 should have Wood Lv3 cost (-1 HP)");
    assert_eq!(game.players[1].hp, 50, "P1 should be unaffected");
    assert_eq!(game.players[2].hp, 50, "P2 (teammate) should be unaffected");
}

/// TEST 4: Furthest enemy with dead player
/// When furthest enemy is dead, target next furthest alive enemy
#[test]
fn test_attribute_bolt_skips_dead_enemy() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
    ];

    let mut game = Game::new(names, chars);

    // Setup: P0's turn, Wood Lv3
    game.current_player_index = 0;
    game.players[0].attributes.wood = 3;
    game.players[0].hp = 50;
    game.players[0].shield = 0;

    // P3 is dead (furthest enemy)
    game.players[3].hp = 0;
    game.players[3].is_dead = true;

    // P1 is alive (closer enemy)
    game.players[1].hp = 50;
    game.players[1].shield = 0;

    // Set turn phase to PlayCard and add a card to hand
    game.turn_phase = TurnPhase::PlayCard;
    game.players[0].draw_card(1); // Add card ID 1 to hand
    let card_id = game.players[0].hand[0];

    // P0 uses Wood attribute bolt
    let result = game.play_attribute_bolt(card_id, AttributeType::Wood);
    assert!(result.is_ok(), "Attribute bolt should succeed");

    // Since P3 is dead, should target P1 (next alive enemy)
    assert_eq!(game.players[1].hp, 47, "P1 should take 3 damage (P3 is dead)");
    assert_eq!(game.players[3].hp, 0, "P3 should remain dead");
}

/// TEST 5: Fire Lv3 only applies to attribute bolts
/// NEW RULE (Line 170): "Fire Lv3: +1 to if the spell is attribute bolt"
#[test]
fn test_fire_lv3_only_for_attribute_bolts() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,  // Fire character
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
    ];

    let mut game = Game::new(names, chars);

    // Setup: P0 has Fire Lv3
    game.current_player_index = 0;
    game.players[0].attributes.fire = 3;
    game.players[0].attributes.thunder = 2;  // For using Thunder bolt

    // All players at full HP
    for i in 0..4 {
        game.players[i].hp = 50;
        game.players[i].shield = 0;
    }

    // Set turn phase to PlayCard and add a card to hand
    game.turn_phase = TurnPhase::PlayCard;
    game.players[0].draw_card(1); // Add card ID 1 to hand
    let card_id = game.players[0].hand[0];

    // P0 uses Thunder attribute bolt (level 2)
    let result = game.play_attribute_bolt(card_id, AttributeType::Thunder);
    assert!(result.is_ok(), "Attribute bolt should succeed");

    // Base damage: 2 (Thunder level)
    // Fire Lv3 bonus: +1
    // Total: 3 damage
    // Target: P3 (furthest enemy)
    assert_eq!(game.players[3].hp, 47, "P3 should take 3 damage (2 base + 1 Fire Lv3)");
}

/// TEST 6: Spell damage vs Skill damage vs Direct damage
#[test]
fn test_damage_type_classifications() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
    ];

    let mut game = Game::new(names, chars);

    // Setup: P0 has Wood Lv5 (reduces spell damage by 1)
    game.players[0].hp = 50;
    game.players[0].shield = 0;
    game.players[0].attributes.wood = 5;

    // Test 1: Spell damage (reduced by Wood Lv5)
    game.players[0].take_damage(5, DamageType::Spell);
    assert_eq!(game.players[0].hp, 46, "Spell damage: 5 - 1 (Wood Lv5) = 4 actual damage");

    // Test 2: Skill damage (NOT reduced by Wood Lv5)
    game.players[0].hp = 50;
    game.players[0].take_damage(5, DamageType::Skill);
    assert_eq!(game.players[0].hp, 45, "Skill damage: 5 damage (no reduction)");

    // Test 3: Direct damage (NOT reduced by Wood Lv5, bypasses shield)
    game.players[0].hp = 50;
    game.players[0].shield = 10;
    game.players[0].take_damage(5, DamageType::Direct);
    assert_eq!(game.players[0].hp, 45, "Direct damage: 5 damage (bypasses shield)");
    assert_eq!(game.players[0].shield, 10, "Shield untouched by direct damage");
}

/// TEST 7: Guard Wood Carving only reduces Spell and Skill damage
/// NEW RULE (Line 176): "Guard Wood Carving buff: -4 from spell and skill damage"
#[test]
fn test_guard_wood_carving_spell_and_skill_only() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
    ];

    let mut game = Game::new(names, chars);

    // Setup: P0 has Guard Wood Carving buff
    game.players[0].hp = 50;
    game.players[0].shield = 0;
    let buff = mage_battle::buff::Buff::new_with_data(
        mage_battle::buff::BuffType::GuardWoodCarving,
        mage_battle::buff::BuffDuration::UntilHit(3),
        3,  // 3 hits remaining
    );
    game.players[0].buffs.add(buff);

    // Test 1: Spell damage (reduced by -4)
    game.players[0].take_damage(10, DamageType::Spell);
    assert_eq!(game.players[0].hp, 44, "10 - 4 (Guard) = 6 damage");

    // Test 2: Skill damage (reduced by -4)
    game.players[0].hp = 50;
    game.players[0].take_damage(10, DamageType::Skill);
    assert_eq!(game.players[0].hp, 44, "10 - 4 (Guard) = 6 damage");

    // Test 3: Direct damage (NOT reduced)
    game.players[0].hp = 50;
    game.players[0].take_damage(10, DamageType::Direct);
    assert_eq!(game.players[0].hp, 40, "10 damage (no reduction for direct damage)");
}
