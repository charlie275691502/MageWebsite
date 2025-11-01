// NOTE: These tests are disabled because play_attribute_bolt() has been removed.
// Attribute bolts are now data-driven (loaded from spells.json as regular spells).
// These tests should be rewritten to use play_spell_card() with attribute bolt spell cards.

use mage_battle::character::CharacterType;
use mage_battle::game::Game;

#[test]
#[ignore]
fn test_wood_lv3_proficiency_with_attribute_bolt() {
    // TODO: Rewrite using play_spell_card() with wood attribute bolt spell from spells.json
}

#[test]
#[ignore]
fn test_wood_lv3_basic_spell_from_50hp() {
    // TODO: Rewrite using play_spell_card() with wood attribute bolt spell from spells.json
}

#[test]
#[ignore]
fn test_wood_lv3_hp_cost_not_affected_by_shield() {
    // TODO: Rewrite using play_spell_card() with wood attribute bolt spell from spells.json
}
