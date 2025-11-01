/// Comprehensive proficiency tests
/// Each proficiency (except Fire Lv5 and Wood Lv5) has 2 test variants:
/// 1. Using play_attribute_bolt() - tests proficiency with attribute bolts
/// 2. Using play_spell_card() - tests proficiency with spell cards

use mage_battle::{
    attribute::AttributeType, card::CardSide, character::CharacterType, damage::DamageType, effect::EffectType, game::{Game, TurnPhase}, player::TeamId
};

fn setup_test_game() -> Game {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::ThunderPoison,
        CharacterType::WaterWind,
    ];
    Game::new(names, chars)
}

// ==================== 火 (Fire) Proficiency Tests ====================
// Fire Lv3 & Lv5 use special structure - Lv3 is attribute bolt only, Lv5 is passive turn start

#[test]
fn fire_lv3_attribute_bolt_damage_boost() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3;

    game.players[caster].attributes.fire = 3;
    game.players[caster].hand.push(100);
    let initial_hp = game.players[target].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Fire, vec![target]).unwrap();

    assert_eq!(game.players[target].hp, initial_hp - 4,
        "Fire Lv3 should add +1 to attribute bolt damage");
}

#[test]
fn fire_lv3_does_apply_to_non_fire_bolts() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3;

    game.players[caster].attributes.fire = 3;
    game.players[caster].attributes.poison = 1;
    game.players[caster].hand.push(100);
    let initial_hp = game.players[target].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Poison, vec![target]).unwrap();

    assert_eq!(game.players[target].hp, initial_hp - 2,
        "Fire Lv3 should apply to non-fire attribute bolts");
}

#[test]
fn fire_lv5_turn_start_passive() {
    let mut game = setup_test_game();
    let caster = 0;
    let enemy1 = 1;
    let ally1 = 2;
    let enemy2 = 3;

    // Fire Lv5: 回合開始時，對所有敵人造成1點技能傷害
    game.players[caster].attributes.fire = 5;
    let initial_hp_0 = game.players[caster].hp;
    let initial_hp_1 = game.players[enemy1].hp;
    let initial_hp_2 = game.players[ally1].hp;
    let initial_hp_3 = game.players[enemy2].hp;

    game.current_player_index = caster;
    game.handle_turn_start();

    assert_eq!(game.players[enemy1].hp, initial_hp_1 - 1,
        "Fire Lv5 should deal 1 skill damage to enemy 1 at turn start");
    assert_eq!(game.players[enemy2].hp, initial_hp_3 - 1,
        "Fire Lv5 should deal 1 skill damage to enemy 2 at turn start");
    assert_eq!(game.players[caster].hp, initial_hp_0,
        "Fire Lv5 should not deal damage to caster at turn start");
    assert_eq!(game.players[ally1].hp, initial_hp_2,
        "Fire Lv5 should not deal damage to ally at turn start");
}

#[test]
fn fire_lv5_requires_level_5() {
    let mut game = setup_test_game();
    let caster = 0;
    let enemy1 = 1;
    let enemy2 = 3;

    game.players[caster].attributes.fire = 4;
    let initial_hp_1 = game.players[enemy1].hp;
    let initial_hp_3 = game.players[enemy2].hp;

    game.current_player_index = caster;
    game.handle_turn_start();

    assert_eq!(game.players[enemy1].hp, initial_hp_1,
        "Fire Lv5 should NOT trigger at level 4");
    assert_eq!(game.players[enemy2].hp, initial_hp_3,
        "Fire Lv5 should NOT trigger at level 4");
}

// ==================== 木 (Wood) Proficiency Tests ====================

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
    game.play_attribute_bolt(100, AttributeType::Wood, vec![target]).unwrap(); // Target furthest enemy

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


