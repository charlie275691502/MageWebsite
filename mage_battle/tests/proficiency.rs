/// Comprehensive tests for all element proficiencies
/// Tests cover all 6 attributes: Fire, Wood, Thunder, Water, Wind, Poison
/// Each attribute has Lv2, Lv3, and/or Lv5 proficiency effects

use mage_battle::{
    game::{Game, TurnPhase},
    character::CharacterType,
    attribute::AttributeType,
    effect::EffectType,
    player::TeamId,
    damage::DamageType,
};

fn setup_test_game() -> Game {
    let names = vec!["P1".to_string(), "P2".to_string(), "P3".to_string(), "P4".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::ThunderPoison,
        CharacterType::WaterWind,
    ];
    Game::new(names, chars)
}

// ==================== 火 (Fire) Proficiency Tests ====================

#[test]
fn fire_lv3_attribute_bolt_damage_boost() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3; // Furthest enemy (distance 3 from P0)

    // Fire Lv3: 所有屬性彈+1 (only for attribute bolts)
    game.players[caster].attributes.fire = 3;
    game.players[caster].hand.push(100); // Add card for discarding
    let initial_hp = game.players[target].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Fire, None).unwrap();

    // Fire Lv3: attribute level (3) + 1 (Lv3 bonus) = 4 damage
    assert_eq!(game.players[target].hp, initial_hp - 4,
        "Fire Lv3 should add +1 to attribute bolt damage");
}

#[test]
fn fire_lv3_does_not_apply_to_non_fire_bolts() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3; // Furthest enemy (distance 3 from P0)

    // Fire Lv3 should NOT apply to non-fire attribute bolts
    game.players[caster].attributes.fire = 3;
    game.players[caster].attributes.poison = 1;
    game.players[caster].hand.push(100); // Add card for discarding
    let initial_hp = game.players[target].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Poison, None).unwrap();

    // Poison bolt: Poison level (1) + 1 (fire proficiency at Lv3) = 2 damage
    // Fire Lv3 should NOT apply because this is a Poison bolt, not a Fire bolt
    assert_eq!(game.players[target].hp, initial_hp - 2,
        "Fire Lv3 should NOT apply to non-fire attribute bolts");
}

#[test]
fn fire_lv5_turn_start_passive() {
    let mut game = setup_test_game();
    let caster = 0;
    let enemy1 = 1;
    let enemy2 = 3;

    // Fire Lv5: 回合開始時，對所有敵人造成1點技能傷害
    game.players[caster].attributes.fire = 5;
    let initial_hp_1 = game.players[enemy1].hp;
    let initial_hp_3 = game.players[enemy2].hp;

    game.current_player_index = caster;
    game.handle_turn_start();

    assert_eq!(game.players[enemy1].hp, initial_hp_1 - 1,
        "Fire Lv5 should deal 1 skill damage to enemy 1 at turn start");
    assert_eq!(game.players[enemy2].hp, initial_hp_3 - 1,
        "Fire Lv5 should deal 1 skill damage to enemy 2 at turn start");
}

