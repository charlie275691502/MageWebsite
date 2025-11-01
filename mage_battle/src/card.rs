use crate::attribute::AttributeType;
use crate::effect::{SpellEffect, TargetPool, EffectType};
use crate::buff::BuffType;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// 卡片編號（1-60）
pub type CardId = u32;

/// JSON format for card effects
#[derive(Debug, Clone, Deserialize)]
struct JsonEffect {
    #[serde(rename = "type")]
    effect_type: String,
    params: Vec<serde_json::Value>,
}

/// JSON format for spell data
#[derive(Debug, Clone, Deserialize)]
struct JsonSpell {
    spell_id: String,
    name: String,
    cost: String,
    description: String,
    #[serde(default)]
    target_pool: Option<String>,
    #[serde(default)]
    effects: Option<Vec<JsonEffect>>,
    #[serde(default)]
    is_attribute_bolt: bool,
}

/// JSON format for card data (referencing spell IDs)
#[derive(Debug, Clone, Deserialize)]
struct JsonCard {
    id: u32,
    top_spell_id: String,
    bottom_spell_id: Option<String>,
}

/// Root JSON structure for cards
#[derive(Debug, Deserialize)]
struct CardsData {
    cards: Vec<JsonCard>,
}

/// Root JSON structure for spells
#[derive(Debug, Deserialize)]
struct SpellsData {
    spells: Vec<JsonSpell>,
}

/// 法術卡片（雙面）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: CardId,
    pub top_spell: SpellSide,
    pub bottom_spell: Option<SpellSide>,  // 強力卡只有一個法術（只有Top）
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardSide {
    Top,
    Bottom,
}

impl Card {
    pub fn new(id: CardId, top_spell: SpellSide, bottom_spell: Option<SpellSide>) -> Self {
        Self {
            id,
            top_spell,
            bottom_spell,
        }
    }

    /// 獲取指定面的法術
    pub fn get_spell(&self, side: CardSide) -> Option<&SpellSide> {
        match side {
            CardSide::Top => Some(&self.top_spell),
            CardSide::Bottom => self.bottom_spell.as_ref(),
        }
    }

    /// 檢查是否只有單面（強力卡）
    pub fn is_single_sided(&self) -> bool {
        self.bottom_spell.is_none()
    }

    /// 檢查指定面是否可以使用（檢查屬性需求）
    pub fn can_play(&self, side: CardSide, attributes: &crate::attribute::AttributePoints) -> bool {
        if let Some(spell) = self.get_spell(side) {
            spell.can_play(attributes)
        } else {
            false
        }
    }
}

/// 法術的一面
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellSide {
    pub spell_id: String,        // 例如 "A1", "B33"
    pub name: String,             // 法術名稱
    pub requirements: Vec<(AttributeType, u8)>,  // 發動條件
    pub effect: SpellEffect,      // 效果
    pub is_aoyi: bool,            // 是否為奧義
}

impl SpellSide {
    pub fn new(
        spell_id: String,
        name: String,
        requirements: Vec<(AttributeType, u8)>,
        effect: SpellEffect,
    ) -> Self {
        Self {
            spell_id,
            name,
            requirements,
            effect,
            is_aoyi: false,
        }
    }

    pub fn as_aoyi(mut self) -> Self {
        self.is_aoyi = true;
        self
    }

    /// 檢查是否可以使用
    pub fn can_play(&self, attributes: &crate::attribute::AttributePoints) -> bool {
        attributes.meets_requirement(&self.requirements)
    }

    /// 是否為奧義
    pub fn is_aoyi(&self) -> bool {
        self.is_aoyi
    }

    /// 獲取主要屬性（用於判斷精通效果）
    pub fn get_main_attribute(&self) -> Option<AttributeType> {
        self.requirements.iter()
            .filter(|(attr, _)| attr.is_main())
            .max_by_key(|(_, level)| level)
            .map(|(attr, _)| *attr)
    }