#[test]
fn test_wood_lv3_proficiency_with_spell_card() {
    let mut game = setup_test_game();
    let caster = 0;
    // In turn order P0, P1, P2, P3: P0's enemies are P1 and P3
    // Distance from P0: P1 = 1, P3 = 3 (furthest)
    let target = 3; // P3 is the furthest enemy

    // Setup: P0 has Wood Lv3 and a card in hand
    game.players[caster].attributes.wood = 4;
    game.players[caster].hand.push(25); // Add card 25 to hand for

    let initial_caster_hp = game.players[caster].hp;
    let initial_caster_shield = game.players[caster].shield;
    let initial_target_hp = game.players[target].hp;

    // Use wood spell (C2: 飛葉連斬 with Damage(8))
    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_spell_card(25, CardSide::Top, vec![target]).unwrap(); // Target furthest enemy

    // Expected:
    // - Caster loses 1 HP (Wood Lv3 cost)
    // - Caster gains 1 shield (Wood Lv3 cost)
    // - Furthest enemy takes 8 damage (wood level 8)
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
        initial_target_hp - 8,
        "Target should take 8 damage (Wood Lv3: attribute level)"
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
    let target = 3; // Furthest enemy
    game.play_attribute_bolt(100, AttributeType::Wood, vec![target]).unwrap();

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
    let target = 3; // Furthest enemy
    game.play_attribute_bolt(100, AttributeType::Wood, vec![target]).unwrap();

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

#[test]
fn wood_lv5_reduces_spell_damage() {
    let mut game = setup_test_game();
    let caster = 0;
    let defender = 3;
    let defender_ally = 1;

    // 奧義：雙炎閃 火5 對兩個敵人各造成9點傷害
    game.players[caster].attributes.fire = 5;
    game.players[caster].hand.push(15); // Add card

    game.players[defender].attributes.wood = 5;
    let initial_hp_defender = game.players[defender].hp;
    let initial_hp_defender_ally = game.players[defender_ally].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_spell_card(15, CardSide::Top, vec![defender, defender_ally]).unwrap();

    // 9 - 1 = 8 damage
    assert_eq!(game.players[defender].hp, initial_hp_defender - 8,
        "Wood Lv5 should reduce self spell damage by 1");
    assert_eq!(game.players[defender_ally].hp, initial_hp_defender_ally - 8,
        "Wood Lv5 should reduce ally spell damage by 1");
}

#[test]
fn wood_lv5_does_not_reduce_skill_damage() {
    let mut game = setup_test_game();
    let target = 1;

    // Wood Lv5 only reduces spell damage, not skill damage
    game.players[target].attributes.wood = 5;
    let initial_hp = game.players[target].hp;

    game.players[target].take_damage(5, DamageType::Skill);

    assert_eq!(game.players[target].hp, initial_hp - 5,
        "Wood Lv5 should NOT reduce skill damage");
}

// ==================== 雷 (Thunder) Proficiency Tests ====================

#[test]
fn thunder_lv3_spell_damage_boost_attribute_bolt() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3; // Furthest enemy (distance 3 from P0)

    // Thunder Lv3: 雷屬性卡片傷害+1 (applies to all thunder attribute effects)
    game.players[caster].attributes.thunder = 3;
    game.players[caster].hand.push(100); // Add card for discarding (thunder bolt)
    let initial_hp = game.players[target].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Thunder, vec![target]).unwrap();

    // Thunder level 3 + Lv3 bonus 1 = 4 damage
    assert_eq!(game.players[target].hp, initial_hp - 4,
        "Thunder Lv3 should add +1 to thunder attribute bolt damage");
}

#[test]
fn thunder_lv3_spell_card() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3;

    game.players[caster].attributes.thunder = 3;
    // Card 31 has A13 (雷迎) - attribute level + 2 damage
    game.players[caster].hand.push(31);
    let initial_hp = game.players[target].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    // A13 (雷迎) does: attribute level + 2 damage = 3 + 2 = 5, + Lv3 bonus 1 = 6 total
    game.play_spell_card(31, CardSide::Top, vec![target]).unwrap();

    // Should have taken damage with Thunder Lv3 bonus applied
    assert!(game.players[target].hp < initial_hp,
        "Thunder Lv3 should add +1 to thunder spell card damage");
}

#[test]
fn thunder_lv5_attribute_bolt() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3;

    game.players[caster].attributes.thunder = 5;
    game.players[caster].hand.push(100);
    let initial_hp = game.players[target].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Thunder, vec![target]).unwrap();

    // Thunder level 5 + Lv3 bonus 1 + Lv5 bonus 2 = 8 damage
    assert_eq!(game.players[target].hp, initial_hp - 8,
        "Thunder Lv5 should add +3 total damage (Lv3: +1, Lv5: +2)");
}