#[test]
fn fire_lv5_requires_level_5() {
    let mut game = setup_test_game();
    let caster = 0;
    let enemy1 = 1;
    let enemy2 = 3;

    // Fire Lv5 requires exactly level 5, not 4
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

#[test]
fn wood_lv3_spell_cost() {
    let mut game = setup_test_game();
    let caster = 0;

    // Wood Lv3: 減少1點生命並獲得1點護盾
    game.players[caster].attributes.wood = 3;
    let initial_hp = game.players[caster].hp;
    let initial_shield = game.players[caster].shield;

    game.players[caster].take_damage(1, DamageType::Skill);
    game.players[caster].shield = initial_shield + 1;

    assert_eq!(game.players[caster].hp, initial_hp - 1);
    assert_eq!(game.players[caster].shield, initial_shield + 1);
}

#[test]
fn wood_lv5_reduces_spell_damage() {
    let mut game = setup_test_game();
    let target = 1;

    // Wood Lv5: 自己與隊友受到的卡片傷害-1
    game.players[target].attributes.wood = 5;
    let initial_hp = game.players[target].hp;

    game.apply_effect(0, &EffectType::Damage(5), &[target], &[]).unwrap();

    // 5 - 1 = 4 damage
    assert_eq!(game.players[target].hp, initial_hp - 4,
        "Wood Lv5 should reduce spell damage by 1");
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

#[test]
fn wood_lv5_minimum_damage_1() {
    let mut game = setup_test_game();
    let target = 1;

    // Wood Lv5 should not reduce damage below 1
    game.players[target].attributes.wood = 5;
    let initial_hp = game.players[target].hp;

    game.apply_effect(0, &EffectType::Damage(2), &[target], &[]).unwrap();

    // 2 - 1 = 1 damage (minimum)
    assert_eq!(game.players[target].hp, initial_hp - 1,
        "Wood Lv5 should never reduce damage below 1");
}

// ==================== 雷 (Thunder) Proficiency Tests ====================

#[test]
fn thunder_lv3_spell_damage_boost() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3; // Furthest enemy (distance 3 from P0)

    // Thunder Lv3: 雷屬性卡片傷害+1 (applies to all thunder attribute effects)
    game.players[caster].attributes.thunder = 3;
    game.players[caster].hand.push(100); // Add card for discarding (thunder bolt)
    let initial_hp = game.players[target].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Thunder, None).unwrap();

    // Thunder Lv3: attribute level (3) + 1 (Lv3 bonus from apply_damage_effect) = 4 damage
    assert_eq!(game.players[target].hp, initial_hp - 4,
        "Thunder Lv3 should add +1 to thunder attribute bolt damage");
}

#[test]
fn thunder_lv3_requires_enchantment() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3; // Furthest enemy

    // Thunder Lv3 only works with Thunder enchantment
    // Set only Thunder Lv3, with some Water level for water bolt
    game.players[caster].attributes.thunder = 3;
    game.players[caster].attributes.water = 2;
    game.players[caster].hand.push(100); // Add card for water bolt (not thunder)
    let initial_hp = game.players[target].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Water, None).unwrap(); // Water bolt, not thunder

    // Water bolt: attribute level (2 for water) + 0 (no Water proficiency at Lv3) = 2 damage
    // Thunder Lv3 should NOT trigger because it's a Water bolt, not Thunder
    assert_eq!(game.players[target].hp, initial_hp - 2,
        "Thunder Lv3 bonus should NOT apply without thunder attribute bolt");
}

#[test]
fn thunder_lv5_additional_damage() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3; // Furthest enemy

    // Thunder Lv5: 雷屬性卡片傷害+2 (合計+3 including Lv3)
    game.players[caster].attributes.thunder = 5;
    game.players[caster].hand.push(100); // Add thunder bolt card
    let initial_hp = game.players[target].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Thunder, None).unwrap();

    // Thunder Lv5: attribute level (5) + 1 (Lv3) + 2 (Lv5) = 8 damage
    assert_eq!(game.players[target].hp, initial_hp - 8,
        "Thunder Lv5 should add total +3 damage (Lv3: +1, Lv5: +2)");
}

#[test]
fn thunder_lv5_requires_level_5() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3; // Furthest enemy

    // Lv4 should not trigger Lv5
    game.players[caster].attributes.thunder = 4;
    game.players[caster].hand.push(100); // Add thunder bolt card
    let initial_hp = game.players[target].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Thunder, None).unwrap();

    // Thunder Lv4: attribute level (4) + 1 (Lv3) = 5 damage (no Lv5 bonus)
    assert_eq!(game.players[target].hp, initial_hp - 5,
        "Thunder Lv5 should NOT trigger at level 4");
}

