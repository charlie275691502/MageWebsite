/// Comprehensive spell tests for all spells in the game
/// Tests spells using the correct card IDs from cards.json
///
/// Card-to-Spell Mapping:
/// - Card IDs 1-60 each have a top spell, and many have a bottom spell
/// - Tests verify damage, healing, shields, buffs, and proficiency interactions

use mage_battle::{
    buff::BuffType,
    card::CardSide,
    character::CharacterType,
    game::{Game, TurnPhase},
};

fn create_test_game() -> Game {
    let names = vec![
        "P0".to_string(),
        "P1".to_string(),
        "P2".to_string(),
        "P3".to_string(),
    ];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::ThunderPoison,
        CharacterType::WaterWind,
    ];
    Game::new(names, chars)
}

// ============================================
// A-Series Basic Elemental Spells - Fire
// ============================================

#[test]
fn test_a1_flame_flare() {
    let mut game = create_test_game();

    // Card 1 has A1 on top: 火焰迸發 (Fire 3) - attribute level + 2 damage
    game.players[0].attributes.fire = 3;
    game.players[0].hand.push(1);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(1, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A1 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 5, "A1 with Fire 3 should deal 3+2=5 damage");
}

#[test]
fn test_a2_fireball() {
    let mut game = create_test_game();

    // Card 3 has A2 on top: 火球 (Fire 2) - 5 damage
    game.players[0].attributes.fire = 2;
    game.players[0].hand.push(3);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(3, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A2 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 5, "A2 should deal 5 damage");
}

#[test]
fn test_a3_large_fireball() {
    let mut game = create_test_game();

    // Card 6 has A3 on top: 大火球 (Fire 3) - 7 damage
    game.players[0].attributes.fire = 3;
    game.players[0].hand.push(6);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(6, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A3 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 7, "A3 should deal 7 damage");
}

#[test]
fn test_a4_blazing_bomb() {
    let mut game = create_test_game();

    // Card 10 has A4 on top: 炙炎爆彈 (Fire 4) - 8 damage
    game.players[0].attributes.fire = 4;
    game.players[0].hand.push(10);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(10, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A4 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 8, "A4 should deal 8 damage");
}

#[test]
fn test_a5_ultimate_fire_pillar() {
    let mut game = create_test_game();

    // Card 14 has A5 on top: 奧義：火柱 (Fire 5) - 12 damage
    game.players[0].attributes.fire = 5;
    game.players[0].hand.push(14);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(14, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A5 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 12, "A5 should deal 12 damage");
}

#[test]
fn test_a6_ultimate_twin_flame() {
    let mut game = create_test_game();

    // Card 15 has A6 on top: 奧義：雙炎閃 (Fire 5) - 9 damage to 2 targets
    game.players[0].attributes.fire = 5;
    game.players[0].hand.push(15);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp1 = game.players[1].hp;
    let initial_hp2 = game.players[3].hp;
    let result = game.play_spell_card(15, CardSide::Top, vec![1, 3]);

    assert!(result.is_ok(), "A6 spell should succeed: {:?}", result);
    assert_eq!(game.players[1].hp, initial_hp1 - 9, "A6 should deal 9 damage to target 1");
    assert_eq!(game.players[3].hp, initial_hp2 - 9, "A6 should deal 9 damage to target 2");
}

// ============================================
// A-Series Basic Elemental Spells - Wood
// ============================================

#[test]
fn test_a7_wood_burial() {
    let mut game = create_test_game();

    // Card 16 has A7 on top: 木葬 (Wood 3) - attribute level + 2 damage
    game.players[0].attributes.wood = 3;
    game.players[0].hand.push(16);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(16, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A7 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 5, "A7 with Wood 3 should deal 3+2=5 damage");
}

#[test]
fn test_a8_leaf_slash() {
    let mut game = create_test_game();

    // Card 18 has A8 on top: 葉斬 (Wood 2) - 5 damage
    game.players[0].attributes.wood = 2;
    game.players[0].hand.push(18);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(18, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A8 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 5, "A8 should deal 5 damage");
}