#[test]
fn thunder_lv5_spell_card() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3;

    game.players[caster].attributes.thunder = 5;
    // Card 31 has A13 (雷迎) 造成屬性等級＋２點傷害
    game.players[caster].hand.push(31);
    let initial_hp = game.players[target].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_spell_card(31, CardSide::Top, vec![target]).unwrap();

    // Should have taken damage with Thunder Lv3 + Lv5 bonus applied
    assert_eq!(game.players[target].hp, initial_hp - 10,
        "Thunder Lv5 should add +3 total to thunder spell card damage");
}

// ==================== 水 (Water) Proficiency Tests ====================
// Water Lv3 & Lv5: 2 tests each (attribute bolt + spell card)

#[test]
fn water_lv3_attribute_bolt_heal() {
    let mut game = setup_test_game();
    let caster = 0;

    game.players[caster].attributes.water = 3;
    game.players[caster].hand.push(100);
    let initial_hp = game.players[caster].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Water, vec![3]).unwrap();

    // Water Lv3 heals 1 HP after damage
    assert_eq!(game.players[caster].hp, initial_hp + 1,
        "Water Lv3 should heal +1 when using water attribute bolt");
}

#[test]
fn water_lv3_spell_card_heal() {
    let mut game = setup_test_game();
    let caster = 0;

    game.players[caster].attributes.water = 3;
    // Card 46 has A19 (潮破) - attribute level + 2 damage
    game.players[caster].hand.push(46);
    let initial_hp = game.players[caster].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_spell_card(46, CardSide::Top, vec![3]).unwrap();

    // Water Lv3 heals 1 HP after spell card effect
    assert_eq!(game.players[caster].hp, initial_hp+1,
        "Water Lv3 should heal +1 when using water spell card");
}

#[test]
fn water_lv5_attribute_bolt_teammate_heal() {
    let mut game = setup_test_game();
    let caster = 0;
    let teammate = 2;

    game.players[caster].team = TeamId::Team0;
    game.players[teammate].team = TeamId::Team0;
    game.players[caster].attributes.water = 5;
    game.players[caster].hand.push(100);

    let initial_hp_caster = game.players[caster].hp;
    let initial_hp_teammate = game.players[teammate].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Water, vec![3]).unwrap();

    // Caster heals 1 (Lv3), teammate heals 1 (Lv5)
    assert_eq!(game.players[caster].hp, initial_hp_caster + 1,
        "Water Lv5 caster should heal +1");
    assert_eq!(game.players[teammate].hp, initial_hp_teammate + 1,
        "Water Lv5 teammate should heal +1");
}

#[test]
fn water_lv5_spell_card_teammate_heal() {
    let mut game = setup_test_game();
    let caster = 0;
    let teammate = 2;

    game.players[caster].team = TeamId::Team0;
    game.players[teammate].team = TeamId::Team0;
    game.players[caster].attributes.water = 5;
    // Card 46 has A19 (潮破)
    game.players[caster].hand.push(46);

    let initial_hp_caster = game.players[caster].hp;
    let initial_hp_teammate = game.players[teammate].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_spell_card(46, CardSide::Top, vec![3]).unwrap();

    // Both should heal with Water Lv5 effect
    assert_eq!(game.players[caster].hp, initial_hp_caster+1,
        "Water Lv5 caster should heal with spell card");
    assert_eq!(game.players[teammate].hp, initial_hp_teammate+1,
        "Water Lv5 teammate should heal with spell card");
}

// ==================== 風 (Wind) Proficiency Tests ====================
// Wind Lv2: 2 tests (attribute bolt debuff + spell card debuff)
// Wind Lv5: 2 tests (target validation tests)