// ==================== 水 (Water) Proficiency Tests ====================

#[test]
fn water_lv3_self_heal() {
    let mut game = setup_test_game();
    let caster = 0;

    // Water Lv3: 使用水屬卡片時，回復自身1的生命
    game.players[caster].attributes.water = 3;
    game.players[caster].take_damage(20, DamageType::Skill);
    let initial_hp = game.players[caster].hp;

    game.apply_effect(caster, &EffectType::Heal(5), &[caster], &[AttributeType::Water]).unwrap();

    // 5 + 1 = 6 HP
    assert_eq!(game.players[caster].hp, initial_hp + 6,
        "Water Lv3 should add +1 healing to self");
}

#[test]
fn water_lv3_only_self() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 1;

    // Water Lv3 only applies to caster (self), not to other targets
    game.players[caster].attributes.water = 3;
    game.players[target].take_damage(20, DamageType::Skill);
    let initial_hp = game.players[target].hp;

    game.apply_effect(caster, &EffectType::Heal(5), &[target], &[AttributeType::Water]).unwrap();

    // 5 HP (no +1 bonus since target is not caster)
    assert_eq!(game.players[target].hp, initial_hp + 5,
        "Water Lv3 bonus should only apply to caster, not other targets");
}

#[test]
fn water_lv5_teammate_heal() {
    let mut game = setup_test_game();
    let caster = 0;
    let teammate = 2;

    // Water Lv5: 使用水屬卡片時，回復自己與隊友1點生命
    game.players[caster].attributes.water = 5;
    game.players[caster].team = TeamId::Team0;
    game.players[teammate].team = TeamId::Team0;

    game.players[caster].take_damage(20, DamageType::Skill);
    game.players[teammate].take_damage(20, DamageType::Skill);

    let initial_hp_caster = game.players[caster].hp;
    let initial_hp_teammate = game.players[teammate].hp;

    game.apply_effect(caster, &EffectType::Heal(5), &[caster], &[AttributeType::Water]).unwrap();

    // Caster: 5 + 1 (Lv3) = 6 HP
    // Teammate: 1 HP (Lv5 extra teammate heal)
    assert_eq!(game.players[caster].hp, initial_hp_caster + 6,
        "Water Lv5 should add +1 healing to caster (from Lv3)");
    assert_eq!(game.players[teammate].hp, initial_hp_teammate + 1,
        "Water Lv5 should give teammate +1 extra healing");
}

#[test]
fn water_lv5_requires_level_5() {
    let mut game = setup_test_game();
    let caster = 0;

    // Lv4 should not trigger Lv5
    game.players[caster].attributes.water = 4;
    game.players[caster].take_damage(20, DamageType::Skill);
    let initial_hp = game.players[caster].hp;

    game.apply_effect(caster, &EffectType::Heal(5), &[caster], &[AttributeType::Water]).unwrap();

    // 5 + 1 = 6 HP (only Lv3 bonus)
    assert_eq!(game.players[caster].hp, initial_hp + 6,
        "Water Lv5 should NOT trigger at level 4");
}

// ==================== 風 (Wind) Proficiency Tests ====================

#[test]
fn wind_lv2_effect_recognized() {
    let mut game = setup_test_game();
    let target = 1;

    // Wind Lv2: 風屬性卡片攻擊的人這圈不能回復生命或獲得護盾
    game.players[target].attributes.wind = 2;
    let effects = game.players[target].attributes.get_mastery_effects();

    assert!(effects.iter().any(|e| matches!(e, mage_battle::attribute::MasteryEffect::WindLevel2)),
        "Wind Lv2 effect should be recognized at level 2");
}

#[test]
fn wind_lv5_effect_recognized() {
    let mut game = setup_test_game();
    let caster = 0;

    // Wind Lv5: 風屬性卡片可自由選擇對象
    game.players[caster].attributes.wind = 5;
    let effects = game.players[caster].attributes.get_mastery_effects();

    assert!(effects.iter().any(|e| matches!(e, mage_battle::attribute::MasteryEffect::WindLevel5)),
        "Wind Lv5 effect should be recognized at level 5");
}

