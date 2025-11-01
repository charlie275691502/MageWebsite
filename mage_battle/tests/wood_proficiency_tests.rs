// Tests for Wood Lv3 proficiency with attribute bolts
// Wood Lv3: 使用木屬性法術時，減少1點生命並獲得1點護盾

use mage_battle::attribute::AttributeType;
use mage_battle::card::CardSide;
use mage_battle::character::CharacterType;
use mage_battle::game::{Game, TurnPhase};
use mage_battle::damage::DamageType;

fn setup_test_game() -> Game {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::WoodWind,      // P0: Wood specialist with Wood Lv3
        CharacterType::FlamePoison,   // P1: Enemy target
        CharacterType::ThunderPoison, // P2: Teammate
        CharacterType::WaterWind,     // P3: Enemy
    ];
    Game::new(names, chars)
}

/// Test Wood Lv3 proficiency triggers with attribute bolt (wood bolt)
/// Wood Lv3: Caster loses 1 HP and gains 1 shield when using wood attribute bolt
#[test]
fn test_wood_lv3_proficiency_with_attribute_bolt() {
    let mut game = setup_test_game();
    let caster = 0;
    // In turn order P0, P1, P2, P3: P0's enemies are P1 and P3
    // Distance from P0: P1 = 1, P3 = 3 (furthest)
    let target = 3; // P3 is the furthest enemy

    // Setup: P0 has Wood Lv3 and a card in hand
    game.players[caster].attributes.wood = 3;
    game.players[caster].hand.push(100); // Add card 100 to hand for discarding

    let initial_caster_hp = game.players[caster].hp;
    let initial_caster_shield = game.players[caster].shield;
    let initial_target_hp = game.players[target].hp;

    // Use wood attribute bolt (C2: 木彈 with IncreaseDamage(0))
    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Wood, None).unwrap(); // Use default targeting (furthest)

    // Expected:
    // - Caster loses 1 HP (Wood Lv3 cost)
    // - Caster gains 1 shield (Wood Lv3 cost)
    // - Furthest enemy takes 3 damage (wood level 3)
    assert_eq!(
        game.players[caster].hp,
        initial_caster_hp - 1,
        "Wood Lv3: Caster should lose 1 HP"
    );
    assert_eq!(
        game.players[caster].shield,
        initial_caster_shield + 1,
        "Wood Lv3: Caster should gain 1 shield"
    );
    assert_eq!(
        game.players[target].hp,
        initial_target_hp - 3,
        "Target should take 3 damage (Wood Lv3: attribute level)"
    );
}

/// Test Wood Lv3 basic effect from 50 HP
/// Verifies Wood Lv3 cost works correctly with standard starting HP
#[test]
fn test_wood_lv3_basic_spell_from_50hp() {
    let mut game = setup_test_game();
    let caster = 0;

    // Setup: P0 has Wood Lv3, standard starting HP, and a card in hand
    game.players[caster].attributes.wood = 3;
    game.players[caster].hand.push(100); // Add card to hand for discarding
    assert_eq!(game.players[caster].hp, 50, "Verify starting HP is 50");

    let initial_shield = game.players[caster].shield;

    // Use wood attribute bolt
    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Wood, None).unwrap();

    // Expected:
    // - HP: 50 - 1 = 49
    // - Shield: 0 + 1 = 1
    assert_eq!(
        game.players[caster].hp,
        49,
        "Wood Lv3: HP should be 50 - 1 = 49"
    );
    assert_eq!(
        game.players[caster].shield,
        initial_shield + 1,
        "Wood Lv3: Shield should increase by 1"
    );
}

/// Test Wood Lv3 HP cost is NOT affected by shield
/// Wood Lv3 cost is a direct HP loss, it bypasses shield entirely
#[test]
fn test_wood_lv3_hp_cost_not_affected_by_shield() {
    let mut game = setup_test_game();
    let caster = 0;

    // Setup: P0 has Wood Lv3, 10 shield, and a card in hand
    game.players[caster].attributes.wood = 3;
    game.players[caster].hand.push(100); // Add card to hand for discarding
    game.players[caster].hp = 50;
    game.players[caster].shield = 10;

    let initial_hp = game.players[caster].hp;
    let initial_shield = game.players[caster].shield;

    // Use wood attribute bolt
    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Wood, None).unwrap();

    // Expected:
    // - HP: 50 - 1 = 49 (cost is direct damage, unaffected by shield)
    // - Shield: 10 + 1 = 11 (both cost reduction and spell shield gain apply)
    assert_eq!(
        game.players[caster].hp,
        initial_hp - 1,
        "Wood Lv3 HP cost should NOT be affected by shield"
    );
    assert_eq!(
        game.players[caster].shield,
        initial_shield + 1,
        "Shield should still increase by 1 from Wood Lv3 effect"
    );
}