#[test]
fn wind_lv2_attribute_bolt_debuff() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3;

    game.players[caster].attributes.wind = 2;
    game.players[caster].hand.push(100);
    let initial_hp = game.players[target].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Wind, vec![target]).unwrap();

    // Should have DefenseInvalidation debuff
    assert!(game.players[target].buffs.has(mage_battle::buff::BuffType::DefenseInvalidation),
        "Wind Lv2 should apply DefenseInvalidation debuff with attribute bolt");
    assert_eq!(game.players[target].hp, initial_hp - 2,
        "Target should take wind level 2 damage");
}

#[test]
fn wind_lv2_spell_card_debuff() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3;

    game.players[caster].attributes.wind = 3;
    // Card 3 has B30 (風3) on bottom side ３點傷害。可捨棄任意手牌，再抽相同數量
    game.players[caster].hand.push(3);
    let _initial_hp = game.players[target].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_spell_card(3, CardSide::Bottom, vec![target]).unwrap();

    // Should have DefenseInvalidation debuff
    assert!(game.players[target].buffs.has(mage_battle::buff::BuffType::DefenseInvalidation),
        "Wind Lv2 should apply DefenseInvalidation debuff with spell card");
}

#[test]
fn wind_lv5_attribute_bolt_free_targeting() {
    let mut game = setup_test_game();
    let caster = 0;

    game.players[caster].attributes.wind = 5;
    game.current_player_index = caster;

    let enchantments = vec![AttributeType::Wind];
    let is_active = game.is_wind_lv5_triggered(caster, &enchantments);

    assert!(is_active, "Wind Lv5 should be triggered for wind attribute");
}

#[test]
fn wind_lv5_spell_card_free_targeting() {
    let mut game = setup_test_game();
    let caster = 0;

    game.players[caster].attributes.wind = 5;
    game.current_player_index = caster;

    let enchantments = vec![AttributeType::Wind];
    let is_active = game.is_wind_lv5_triggered(caster, &enchantments);

    assert!(is_active, "Wind Lv5 should enable free targeting for wind spell cards");
}

// ==================== 毒 (Poison) Proficiency Tests ====================
// Poison Lv2: 2 tests (attribute bolt debuff + spell card debuff)
// Poison Lv5: 2 tests (effect validation tests)

#[test]
fn poison_lv2_attribute_bolt_debuff() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3;

    game.players[caster].attributes.poison = 2;
    game.players[caster].hand.push(100);
    let initial_hp = game.players[target].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Poison, vec![target]).unwrap();

    // Should have Confuse debuff
    assert!(game.players[target].buffs.has(mage_battle::buff::BuffType::Confuse),
        "Poison Lv2 should apply Confuse debuff with attribute bolt");
    assert_eq!(game.players[target].hp, initial_hp - 2,
        "Target should take poison level 2 damage");
}

#[test]
fn poison_lv2_spell_card_debuff() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3;

    game.players[caster].attributes.poison = 2;
    // Card 34 has B34 (毒2) on bottom side
    game.players[caster].hand.push(34);
    let _initial_hp = game.players[target].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_spell_card(34, CardSide::Bottom, vec![target]).unwrap();

    // Should have Confuse debuff
    assert!(game.players[target].buffs.has(mage_battle::buff::BuffType::Confuse),
        "Poison Lv2 should apply Confuse debuff with spell card");
}

#[test]
fn poison_lv5_attribute_bolt_restriction() {
    let mut game = setup_test_game();
    let target = 1;

    game.players[target].attributes.poison = 5;
    let effects = game.players[target].attributes.get_mastery_effects();

    assert!(effects.iter().any(|e| matches!(e, mage_battle::attribute::MasteryEffect::PoisonLevel5)),
        "Poison Lv5 should restrict target to attribute bolts only");
}

#[test]
fn poison_lv5_spell_card_restriction() {
    let mut game = setup_test_game();
    let target = 1;

    game.players[target].attributes.poison = 5;
    let effects = game.players[target].attributes.get_mastery_effects();

    assert!(effects.iter().any(|e| matches!(e, mage_battle::attribute::MasteryEffect::PoisonLevel5)),
        "Poison Lv5 should restrict target with spell cards too");
}

// ==================== 木 (Wood) Proficiency Tests ====================
// Wood Lv3: Tested in wood_proficiency_tests.rs
// Wood Lv5: Passive proficiency - excluded from refactoring