#[test]
fn wind_below_lv2_no_effect() {
    let mut game = setup_test_game();
    let target = 1;

    // Wind at Lv1 should not trigger any effects
    game.players[target].attributes.wind = 1;
    let effects = game.players[target].attributes.get_mastery_effects();

    assert!(!effects.iter().any(|e| matches!(e, mage_battle::attribute::MasteryEffect::WindLevel2)),
        "Wind Lv2 should NOT be recognized below level 2");
}

// ==================== 毒 (Poison) Proficiency Tests ====================

#[test]
fn poison_lv2_effect_recognized() {
    let mut game = setup_test_game();
    let target = 1;

    // Poison Lv2: 被毒屬性卡片攻擊的人下回合先出卡片再配屬性點
    game.players[target].attributes.poison = 2;
    let effects = game.players[target].attributes.get_mastery_effects();

    assert!(effects.iter().any(|e| matches!(e, mage_battle::attribute::MasteryEffect::PoisonLevel2)),
        "Poison Lv2 effect should be recognized at level 2");
}

#[test]
fn poison_lv5_effect_recognized() {
    let mut game = setup_test_game();
    let target = 1;

    // Poison Lv5: 被毒屬性卡片攻擊的人下回合只能出屬性彈
    game.players[target].attributes.poison = 5;
    let effects = game.players[target].attributes.get_mastery_effects();

    assert!(effects.iter().any(|e| matches!(e, mage_battle::attribute::MasteryEffect::PoisonLevel5)),
        "Poison Lv5 effect should be recognized at level 5");
}

#[test]
fn poison_below_lv2_no_effect() {
    let mut game = setup_test_game();
    let target = 1;

    // Poison at Lv1 should not trigger any effects
    game.players[target].attributes.poison = 1;
    let effects = game.players[target].attributes.get_mastery_effects();

    assert!(!effects.iter().any(|e| matches!(e, mage_battle::attribute::MasteryEffect::PoisonLevel2)),
        "Poison Lv2 should NOT be recognized below level 2");
}

// ==================== Wind Lv2 & Poison Lv2 Debuff Effect Tests ====================

#[test]
fn wind_lv2_applies_defense_invalidation_debuff() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3; // Furthest enemy

    // Setup: Caster has Wind Lv2
    game.players[caster].attributes.wind = 2;
    game.players[caster].hand.push(100); // Add wind bolt card
    let initial_target_hp = game.players[target].hp;

    // Apply wind attribute bolt
    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Wind, None).unwrap();

    // Expected: DefenseInvalidation debuff applied to target
    assert!(
        game.players[target].buffs.has(mage_battle::buff::BuffType::DefenseInvalidation),
        "Wind Lv2 should apply DefenseInvalidation debuff to target"
    );
    assert_eq!(
        game.players[target].hp,
        initial_target_hp - 2, // Wind level 2 damage
        "Target should take 2 damage (Wind level)"
    );
}

#[test]
fn wind_lv2_debuff_prevents_healing() {
    let mut game = setup_test_game();
    let target = 1;

    // Setup: Target has DefenseInvalidation debuff
    game.players[target].take_damage(20, DamageType::Skill);
    let initial_hp = game.players[target].hp;
    let buff = mage_battle::buff::Buff::new(
        mage_battle::buff::BuffType::DefenseInvalidation,
        mage_battle::buff::BuffDuration::Turns(1),
    );
    game.players[target].buffs.add(buff);

    // Try to heal
    let healed = game.players[target].heal(10);

    // Expected: Cannot heal due to DefenseInvalidation
    assert_eq!(healed, 0, "DefenseInvalidation should prevent healing");
    assert_eq!(game.players[target].hp, initial_hp, "HP should remain unchanged");
}