    /// 獲取需求描述
    pub fn requirement_string(&self) -> String {
        if self.requirements.is_empty() {
            return "無".to_string();
        }

        self.requirements
            .iter()
            .map(|(attr, level)| format!("{}{}", attr.to_string(), level))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// 卡片資料庫（從Excel數據生成）
pub struct CardDatabase {
    cards: Vec<Card>,
}

impl CardDatabase {
    pub fn new() -> Self {
        Self {
            cards: Self::load_cards(),
        }
    }

    /// Parse cost string like "火2" or "水4 雷2" into requirements
    fn parse_cost(cost: &str) -> Vec<(AttributeType, u8)> {
        if cost.is_empty() {
            return vec![];
        }

        let attr_map: std::collections::HashMap<char, AttributeType> = [
            ('火', AttributeType::Fire),
            ('木', AttributeType::Wood),
            ('雷', AttributeType::Thunder),
            ('水', AttributeType::Water),
            ('風', AttributeType::Wind),
            ('毒', AttributeType::Poison),
        ].iter().cloned().collect();

        let mut requirements = Vec::new();
        let parts: Vec<&str> = cost.split_whitespace().collect();

        for part in parts {
            let chars: Vec<char> = part.chars().collect();
            if chars.len() >= 2 {
                if let Some(&attr_type) = attr_map.get(&chars[0]) {
                    if let Some(level) = chars[1..].iter().collect::<String>().parse::<u8>().ok() {
                        requirements.push((attr_type, level));
                    }
                }
            }
        }

        requirements
    }

    /// Parse target pool string
    fn parse_target_pool(target: &str) -> TargetPool {
        match target {
            "Default" => TargetPool::Default,
            "Self_" => TargetPool::Self_,
            "Enemies" => TargetPool::Enemies,
            "Allies" => TargetPool::Allies,
            "All" => TargetPool::All,
            "AllOtherPlayers" => TargetPool::AllOtherPlayers,
            "ChooseFromEnemies" => TargetPool::ChooseFromEnemies,
            "ChooseFromAllies" => TargetPool::ChooseFromAllies,
            "ChooseFromAll" => TargetPool::ChooseFromAll,
            _ => TargetPool::Default,
        }
    }

    /// Parse single effect from JSON
    fn parse_effect(json_effect: &JsonEffect) -> Option<EffectType> {
        let effect_type = json_effect.effect_type.as_str();
        let params = &json_effect.params;

        let get_u32 = |idx: usize| -> Option<u32> {
            params.get(idx)?.as_u64().map(|v| v as u32)
        };

        let get_buff = |idx: usize| -> Option<BuffType> {
            let s = params.get(idx)?.as_str()?;
            match s {
                "Immune" => Some(BuffType::Immune),
                "Invincible" => Some(BuffType::Invincible),
                "Paralysis" => Some(BuffType::Paralysis),
                "Seal" => Some(BuffType::Seal),
                "Silent" => Some(BuffType::Silent),
                "MasterDisable" => Some(BuffType::MasterDisable),
                "DefenseInvalidation" => Some(BuffType::DefenseInvalidation),
                "Confuse" => Some(BuffType::Confuse),
                "HealthDrain" => Some(BuffType::HealthDrain),
                "HealthDrainTarget" => Some(BuffType::HealthDrainTarget),
                "Regeneration" => Some(BuffType::Regeneration),
                "BurningOut" => Some(BuffType::BurningOut),
                "GuardWoodCarving" => Some(BuffType::GuardWoodCarving),
                _ => None,
            }
        };

        match effect_type {
            // Damage effects
            "Damage" => Some(EffectType::Damage(get_u32(0)?)),
            "DoubleDamage" => Some(EffectType::DoubleDamage(get_u32(0)?)),
            "TripleDamage" => Some(EffectType::TripleDamage(get_u32(0)?)),
            "IncreaseDamage" => Some(EffectType::IncreaseDamage(get_u32(0)?)),
            "SubMTDDamage" => Some(EffectType::SubMTDDamage(get_u32(0)?)),
            "ShieldDDamage" => Some(EffectType::ShieldDDamage(get_u32(0)?)),

            // Attribute resets
            "FlameDeallocation" => Some(EffectType::FlameDeallocation),
            "WoodDeallocation" => Some(EffectType::WoodDeallocation),
            "SparkDeallocation" => Some(EffectType::SparkDeallocation),
            "WaterDeallocation" => Some(EffectType::WaterDeallocation),

            // Healing
            "Heal" => Some(EffectType::Heal(get_u32(0)?)),
            "HealSelf" => Some(EffectType::HealSelf(get_u32(0)?)),

            // Shield
            "Shield" => Some(EffectType::Shield(get_u32(0)?)),
            "ShieldSelf" => Some(EffectType::ShieldSelf(get_u32(0)?)),
            "DestroyAllShield" => Some(EffectType::DestroyAllShield),

            // Buffs
            "BuffOne" => Some(EffectType::BuffOne(get_buff(0)?)),
            "BuffForever" => Some(EffectType::BuffForever(get_buff(0)?)),
            "BuffSelfOne" => Some(EffectType::BuffSelfOne(get_buff(0)?)),
            "BuffSelfTwo" => Some(EffectType::BuffSelfTwo(get_buff(0)?)),
            "BuffSelfFour" => Some(EffectType::BuffSelfFour(get_buff(0)?)),
            "BuffSelfSix" => Some(EffectType::BuffSelfSix(get_buff(0)?)),
            "HealthDrain" => Some(EffectType::HealthDrain),
            "GuardWoodCarving" => Some(EffectType::GuardWoodCarving(get_buff(0)?)),
            "RemoveAllBuff" => Some(EffectType::RemoveAllBuff),

            // Attribute operations
            "GainAPSelf" => Some(EffectType::GainAPSelf(get_u32(0)?)),
            "MoveAP" => Some(EffectType::MoveAP(get_u32(0)?)),
            "MoveAPSelf" => Some(EffectType::MoveAPSelf(get_u32(0)?)),
            "RemoveMainMTAP" => Some(EffectType::RemoveMainMTAP(get_u32(0)?)),
            "RemoveAP" => Some(EffectType::RemoveAP(get_u32(0)?)),

            // Card operations
            "GetAoyiSelf" => Some(EffectType::GetAoyiSelf),
            "RandomDiscard" => Some(EffectType::RandomDiscard),
            "ViewTopDrawPile" => Some(EffectType::ViewTopDrawPile),
            "ViewAndDiscard" => Some(EffectType::ViewAndDiscard(get_u32(0)?)),
            "DiscardAndDraw" => Some(EffectType::DiscardAndDraw),

            // Special
            "Dearouse" => Some(EffectType::Dearouse),
            "AddOneMagicType" => Some(EffectType::AddOneMagicType),

            _ => None,
        }
    }

    /// Parse JSON spell to SpellSide
    fn parse_spell(json_spell: &JsonSpell) -> Option<SpellSide> {
        let target_pool = json_spell.target_pool
            .as_ref()
            .map(|s| Self::parse_target_pool(s))
            .unwrap_or(TargetPool::Default);

        let effects = json_spell.effects.as_ref()?;

        if effects.is_empty() {
            return None;
        }

        let effect1 = Self::parse_effect(&effects[0])?;
        let effect2 = effects.get(1).and_then(|e| Self::parse_effect(e));

        let mut spell_effect = SpellEffect::new(target_pool, effect1);
        if let Some(e2) = effect2 {
            spell_effect = spell_effect.with_second_effect(e2);
        }

        Some(SpellSide::new(
            json_spell.spell_id.clone(),
            json_spell.name.clone(),
            Self::parse_cost(&json_spell.cost),
            spell_effect,
        ))
    }

    /// 加載所有卡片從 JSON 文件
    fn load_cards() -> Vec<Card> {
        // Load spells first
        let spells_json = include_str!("../spells.json");
        let spells_data: SpellsData = match serde_json::from_str(spells_json) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to parse spells.json: {}", e);
                return Self::load_fallback_cards();
            }
        };

        // Build spell lookup map
        let mut spell_map = std::collections::HashMap::new();
        for json_spell in spells_data.spells {
            if let Some(spell) = Self::parse_spell(&json_spell) {
                spell_map.insert(json_spell.spell_id.clone(), spell);
            }
        }

        // Load cards
        let cards_json = include_str!("../cards.json");
        let cards_data: CardsData = match serde_json::from_str(cards_json) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to parse cards.json: {}", e);
                return Self::load_fallback_cards();
            }
        };

        let mut cards = Vec::new();

        for json_card in cards_data.cards {
            // Look up top spell
            let top_spell = match spell_map.get(&json_card.top_spell_id) {
                Some(spell) => spell.clone(),
                None => {
                    eprintln!("Spell {} not found for card {}", json_card.top_spell_id, json_card.id);
                    // Fallback
                    SpellSide::new(
                        json_card.top_spell_id.clone(),
                        format!("未知法術{}", json_card.top_spell_id),
                        vec![(AttributeType::Fire, 1)],
                        SpellEffect::new(TargetPool::Default, EffectType::Damage(2)),
                    )
                }
            };

            // Look up bottom spell if exists
            let bottom_spell = json_card.bottom_spell_id
                .and_then(|spell_id| spell_map.get(&spell_id).cloned());

            cards.push(Card::new(json_card.id, top_spell, bottom_spell));
        }

        // Fill remaining cards with placeholders if less than 60
        for i in (cards.len() as u32 + 1)..=60 {
            let top_spell = SpellSide::new(
                format!("T{}", i),
                format!("測試法術{}", i),
                vec![(AttributeType::Fire, 1)],
                SpellEffect::new(TargetPool::Default, EffectType::Damage(2)),
            );

            cards.push(Card::new(i, top_spell, None));
        }

        cards.sort_by_key(|c| c.id);
        cards
    }

    /// Fallback card loading if JSON parsing fails
    fn load_fallback_cards() -> Vec<Card> {
        let mut cards = Vec::new();

        for i in 1..=60 {
            let top_spell = SpellSide::new(
                format!("T{}", i),
                format!("測試法術{}", i),
                vec![(AttributeType::Fire, 1)],
                SpellEffect::new(TargetPool::Default, EffectType::Damage(2)),
            );

            let bottom_spell = if i % 2 == 0 {
                Some(SpellSide::new(
                    format!("B{}", i),
                    format!("測試法術{}B", i),
                    vec![(AttributeType::Wood, 1)],
                    SpellEffect::new(TargetPool::Default, EffectType::Damage(3)),
                ))
            } else {
                None
            };

            cards.push(Card::new(i, top_spell, bottom_spell));
        }

        cards
    }

    pub fn get_card(&self, id: CardId) -> Option<&Card> {
        self.cards.iter().find(|c| c.id == id)
    }

    pub fn get_all_cards(&self) -> &[Card] {
        &self.cards
    }

    /// 獲取所有奧義卡
    pub fn get_aoyi_cards(&self) -> Vec<&Card> {
        self.cards.iter().filter(|c| {
            c.top_spell.is_aoyi || c.bottom_spell.as_ref().map_or(false, |s| s.is_aoyi)
        }).collect()
    }

    /// 創建初始牌組（60張公牌）
    pub fn create_deck(&self) -> Vec<CardId> {
        self.cards.iter().map(|c| c.id).collect()
    }
}