#[test]
fn test_a9_flying_leaf_slash() {
    let mut game = create_test_game();

    // Card 21 has A9 on top: 飛葉斬 (Wood 3) - 7 damage
    game.players[0].attributes.wood = 3;
    game.players[0].hand.push(21);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(21, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A9 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 7, "A9 should deal 7 damage");
}

#[test]
fn test_a10_continuous_leaf_slash() {
    let mut game = create_test_game();

    // Card 25 has A10 on top: 飛葉連斬 (Wood 4) - 8 damage
    game.players[0].attributes.wood = 4;
    game.players[0].hand.push(25);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(25, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A10 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 8, "A10 should deal 8 damage");
}

#[test]
fn test_a11_ultimate_wither_vine() {
    let mut game = create_test_game();

    // Card 29 has A11 on top: 奧義：枯藤 (Wood 5) - 12 damage
    game.players[0].attributes.wood = 5;
    game.players[0].hand.push(29);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(29, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A11 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 12, "A11 should deal 12 damage");
}

#[test]
fn test_a12_ultimate_ancient_tree() {
    let mut game = create_test_game();

    // Card 30 has A12 on top: 奧義：老樹 (Wood 5) - Grants Immune for 2 turns to self
    game.players[0].attributes.wood = 5;
    game.players[0].hand.push(30);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    // Self_ target_pool still requires providing self as target
    let result = game.play_spell_card(30, CardSide::Top, vec![0]);

    assert!(result.is_ok(), "A12 spell should succeed: {:?}", result);
    assert!(game.players[0].buffs.has(BuffType::Immune), "A12 should grant Immune buff to self");
}

// ============================================
// A-Series Basic Elemental Spells - Thunder
// ============================================

#[test]
fn test_a13_thunder_welcome() {
    let mut game = create_test_game();

    // Card 31 has A13 on top: 雷迎 (Thunder 3) - attribute level + 2 damage
    // Thunder Lv3 gives +1 proficiency bonus: (3+2) + 1 = 6 damage
    game.players[0].attributes.thunder = 3;
    game.players[0].hand.push(31);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(31, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A13 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 6, "A13 with Thunder 3 should deal (3+2)+1=6 damage including proficiency");
}

#[test]
fn test_a14_thunder_spear() {
    let mut game = create_test_game();

    // Card 33 has A14 on top: 雷槍 (Thunder 2) - 5 damage
    game.players[0].attributes.thunder = 2;
    game.players[0].hand.push(33);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(33, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A14 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 5, "A14 should deal 5 damage");
}

#[test]
fn test_a15_thunder_spear_arrow() {
    let mut game = create_test_game();

    // Card 36 has A15 on top: 雷矛箭 (Thunder 3) - 7 damage
    // Thunder Lv3 gives +1 proficiency bonus: 7 + 1 = 8 damage
    game.players[0].attributes.thunder = 3;
    game.players[0].hand.push(36);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(36, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A15 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 8, "A15 should deal 7+1=8 damage including proficiency");
}

#[test]
fn test_a16_lightning_strike() {
    let mut game = create_test_game();

    // Card 40 has A16 on top: 閃電雷擊 (Thunder 4) - 8 damage
    // Thunder Lv4 gives +1 proficiency bonus (Lv3): 8 + 1 = 9 damage
    game.players[0].attributes.thunder = 4;
    game.players[0].hand.push(40);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(40, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A16 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 9, "A16 should deal 8+1=9 damage including proficiency");
}

#[test]
fn test_a17_ultimate_focused_shot() {
    let mut game = create_test_game();

    // Card 44 has A17 on top: 奧義：聚能射擊 (Thunder 5) - 12 damage
    // Thunder Lv5 gives +3 proficiency bonus: 12 + 3 = 15 damage
    game.players[0].attributes.thunder = 5;
    game.players[0].hand.push(44);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(44, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A17 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 15, "A17 should deal 12+3=15 damage including proficiency");
}

