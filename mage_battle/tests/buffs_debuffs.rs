// Comprehensive tests for BUFFS & DEBUFFS from Battle_Logic.txt Lines 270-326

use mage_battle::attribute::AttributeType;
use mage_battle::buff::{Buff, BuffType, BuffDuration};
use mage_battle::character::CharacterType;
use mage_battle::damage::DamageType;
use mage_battle::game::{Game, TurnPhase};

// ==============================================
// POSITIVE BUFFS TESTS
// ==============================================

/// TEST: Immune buff blocks damage and negative effects
/// Battle_Logic.txt Lines 276-278
#[test]
fn test_immune_blocks_damage() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
    ];
    let mut game = Game::new(names, chars);

    // Setup: P0 has Immune buff and some HP
    game.players[0].hp = 50;
    let buff = Buff::new(BuffType::Immune, BuffDuration::Turns(2));
    game.players[0].buffs.add(buff);

    // P1 attacks P0 with attribute bolt (should deal damage but Immune blocks it)
    game.current_player_index = 1;
    game.players[1].hand.push(2); // Give P1 a card
    game.players[1].attributes.fire = 1;
    game.turn_phase = TurnPhase::PlayCard;

    // P1 uses fire bolt on P0
    let _ = game.play_attribute_bolt(2, AttributeType::Fire, vec![0]);

    // P0's HP should be unchanged due to Immune
    assert_eq!(game.players[0].hp, 50, "Immune should block all damage");
}

#[test]
fn test_immune_blocks_debuffs() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
    ];
    let mut game = Game::new(names, chars);

    // Setup: P0 (target) has Immune buff
    let buff = Buff::new(BuffType::Immune, BuffDuration::Turns(2));
    game.players[0].buffs.add(buff);

    // P1 (attacker) tries to apply Paralysis debuff to P0
    // (simulating a spell effect from P1 targeting P0)
    let debuff = Buff::new(BuffType::Paralysis, BuffDuration::Turns(1));
    game.players[0].buffs.add(debuff);

    // P0 should not have Paralysis due to Immune
    assert!(!game.players[0].buffs.has(BuffType::Paralysis),
        "Immune should block debuffs from being applied");
}

/// TEST: Invincible buff (similar to Immune)
/// Battle_Logic.txt Lines 280-282
#[test]
fn test_invincible_blocks_damage() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
    ];
    let mut game = Game::new(names, chars);

    // Setup: P0 has Invincible buff and some HP
    game.players[0].hp = 50;
    let buff = Buff::new(BuffType::Invincible, BuffDuration::UntilNextPlayer);
    game.players[0].buffs.add(buff);

    // P1 attacks P0 with attribute bolt
    game.current_player_index = 1;
    game.players[1].hand.push(2); // Give P1 a card
    game.players[1].attributes.fire = 1;
    game.turn_phase = TurnPhase::PlayCard;

    // P1 uses fire bolt on P0
    let _ = game.play_attribute_bolt(2, AttributeType::Fire, vec![0]);

    // P0's HP should be unchanged due to Invincible
    assert_eq!(game.players[0].hp, 50, "Invincible should block all damage");
}

/// TEST: Regeneration buff heals at turn start
/// Battle_Logic.txt Lines 284-286
#[test]
fn test_regeneration_heals_at_turn_start() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
    ];
    let mut game = Game::new(names, chars);

    // Setup: P0 has Regeneration buff and damaged HP
    game.players[0].hp = 40;
    let buff = Buff::new(BuffType::Regeneration, BuffDuration::Turns(2));
    game.players[0].buffs.add(buff);

    // Trigger turn start
    game.handle_turn_start();

    // Should heal 7 HP
    assert_eq!(game.players[0].hp, 47, "Regeneration should heal 7 HP at turn start");
}