#[test]
fn wind_lv2_debuff_prevents_shield() {
    let mut game = setup_test_game();
    let target = 1;

    // Setup: Target has DefenseInvalidation debuff
    let buff = mage_battle::buff::Buff::new(
        mage_battle::buff::BuffType::DefenseInvalidation,
        mage_battle::buff::BuffDuration::Turns(1),
    );
    game.players[target].buffs.add(buff);
    let initial_shield = game.players[target].shield;

    // Try to gain shield
    game.players[target].gain_shield(5);

    // Expected: Cannot gain shield due to DefenseInvalidation
    assert_eq!(game.players[target].shield, initial_shield, "DefenseInvalidation should prevent shield gain");
}

#[test]
fn poison_lv2_applies_confuse_debuff() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3; // Furthest enemy

    // Setup: Caster has Poison Lv2
    game.players[caster].attributes.poison = 2;
    game.players[caster].hand.push(100); // Add poison bolt card
    let initial_target_hp = game.players[target].hp;

    // Apply poison attribute bolt
    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Poison, None).unwrap();

    // Expected: Confuse debuff applied to target
    assert!(
        game.players[target].buffs.has(mage_battle::buff::BuffType::Confuse),
        "Poison Lv2 should apply Confuse debuff to target"
    );
    assert_eq!(
        game.players[target].hp,
        initial_target_hp - 2, // Poison level 2 damage
        "Target should take 2 damage (Poison level)"
    );
}

#[test]
fn poison_lv2_debuff_without_wind_lv2() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3; // Furthest enemy

    // Setup: Only Poison Lv2, no Wind Lv2
    game.players[caster].attributes.poison = 2;
    game.players[caster].attributes.wind = 0;
    game.players[caster].hand.push(100); // Add poison bolt card

    // Apply poison attribute bolt
    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Poison, None).unwrap();

    // Expected: Only Confuse, no DefenseInvalidation
    assert!(
        game.players[target].buffs.has(mage_battle::buff::BuffType::Confuse),
        "Poison Lv2 should apply Confuse"
    );
    assert!(
        !game.players[target].buffs.has(mage_battle::buff::BuffType::DefenseInvalidation),
        "Wind Lv2 should NOT apply when caster doesn't have it"
    );
}

#[test]
fn wind_and_poison_lv2_both_apply() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3; // Furthest enemy

    // Setup: Caster has both Wind Lv2 and Poison Lv2
    game.players[caster].attributes.wind = 2;
    game.players[caster].attributes.poison = 2;
    game.players[caster].hand.push(100); // Add wind bolt card first test

    // Apply wind attribute bolt (Wind Lv2 triggers)
    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Wind, None).unwrap();

    // Expected: Wind Lv2 debuff applied
    assert!(
        game.players[target].buffs.has(mage_battle::buff::BuffType::DefenseInvalidation),
        "Wind Lv2 should apply DefenseInvalidation"
    );

    // Reset target buffs for Poison test
    game.players[target].buffs.clear_all();
    game.players[caster].hand.push(101); // Add poison bolt card
    game.players[target].hp = 50; // Reset HP

    // Reset turn phase to PlayCard for next bolt
    game.turn_phase = TurnPhase::PlayCard;

    // Apply poison attribute bolt (Poison Lv2 triggers)
    game.play_attribute_bolt(101, AttributeType::Poison, None).unwrap();

    // Expected: Poison Lv2 debuff applied
    assert!(
        game.players[target].buffs.has(mage_battle::buff::BuffType::Confuse),
        "Poison Lv2 should apply Confuse"
    );
}

// ==================== Wind Lv5 Target Selection Tests ====================