#[test]
fn test_a18_ultimate_flicker_shot() {
    let mut game = create_test_game();

    // Card 45 has A18 on top: 奧義：閃爍射擊 (Thunder 5) - TripleDamage(3)
    // Thunder Lv5 gives +3 proficiency bonus per hit: (3+3) x 3 = 18 total damage
    game.players[0].attributes.thunder = 5;
    game.players[0].hand.push(45);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(45, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A18 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 18, "A18 should deal (3+3)x3=18 total damage including proficiency");
}

// ============================================
// A-Series Basic Elemental Spells - Water
// ============================================

#[test]
fn test_a19_tide_break() {
    let mut game = create_test_game();

    // Card 46 has A19 on top: 潮破 (Water 3) - attribute level + 2 damage
    game.players[0].attributes.water = 3;
    game.players[0].hand.push(46);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(46, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A19 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 5, "A19 with Water 3 should deal 3+2=5 damage");
}

#[test]
fn test_a20_water_bullet() {
    let mut game = create_test_game();

    // Card 48 has A20 on top: 水彈 (Water 2) - 5 damage
    game.players[0].attributes.water = 2;
    game.players[0].hand.push(48);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(48, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A20 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 5, "A20 should deal 5 damage");
}

#[test]
fn test_a21_condensed_water() {
    let mut game = create_test_game();

    // Card 51 has A21 on top: 凝水彈 (Water 3) - 7 damage
    game.players[0].attributes.water = 3;
    game.players[0].hand.push(51);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(51, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A21 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 7, "A21 should deal 7 damage");
}

#[test]
fn test_a22_water_pillar_impact() {
    let mut game = create_test_game();

    // Card 55 has A22 on top: 水柱衝擊 (Water 4) - 8 damage
    game.players[0].attributes.water = 4;
    game.players[0].hand.push(55);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(55, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A22 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 8, "A22 should deal 8 damage");
}

#[test]
fn test_a23_ultimate_waterfall() {
    let mut game = create_test_game();

    // Card 59 has A23 on top: 奧義：水瀑 (Water 5) - 12 damage
    game.players[0].attributes.water = 5;
    game.players[0].hand.push(59);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(59, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A23 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 12, "A23 should deal 12 damage");
}

#[test]
fn test_a24_ultimate_water_curtain() {
    let mut game = create_test_game();

    // Card 60 has A24 on top: 奧義：水濂 (Water 5) - Heals 15 HP to one ally
    game.players[0].attributes.water = 5;
    game.players[0].hand.push(60);
    game.players[1].hp = 30; // Reduce teammate HP to test healing
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(60, CardSide::Top, vec![1]);

    assert!(result.is_ok(), "A24 spell should succeed: {:?}", result);
    assert_eq!(game.players[1].hp, initial_hp + 15, "A24 should heal 15 HP");
}

// ============================================
// B-Series Combination Spells - Key Tests
// ============================================