impl Default for CardDatabase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attribute::AttributePoints;
    use crate::effect::EffectType;

    #[test]
    fn test_card_requirements() {
        let side_a = SpellSide::new(
            "A1".to_string(),
            "炎爆".to_string(),
            vec![(AttributeType::Fire, 1)],
            SpellEffect::new(TargetPool::Default, EffectType::IncreaseDamage(2)),
        );

        let side_b = SpellSide::new(
            "B33".to_string(),
            "御者".to_string(),
            vec![(AttributeType::Wind, 5)],
            SpellEffect::new(TargetPool::Default, EffectType::Damage(6))
                .with_second_effect(EffectType::GainAPSelf(2)),
        );

        let card = Card::new(1, side_a, Some(side_b));

        let mut attrs = AttributePoints::new();
        attrs.set(AttributeType::Fire, 1);

        assert!(card.can_play(CardSide::Top, &attrs));

        attrs.set(AttributeType::Fire, 0);
        attrs.set(AttributeType::Wind, 5);

        assert!(!card.can_play(CardSide::Top, &attrs));  // A面無法使用
    }

    #[test]
    fn test_load_cards_from_json() {
        let db = CardDatabase::new();
        let cards = db.get_all_cards();

        // Should have 60 cards total (8 from JSON + 52 placeholders)
        assert_eq!(cards.len(), 60, "Should have 60 cards total");

        // Test card 1 (A1 炎爆 + B33 御者)
        let card1 = db.get_card(1).expect("Card 1 should exist");
        assert_eq!(card1.top_spell.name, "炎爆");
        assert_eq!(card1.top_spell.requirements, vec![(AttributeType::Fire, 1)]);

        // Verify the effect is IncreaseDamage(2)
        match &card1.top_spell.effect.effect1 {
            EffectType::IncreaseDamage(val) => assert_eq!(val, &2),
            _ => panic!("Expected IncreaseDamage effect"),
        }

        // Test bottom spell exists
        let bottom = card1.bottom_spell.as_ref().expect("Card 1 should have bottom spell");
        assert_eq!(bottom.name, "御者");
        assert_eq!(bottom.requirements, vec![(AttributeType::Wind, 5)]);

        // Test card 3 (A2 火球 + B30 隨風)
        let card3 = db.get_card(3).expect("Card 3 should exist");
        assert_eq!(card3.top_spell.name, "火球");

        // Verify the effect is Damage(5)
        match &card3.top_spell.effect.effect1 {
            EffectType::Damage(val) => assert_eq!(val, &5),
            _ => panic!("Expected Damage effect"),
        }

        println!("✅ Successfully loaded and parsed cards from cards.json");
        println!("Total cards loaded: {}", cards.len());
    }
}