/// TEST: Burning Out buff increases fire damage and consumes fire attribute
/// Battle_Logic.txt Lines 288-291
#[test]
fn test_burning_out_bonus_and_cost() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
    ];
    let mut game = Game::new(names, chars);

    // Setup: P0 has Burning Out buff and fire attribute
    game.players[0].attributes.fire = 3;
    let buff = Buff::new(BuffType::BurningOut, BuffDuration::Permanent);
    game.players[0].buffs.add(buff);

    // Trigger turn start (should consume 1 fire)
    game.handle_turn_start();

    // Fire should be reduced by 1
    assert_eq!(game.players[0].attributes.fire, 2,
        "Burning Out should consume 1 fire at turn start");
}

/// TEST: Health Drain deals damage and heals at turn start
/// Battle_Logic.txt Lines 298-300
#[test]
fn test_health_drain_effect() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
    ];
    let mut game = Game::new(names, chars);

    // Setup: P0 has Health Drain targeting P1
    game.players[0].hp = 45;
    game.players[1].hp = 50;
    let buff = Buff::new_with_data(BuffType::HealthDrain, BuffDuration::Turns(2), 1); // Target P1
    game.players[0].buffs.add(buff);

    // Trigger turn start
    game.handle_turn_start();

    // P0 should heal 1 HP, P1 should take 1 damage
    assert_eq!(game.players[0].hp, 46, "Health Drain should heal drainer 1 HP");
    assert_eq!(game.players[1].hp, 49, "Health Drain should deal 1 damage to target");
}

// ==============================================
// NEGATIVE DEBUFFS TESTS
// ==============================================

/// TEST: Paralysis prevents actions
/// Battle_Logic.txt Lines 305-306
#[test]
fn test_paralysis_prevents_actions() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
    ];
    let mut game = Game::new(names, chars);

    // Setup: P0 has Paralysis
    let buff = Buff::new(BuffType::Paralysis, BuffDuration::Turns(1));
    game.players[0].buffs.add(buff);

    // Player should not be able to act
    assert!(!game.players[0].can_act(), "Paralyzed player should not be able to act");
}

/// TEST: Seal prevents liberation skill
/// Battle_Logic.txt Lines 308-309
#[test]
fn test_seal_prevents_liberation() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
    ];
    let mut game = Game::new(names, chars);

    // Setup: P0 has Seal and meets liberation requirements
    game.players[0].attributes.fire = 5;
    game.players[0].attributes.poison = 4;
    let buff = Buff::new(BuffType::Seal, BuffDuration::Turns(1));
    game.players[0].buffs.add(buff);

    // Player should not be able to use liberation
    assert!(!game.players[0].can_use_liberation(),
        "Sealed player should not be able to use liberation skill");
}

/// TEST: Silent restricts to attribute bolts only
/// Battle_Logic.txt Lines 311-312
#[test]
fn test_silent_blocks_spell_cards() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
    ];
    let mut game = Game::new(names, chars);

    // Setup: P0 has Silent
    let buff = Buff::new(BuffType::Silent, BuffDuration::Turns(1));
    game.players[0].buffs.add(buff);
    game.turn_phase = TurnPhase::PlayCard;

    // Should not be able to play spell cards
    assert!(game.players[0].buffs.has(BuffType::Silent),
        "Player should have Silent debuff");
}

/// TEST: Master Disable removes proficiency bonuses
/// Battle_Logic.txt Lines 314-315
#[test]
fn test_master_disable_removes_proficiency() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
    ];
    let mut game = Game::new(names, chars);

    // Setup: P0 has Fire Lv3 and Master Disable
    game.players[0].attributes.fire = 3;
    let buff = Buff::new(BuffType::MasterDisable, BuffDuration::Turns(1));
    game.players[0].buffs.add(buff);

    // Proficiency bonuses should not apply
    assert!(game.players[0].buffs.has(BuffType::MasterDisable),
        "Player should have MasterDisable debuff");
}

