use crate::attribute::AttributeType;
use crate::effect::{SpellEffect, TargetPool};
use serde::{Deserialize, Serialize};

/// 卡片編號（1-60）
pub type CardId = u32;

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

/// 屬性彈（基礎攻擊）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeBolt {
    pub attribute: AttributeType,
    pub level: u8,  // 屬性等級（1-5）
}

impl AttributeBolt {
    pub fn new(attribute: AttributeType, level: u8) -> Self {
        Self {
            attribute,
            level: level.min(5),
        }
    }

    /// 獲取基礎傷害（等於屬性等級）
    pub fn base_damage(&self) -> u32 {
        self.level as u32
    }

    /// 獲取名稱
    pub fn name(&self) -> String {
        format!("{}屬性彈", self.attribute.to_string())
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

    /// 加載所有卡片（這裡先創建一些測試卡片，實際數據需要從Excel解析）
    fn load_cards() -> Vec<Card> {
        use crate::effect::{SpellEffect, TargetPool, EffectType};

        let mut cards = Vec::new();

        // 創建60張測試卡片
        for i in 1..=60 {
            // 簡單的測試法術
            let top_spell = SpellSide::new(
                format!("T{}", i),
                format!("測試法術{}", i),
                vec![(AttributeType::Fire, 1)],  // 需要火1
                SpellEffect::new(
                    TargetPool::Default,
                    EffectType::Damage(2)
                ),
            );

            // 一半的卡片有底部法術，一半只有頂部
            let bottom_spell = if i % 2 == 0 {
                Some(SpellSide::new(
                    format!("B{}", i),
                    format!("測試法術{}B", i),
                    vec![(AttributeType::Wood, 1)],  // 需要木1
                    SpellEffect::new(
                        TargetPool::Default,
                        EffectType::Damage(3)
                    ),
                ))
            } else {
                None  // 單面強力卡
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

        let card = Card::new(1, side_a, side_b);

        let mut attrs = AttributePoints::new();
        attrs.set(AttributeType::Fire, 1);

        assert!(card.can_play(&attrs));

        attrs.set(AttributeType::Fire, 0);
        attrs.set(AttributeType::Wind, 5);

        assert!(!card.can_play(&attrs));  // A面無法使用
    }
}