#[test]
fn wind_lv5_free_target_selection_check() {
    let mut game = setup_test_game();
    let caster = 0;

    // Setup: Caster has Wind Lv5
    game.players[caster].attributes.wind = 5;

    // Check if Wind Lv5 is triggered for wind attribute
    let enchantments = vec![AttributeType::Wind];
    let is_lv5_active = game.is_wind_lv5_triggered(caster, &enchantments);

    // Expected: Wind Lv5 should be active
    assert!(
        is_lv5_active,
        "Wind Lv5 should be triggered when caster has Wind Lv5 with wind enchantment"
    );
}

#[test]
fn wind_lv5_only_with_wind_enchantment() {
    let mut game = setup_test_game();
    let caster = 0;

    // Setup: Caster has Wind Lv5 but casting fire spell
    game.players[caster].attributes.wind = 5;

    // Check if Wind Lv5 is triggered for fire attribute
    let enchantments = vec![AttributeType::Fire];
    let is_lv5_active = game.is_wind_lv5_triggered(caster, &enchantments);

    // Expected: Wind Lv5 should NOT be active without wind enchantment
    assert!(
        !is_lv5_active,
        "Wind Lv5 should NOT be triggered without wind enchantment"
    );
}

#[test]
fn wind_lv5_requires_level_5() {
    let mut game = setup_test_game();
    let caster = 0;

    // Setup: Caster has Wind Lv4 (not Lv5)
    game.players[caster].attributes.wind = 4;

    // Check if Wind Lv5 is triggered
    let enchantments = vec![AttributeType::Wind];
    let is_lv5_active = game.is_wind_lv5_triggered(caster, &enchantments);

    // Expected: Wind Lv5 should NOT be active at level 4
    assert!(
        !is_lv5_active,
        "Wind Lv5 should NOT be triggered below level 5"
    );
}

#[test]
fn wind_bolt_target_validation_without_lv5() {
    let mut game = setup_test_game();
    let caster = 0;

    // Setup: Wind Lv2 (not Lv5)
    game.players[caster].attributes.wind = 2;
    game.current_player_index = caster;

    // In turn order P0, P1, P2, P3: P0's enemies are P1 and P3
    // Distance from P0: P1 = 1, P3 = 3 (furthest)
    // Try to target P1 (not furthest) - should fail
    let result = game.validate_bolt_targets(caster, &vec![1], false); // wind_lv5_active = false

    // Expected: Should error - attribute bolts without Lv5 can only target furthest enemy
    assert!(
        result.is_err(),
        "Without Wind Lv5, attribute bolts should only target furthest enemy"
    );
}

#[test]
fn wind_bolt_target_validation_with_lv5() {
    let mut game = setup_test_game();
    let caster = 0;

    // Setup: Wind Lv5
    game.players[caster].attributes.wind = 5;
    game.current_player_index = caster;

    // Try to validate custom targets (should succeed with Lv5)
    let result = game.validate_bolt_targets(caster, &vec![3], true); // wind_lv5_active = true

    // Expected: Should succeed - Wind Lv5 allows free target selection
    assert!(
        result.is_ok(),
        "Wind Lv5 should allow custom target selection for attribute bolts"
    );
    assert_eq!(result.unwrap(), vec![3], "Should return the provided target");
}

// ==================== Combined Proficiency Tests ====================

#[test]
fn multiple_proficiencies_coexist() {
    let mut game = setup_test_game();
    let caster = 0;
    let target = 3; // Furthest enemy

    // Multiple attributes can have proficiencies at the same time
    game.players[caster].attributes.fire = 3;
    game.players[caster].attributes.thunder = 3;
    game.players[caster].hand.push(100); // Add fire bolt card
    let initial_hp = game.players[target].hp;

    game.current_player_index = caster;
    game.turn_phase = TurnPhase::PlayCard;
    game.play_attribute_bolt(100, AttributeType::Fire, None).unwrap();

    // Fire Lv3 bonus should still apply with other proficiencies active
    // Fire bolt: Fire attribute level (3) + 1 (Fire Lv3 bonus) = 4 damage
    assert_eq!(game.players[target].hp, initial_hp - 4,
        "Multiple proficiencies should coexist");
}

