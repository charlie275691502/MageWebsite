use mage_battle::attribute::AttributeType;
use mage_battle::character::CharacterType;
use mage_battle::game::{Game, TurnPhase};

#[test]
fn test_wood_lv3_proficiency_with_attribute_bolt() {
    // Test case: Mage A has 45 hp and 1 shield and lv 3 wood.
    // If it cast the wood spell again, it will become 44 hp and 2 shield
    let names = vec!["MageA".to_string(), "MageB".to_string(), "MageC".to_string(), "MageD".to_string()];
    let chars = vec![
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
    ];

    let mut game = Game::new(names, chars);

    // Setup: MageA (player 0) with 45 HP, 1 shield, wood lv 3
    game.players[0].hp = 45;
    game.players[0].shield = 1;
    game.players[0].attributes.wood = 3;

    // Set turn phase to PlayCard
    game.turn_phase = TurnPhase::PlayCard;

    // Give player 0 a card to use for attribute bolt
    let card_id = game.players[0].hand[0];

    // Use wood attribute bolt
    let result = game.play_attribute_bolt(card_id, AttributeType::Wood);

    // Verify the result
    assert!(result.is_ok(), "Wood attribute bolt should succeed: {:?}", result.err());

    // Check HP and shield after casting wood bolt
    // HP should be 44 (45 - 1, shield does NOT absorb the cost)
    // Shield should be 2 (1 + 1)
    assert_eq!(game.players[0].hp, 44, "HP should be 44 after wood lv3 cost");
    assert_eq!(game.players[0].shield, 2, "Shield should be 2 after wood lv3 effect");
}

#[test]
fn test_wood_lv3_basic_spell_from_50hp() {
    // Test case: Wood lv3, before 50HP, cast wood basic spell. After 49 HP 1 Shield.
    let names = vec!["MageA".to_string(), "MageB".to_string(), "MageC".to_string(), "MageD".to_string()];
    let chars = vec![
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
    ];

    let mut game = Game::new(names, chars);

    // Setup: MageA (player 0) with 50 HP, 0 shield, wood lv 3
    game.players[0].hp = 50;
    game.players[0].shield = 0;
    game.players[0].attributes.wood = 3;

    // Set turn phase to PlayCard
    game.turn_phase = TurnPhase::PlayCard;

    // Give player 0 a card to use for attribute bolt
    let card_id = game.players[0].hand[0];

    // Use wood attribute bolt (basic spell)
    let result = game.play_attribute_bolt(card_id, AttributeType::Wood);

    // Verify the result
    assert!(result.is_ok(), "Wood basic spell should succeed: {:?}", result.err());

    // Check HP and shield after casting wood basic spell
    // HP should be 49 (50 - 1, direct cost bypassing shield)
    // Shield should be 1 (0 + 1 from wood lv3 proficiency)
    assert_eq!(game.players[0].hp, 49, "HP should be 49 after wood lv3 cost");
    assert_eq!(game.players[0].shield, 1, "Shield should be 1 after wood lv3 effect");
}

#[test]
fn test_wood_lv3_hp_cost_not_affected_by_shield() {
    // Verify that wood lv3's HP cost bypasses shield entirely
    let names = vec!["MageA".to_string(), "MageB".to_string(), "MageC".to_string(), "MageD".to_string()];
    let chars = vec![
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
    ];

    let mut game = Game::new(names, chars);

    // Setup: MageA with 50 HP, 10 shield, wood lv 3
    game.players[0].hp = 50;
    game.players[0].shield = 10;
    game.players[0].attributes.wood = 3;

    // Set turn phase to PlayCard
    game.turn_phase = TurnPhase::PlayCard;

    let card_id = game.players[0].hand[0];

    // Use wood attribute bolt
    let result = game.play_attribute_bolt(card_id, AttributeType::Wood);
    assert!(result.is_ok(), "Wood attribute bolt failed: {:?}", result.err());

    // The HP cost should bypass shield entirely
    // HP: 50 -> 49 (direct HP reduction)
    // Shield: 10 -> 11 (gain 1 shield from proficiency)
    assert_eq!(game.players[0].hp, 49, "HP should decrease by 1 directly, bypassing shield");
    assert_eq!(game.players[0].shield, 11, "Shield should increase by 1 and remain otherwise intact");
}