#[test]
fn test_b1_mixed_fuel() {
    let mut game = create_test_game();

    // B1: 混和燃料 (Fire 4, Wood 2) - 9 damage
    // Need to find which card has B1 on bottom - card 37 has B1
    game.players[0].attributes.fire = 4;
    game.players[0].attributes.wood = 2;
    game.players[0].hand.push(37);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(37, CardSide::Bottom, vec![1]);

    assert!(result.is_ok(), "B1 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 9, "B1 should deal 9 damage");
}

#[test]
fn test_b10_wood_wall() {
    let mut game = create_test_game();

    // B10: 木牆術 (Wood 4, Water 2) - 10 shield to one ally
    // Card 7 has B10 on bottom
    game.players[0].attributes.wood = 4;
    game.players[0].attributes.water = 2;
    game.players[0].hand.push(7);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_shield = game.players[1].shield;
    let result = game.play_spell_card(7, CardSide::Bottom, vec![1]);

    assert!(result.is_ok(), "B10 spell should succeed: {:?}", result);
    assert_eq!(game.players[1].shield, initial_shield + 10, "B10 should grant 10 shield to target");
}

#[test]
fn test_b15_spark() {
    let mut game = create_test_game();

    // B15: 火花 (Thunder 4, Fire 2) - DoubleDamage(1) to two enemies
    // Thunder Lv4 gives +1 proficiency bonus per hit: (1+1) x 2 = 4 damage per target
    // Card 21 has B15 on bottom
    game.players[0].attributes.thunder = 4;
    game.players[0].attributes.fire = 2;
    game.players[0].hand.push(21);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp1 = game.players[1].hp;
    let initial_hp2 = game.players[3].hp;
    let result = game.play_spell_card(21, CardSide::Bottom, vec![1, 3]);

    assert!(result.is_ok(), "B15 spell should succeed: {:?}", result);
    let damage1 = initial_hp1 - game.players[1].hp;
    let damage2 = initial_hp2 - game.players[3].hp;
    assert_eq!(damage1, 4, "B15 should deal (1+1)x2=4 damage to enemy 1 with proficiency");
    assert_eq!(damage2, 4, "B15 should deal (1+1)x2=4 damage to enemy 2 with proficiency");
}

#[test]
fn test_b23_bandage() {
    let mut game = create_test_game();

    // B23: 包紮 (Water 4, Wood 2) - 3 shield + 3 heal to two allies
    // Water Lv3 (included in Lv4) heals caster +1 HP when using water spells
    // Card 35 has B23 on bottom
    game.players[0].attributes.water = 4;
    game.players[0].attributes.wood = 2;
    game.players[0].hand.push(35);
    game.players[0].hp = 40; // Reduce HP to test
    game.players[1].hp = 40;
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_shield0 = game.players[0].shield;
    let initial_shield1 = game.players[1].shield;
    let initial_hp0 = game.players[0].hp;
    let initial_hp1 = game.players[1].hp;
    let result = game.play_spell_card(35, CardSide::Bottom, vec![0, 1]);

    assert!(result.is_ok(), "B23 spell should succeed: {:?}", result);
    assert_eq!(game.players[0].shield, initial_shield0 + 3, "B23 should grant 3 shield to ally 1");
    assert_eq!(game.players[1].shield, initial_shield1 + 3, "B23 should grant 3 shield to ally 2");
    // Water Lv3 proficiency heals caster +1 HP per heal effect, so P0 gets 3+2=5 (spell heal + 2 profs), P1 gets 3
    assert_eq!(game.players[0].hp, initial_hp0 + 5, "B23 should heal 3+2=5 HP to caster (includes Water Lv3 proficiency bonus)");
    assert_eq!(game.players[1].hp, initial_hp1 + 3, "B23 should heal 3 HP to ally 2");
}

#[test]
fn test_b30_wind_basic() {
    let mut game = create_test_game();

    // Card 3 has B30 on bottom: 風3 (Wind 3) - 3 damage, discard and redraw
    game.players[0].attributes.wind = 3;
    game.players[0].hand.push(3);
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let initial_hp = game.players[1].hp;
    let result = game.play_spell_card(3, CardSide::Bottom, vec![1]);

    assert!(result.is_ok(), "B30 spell should succeed: {:?}", result);
    let damage = initial_hp - game.players[1].hp;
    assert_eq!(damage, 3, "B30 should deal 3 damage");
}

// ============================================
// Proficiency Interaction Tests
// Note: Proficiency bonuses apply differently to spell cards vs attribute bolts
// These tests verify the proficiency system works with spell cards
// ============================================

#[test]
fn test_wind_spell_with_wind_lv2_proficiency() {
    let mut game = create_test_game();

    // Wind Lv2: Applies Defense Invalidation to targets when using wind spells
    game.players[0].attributes.wind = 3; // Wind 3 includes Lv2 bonus
    game.players[0].hand.push(3); // Card 3: B30 (風3) on bottom - 3 damage
    game.current_player_index = 0;
    game.turn_phase = TurnPhase::PlayCard;

    let result = game.play_spell_card(3, CardSide::Bottom, vec![1]);

    assert!(result.is_ok(), "Wind spell should succeed: {:?}", result);
    assert!(
        game.players[1].buffs.has(BuffType::DefenseInvalidation),
        "Wind Lv2 should apply Defense Invalidation to target"
    );
}