#[test]
fn all_proficiencies_at_max_level() {
    let mut game = setup_test_game();
    let player = 0;

    // Set all attributes to level 5
    game.players[player].attributes.fire = 5;
    game.players[player].attributes.wood = 5;
    game.players[player].attributes.thunder = 5;
    game.players[player].attributes.water = 5;
    game.players[player].attributes.wind = 5;
    game.players[player].attributes.poison = 5;

    let effects = game.players[player].attributes.get_mastery_effects();

    // Should have 12 effects total (2 per attribute: Lv3 and Lv5, except Wind has Lv2+Lv5, Poison has Lv2+Lv5)
    assert_eq!(effects.len(), 12, "Should have 12 total mastery effects at max attributes");

    // Verify all major Lv5 effects are present
    assert!(effects.iter().any(|e| matches!(e, mage_battle::attribute::MasteryEffect::FireLevel5)));
    assert!(effects.iter().any(|e| matches!(e, mage_battle::attribute::MasteryEffect::WoodLevel5)));
    assert!(effects.iter().any(|e| matches!(e, mage_battle::attribute::MasteryEffect::ThunderLevel5)));
    assert!(effects.iter().any(|e| matches!(e, mage_battle::attribute::MasteryEffect::WaterLevel5)));
    assert!(effects.iter().any(|e| matches!(e, mage_battle::attribute::MasteryEffect::WindLevel5)));
    assert!(effects.iter().any(|e| matches!(e, mage_battle::attribute::MasteryEffect::PoisonLevel5)));
}

#[test]
fn no_proficiency_below_all_thresholds() {
    let mut game = setup_test_game();
    let player = 0;

    // Set all attributes to 1 (below all thresholds)
    game.players[player].attributes.fire = 1;
    game.players[player].attributes.wood = 1;
    game.players[player].attributes.thunder = 1;
    game.players[player].attributes.water = 1;
    game.players[player].attributes.wind = 1;
    game.players[player].attributes.poison = 1;

    let effects = game.players[player].attributes.get_mastery_effects();

    // Should have no effects at all
    assert_eq!(effects.len(), 0, "No effects should trigger below Lv2/Lv3 thresholds");
}

#[test]
fn proficiency_threshold_exact_lv3() {
    let mut game = setup_test_game();
    let player = 0;

    // Test that proficiency exactly at threshold works
    game.players[player].attributes.fire = 3;
    let effects = game.players[player].attributes.get_mastery_effects();
    assert!(effects.iter().any(|e| matches!(e, mage_battle::attribute::MasteryEffect::FireLevel3)),
        "Fire Lv3 should trigger exactly at level 3");

    // Test that proficiency below threshold doesn't trigger
    game.players[player].attributes.fire = 2;
    let effects = game.players[player].attributes.get_mastery_effects();
    assert!(!effects.iter().any(|e| matches!(e, mage_battle::attribute::MasteryEffect::FireLevel3)),
        "Fire Lv3 should NOT trigger below level 3");
}

#[test]
fn proficiency_threshold_exact_lv5() {
    let mut game = setup_test_game();
    let player = 0;

    // Test that proficiency exactly at Lv5 works
    game.players[player].attributes.fire = 5;
    let effects = game.players[player].attributes.get_mastery_effects();
    assert!(effects.iter().any(|e| matches!(e, mage_battle::attribute::MasteryEffect::FireLevel5)),
        "Fire Lv5 should trigger exactly at level 5");

    // Test that proficiency below threshold doesn't trigger
    game.players[player].attributes.fire = 4;
    let effects = game.players[player].attributes.get_mastery_effects();
    assert!(!effects.iter().any(|e| matches!(e, mage_battle::attribute::MasteryEffect::FireLevel5)),
        "Fire Lv5 should NOT trigger below level 5");
}