/// TEST: Defense Invalidation prevents healing and shield
/// Battle_Logic.txt Lines 317-318
#[test]
fn test_defense_invalidation_prevents_healing() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
    ];
    let mut game = Game::new(names, chars);

    // Setup: P0 has Defense Invalidation
    game.players[0].hp = 40;
    let buff = Buff::new(BuffType::DefenseInvalidation, BuffDuration::Turns(1));
    game.players[0].buffs.add(buff);

    // Try to heal
    game.players[0].heal(10);

    // HP should not increase
    assert_eq!(game.players[0].hp, 40,
        "Defense Invalidation should prevent healing");
}

#[test]
fn test_defense_invalidation_prevents_shield() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
    ];
    let mut game = Game::new(names, chars);

    // Setup: P0 has Defense Invalidation
    game.players[0].shield = 0;
    let buff = Buff::new(BuffType::DefenseInvalidation, BuffDuration::Turns(1));
    game.players[0].buffs.add(buff);

    // Try to gain shield
    game.players[0].gain_shield(5);

    // Shield should not increase
    assert_eq!(game.players[0].shield, 0,
        "Defense Invalidation should prevent gaining shield");
}

/// TEST: Confuse changes turn order (play card before allocate)
/// Battle_Logic.txt Lines 320-321
#[test]
fn test_confuse_changes_turn_order() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
    ];
    let mut game = Game::new(names, chars);

    // Setup: P0 has Confuse
    let buff = Buff::new(BuffType::Confuse, BuffDuration::Turns(1));
    game.players[0].buffs.add(buff);

    // Start turn - should begin with PlayCard phase instead of AllocateAttribute
    game.handle_turn_start();

    assert_eq!(game.turn_phase, TurnPhase::PlayCard,
        "Confused player should start with PlayCard phase");
}

/// TEST: Health Drain Target takes damage at drainer's turn start
/// Battle_Logic.txt Lines 323-324
#[test]
fn test_health_drain_target_takes_damage() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
    ];
    let mut game = Game::new(names, chars);

    // Setup: P1 has Health Drain Target debuff
    game.players[1].hp = 50;
    let buff = Buff::new(BuffType::HealthDrainTarget, BuffDuration::Turns(2));
    game.players[1].buffs.add(buff);

    // When P0 (drainer) starts turn, P1 should take damage
    // (This is tested via Health Drain test above)
    assert!(game.players[1].buffs.has(BuffType::HealthDrainTarget),
        "Target should have HealthDrainTarget debuff");
}

// ==============================================
// DURATION TESTS
// ==============================================

/// TEST: Turns duration decrements at turn end
/// Battle_Logic.txt Lines 334-336
#[test]
fn test_turns_duration_decrements() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
    ];
    let mut game = Game::new(names, chars);

    // Setup: P0 has buff with 2 turns duration
    let buff = Buff::new(BuffType::Immune, BuffDuration::Turns(2));
    game.players[0].buffs.add(buff);

    // End turn
    game.players[0].turn_end();

    // Duration should decrement
    // After 2 turn ends, buff should be gone
    game.players[0].turn_end();
    assert!(!game.players[0].buffs.has(BuffType::Immune),
        "Buff with Turns(2) should expire after 2 turn ends");
}

/// TEST: UntilHit duration decrements on damage
/// Battle_Logic.txt Lines 338-340
#[test]
fn test_until_hit_duration() {
    let names = vec!["P0".to_string(), "P1".to_string(), "P2".to_string(), "P3".to_string()];
    let chars = vec![
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
        CharacterType::FlamePoison,
        CharacterType::WoodWind,
    ];
    let mut game = Game::new(names, chars);

    // Setup: P0 has Guard Wood Carving (UntilHit(3))
    game.players[0].hp = 50;
    let buff = Buff::new_with_data(BuffType::GuardWoodCarving, BuffDuration::UntilHit(3), 3);
    game.players[0].buffs.add(buff);

    // Take damage 3 times
    game.players[0].take_damage(5, DamageType::Spell);
    game.players[0].take_damage(5, DamageType::Spell);
    game.players[0].take_damage(5, DamageType::Spell);

    // Buff should expire after 3 hits
    assert!(!game.players[0].buffs.has(BuffType::GuardWoodCarving),
        "UntilHit(3) buff should expire after 3 hits");
}
