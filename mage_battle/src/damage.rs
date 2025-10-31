use serde::{Deserialize, Serialize};

/// Damage types as defined in Battle_Logic.txt Section 6
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageType {
    /// Spell damage: The damage caused by spells (including attribute bolts)
    Spell,
    /// Skill damage: The damage caused by skills (like Fire Lv5 passive)
    Skill,
    /// Direct damage: The damage specified on description (bypasses all protections)
    /// Examples: Wood Lv3 HP cost
    Direct,
}

impl DamageType {
    /// Check if this damage type is affected by Wood Lv5 reduction
    pub fn is_reduced_by_wood_lv5(&self) -> bool {
        matches!(self, DamageType::Spell)
    }

    /// Check if this damage type is affected by Guard Wood Carving
    pub fn is_reduced_by_guard_wood_carving(&self) -> bool {
        matches!(self, DamageType::Spell | DamageType::Skill)
    }

    /// Check if this damage type is blocked by shield
    pub fn is_blocked_by_shield(&self) -> bool {
        // Direct damage bypasses shield
        !matches!(self, DamageType::Direct)
    }
}
