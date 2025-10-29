use crate::attribute::AttributeType;
use serde::{Deserialize, Serialize};

/// 角色類型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharacterType {
    FlamePoison,    // 火毒法師
    WoodWind,       // 木風法師
    ThunderPoison,  // 雷毒法師
    WaterWind,      // 水風法師
}

impl CharacterType {
    pub fn name(&self) -> &'static str {
        match self {
            CharacterType::FlamePoison => "Zeuberer_Flame_1",
            CharacterType::WoodWind => "Zeuberer_Wood_1",
            CharacterType::ThunderPoison => "Zeuberer_Spark_1",
            CharacterType::WaterWind => "Zeuberer_Water_1",
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            CharacterType::FlamePoison => "火毒法師",
            CharacterType::WoodWind => "木風法師",
            CharacterType::ThunderPoison => "雷毒法師",
            CharacterType::WaterWind => "水風法師",
        }
    }

    /// 獲取主屬性
    pub fn main_attribute(&self) -> AttributeType {
        match self {
            CharacterType::FlamePoison => AttributeType::Fire,
            CharacterType::WoodWind => AttributeType::Wood,
            CharacterType::ThunderPoison => AttributeType::Thunder,
            CharacterType::WaterWind => AttributeType::Water,
        }
    }

    /// 獲取副屬性
    pub fn sub_attribute(&self) -> AttributeType {
        match self {
            CharacterType::FlamePoison => AttributeType::Poison,
            CharacterType::WoodWind => AttributeType::Wind,
            CharacterType::ThunderPoison => AttributeType::Poison,
            CharacterType::WaterWind => AttributeType::Wind,
        }
    }

    /// 獲取初始屬性點 [火, 木, 雷, 水, 風, 毒]
    pub fn initial_attributes(&self) -> [u8; 6] {
        match self {
            CharacterType::FlamePoison => [2, 0, 0, 0, 0, 1],
            CharacterType::WoodWind => [0, 2, 0, 0, 1, 0],
            CharacterType::ThunderPoison => [0, 0, 2, 0, 0, 1],
            CharacterType::WaterWind => [0, 0, 0, 2, 1, 0],
        }
    }

    /// 解放技能需求
    pub fn liberation_requirement(&self) -> Vec<(AttributeType, u8)> {
        match self {
            CharacterType::FlamePoison => vec![
                (AttributeType::Fire, 4),
                (AttributeType::Poison, 2),
            ],
            CharacterType::WoodWind => vec![
                (AttributeType::Wood, 4),
                (AttributeType::Wind, 2),
            ],
            CharacterType::ThunderPoison => vec![
                (AttributeType::Thunder, 4),
                (AttributeType::Poison, 2),
            ],
            CharacterType::WaterWind => vec![
                (AttributeType::Water, 4),
                (AttributeType::Wind, 2),
            ],
        }
    }

    pub fn liberation_name(&self) -> &'static str {
        match self {
            CharacterType::FlamePoison => "燃燒殆盡",
            CharacterType::WoodWind => "守護木雕",
            CharacterType::ThunderPoison => "鏈鎖電擊",
            CharacterType::WaterWind => "颶風之眼",
        }
    }

    pub fn liberation_description(&self) -> &'static str {
        match self {
            CharacterType::FlamePoison => "施法者火屬性攻擊傷害+5，回合開始時移除1點火屬性點，無可移除火屬性點則消除此狀態。",
            CharacterType::WoodWind => "賦予所有友軍受到傷害-4的增益狀態，此效果在受到3次攻擊後會消失。",
            CharacterType::ThunderPoison => "對所有其他玩家造成10點傷害。",
            CharacterType::WaterWind => "回合開始時回復7點生命，持續4回合。",
        }
    }

    pub fn all_types() -> Vec<CharacterType> {
        vec![
            CharacterType::FlamePoison,
            CharacterType::WoodWind,
            CharacterType::ThunderPoison,
            CharacterType::WaterWind,
        ]
    }
}

/// 角色狀態
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub character_type: CharacterType,
    pub is_liberated: bool,  // 是否已解放
}

impl Character {
    pub fn new(character_type: CharacterType) -> Self {
        Self {
            character_type,
            is_liberated: false,
        }
    }

    /// 檢查是否可以解放
    pub fn can_liberate(&self, attributes: &crate::attribute::AttributePoints) -> bool {
        !self.is_liberated && attributes.meets_requirement(&self.character_type.liberation_requirement())
    }

    /// 解放角色
    pub fn liberate(&mut self) {
        self.is_liberated = true;
    }

    /// 獲取額外手牌數
    pub fn extra_hand_size(&self) -> usize {
        if self.is_liberated { 1 } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attribute::AttributePoints;

    #[test]
    fn test_character_liberation() {
        let mut character = Character::new(CharacterType::FlamePoison);
        let mut attrs = AttributePoints::new();

        assert!(!character.can_liberate(&attrs));

        attrs.set(AttributeType::Fire, 4);
        attrs.set(AttributeType::Poison, 2);

        assert!(character.can_liberate(&attrs));

        character.liberate();
        assert!(character.is_liberated);
        assert_eq!(character.extra_hand_size(), 1);
    }
}
