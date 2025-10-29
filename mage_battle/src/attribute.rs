use serde::{Deserialize, Serialize};

/// 屬性類型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttributeType {
    Fire,    // 火
    Wood,    // 木
    Thunder, // 雷
    Water,   // 水
    Wind,    // 風
    Poison,  // 毒
}

impl AttributeType {
    /// 是否為主要屬性
    pub fn is_main(&self) -> bool {
        matches!(self,
            AttributeType::Fire |
            AttributeType::Wood |
            AttributeType::Thunder |
            AttributeType::Water
        )
    }

    /// 是否為輔助屬性
    pub fn is_sub(&self) -> bool {
        matches!(self, AttributeType::Wind | AttributeType::Poison)
    }

    pub fn all_types() -> Vec<AttributeType> {
        vec![
            AttributeType::Fire,
            AttributeType::Wood,
            AttributeType::Thunder,
            AttributeType::Water,
            AttributeType::Wind,
            AttributeType::Poison,
        ]
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            AttributeType::Fire => "火",
            AttributeType::Wood => "木",
            AttributeType::Thunder => "雷",
            AttributeType::Water => "水",
            AttributeType::Wind => "風",
            AttributeType::Poison => "毒",
        }
    }
}

/// 屬性點管理
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributePoints {
    pub fire: u8,
    pub wood: u8,
    pub thunder: u8,
    pub water: u8,
    pub wind: u8,
    pub poison: u8,
}

impl AttributePoints {
    pub fn new() -> Self {
        Self {
            fire: 0,
            wood: 0,
            thunder: 0,
            water: 0,
            wind: 0,
            poison: 0,
        }
    }

    /// 獲取指定屬性的點數
    pub fn get(&self, attr: AttributeType) -> u8 {
        match attr {
            AttributeType::Fire => self.fire,
            AttributeType::Wood => self.wood,
            AttributeType::Thunder => self.thunder,
            AttributeType::Water => self.water,
            AttributeType::Wind => self.wind,
            AttributeType::Poison => self.poison,
        }
    }

    /// 設置指定屬性的點數
    pub fn set(&mut self, attr: AttributeType, value: u8) {
        let clamped = value.min(5); // 上限5
        match attr {
            AttributeType::Fire => self.fire = clamped,
            AttributeType::Wood => self.wood = clamped,
            AttributeType::Thunder => self.thunder = clamped,
            AttributeType::Water => self.water = clamped,
            AttributeType::Wind => self.wind = clamped,
            AttributeType::Poison => self.poison = clamped,
        }
    }

    /// 增加屬性點
    pub fn add(&mut self, attr: AttributeType, amount: u8) {
        let current = self.get(attr);
        self.set(attr, current.saturating_add(amount));
    }

    /// 減少屬性點
    pub fn remove(&mut self, attr: AttributeType, amount: u8) -> bool {
        let current = self.get(attr);
        if current >= amount {
            self.set(attr, current - amount);
            true
        } else {
            false
        }
    }

    /// 檢查是否滿足需求
    pub fn meets_requirement(&self, requirements: &[(AttributeType, u8)]) -> bool {
        requirements.iter().all(|(attr, level)| self.get(*attr) >= *level)
    }

    /// 獲取所有輔助屬性點數總和
    pub fn sub_total(&self) -> u8 {
        self.wind + self.poison
    }

    /// 獲取所有主要屬性點數總和
    pub fn main_total(&self) -> u8 {
        self.fire + self.wood + self.thunder + self.water
    }

    /// 重置指定屬性
    pub fn reset(&mut self, attr: AttributeType) {
        self.set(attr, 0);
    }

    /// 將所有點數轉為數組 [火, 木, 雷, 水, 風, 毒]
    pub fn to_array(&self) -> [u8; 6] {
        [self.fire, self.wood, self.thunder, self.water, self.wind, self.poison]
    }

    /// 從數組設置點數
    pub fn from_array(&mut self, arr: [u8; 6]) {
        self.fire = arr[0].min(5);
        self.wood = arr[1].min(5);
        self.thunder = arr[2].min(5);
        self.water = arr[3].min(5);
        self.wind = arr[4].min(5);
        self.poison = arr[5].min(5);
    }
}

/// 屬性精通效果
#[derive(Debug, Clone)]
pub enum MasteryEffect {
    /// 火Lv3: 所有屬性彈+1
    FireLevel3,
    /// 火Lv5: 回合開始時，對所有敵人造成1點傷害
    FireLevel5,
    /// 木Lv3: 減少1點生命並獲得1點護盾
    WoodLevel3,
    /// 木Lv5: 自己與隊友受到的卡片傷害-1
    WoodLevel5,
    /// 雷Lv3: 雷屬性卡片傷害+1
    ThunderLevel3,
    /// 雷Lv5: 雷屬性卡片傷害+2 (合計+3)
    ThunderLevel5,
    /// 水Lv3: 使用水屬卡片時，回復自身1的生命
    WaterLevel3,
    /// 水Lv5: 使用水屬卡片時，回復自己與隊友1點生命
    WaterLevel5,
    /// 風Lv2: 風屬性卡片攻擊的人這圈不能回復生命或獲得護盾
    WindLevel2,
    /// 風Lv5: 風屬性卡片可自由選擇對象
    WindLevel5,
    /// 毒Lv2: 被毒屬性卡片攻擊的人下回合先出卡片再配屬性點
    PoisonLevel2,
    /// 毒Lv5: 被毒屬性卡片攻擊的人下回合只能出屬性彈
    PoisonLevel5,
}

impl AttributePoints {
    /// 獲取當前擁有的精通效果
    pub fn get_mastery_effects(&self) -> Vec<MasteryEffect> {
        let mut effects = Vec::new();

        // 火
        if self.fire >= 3 {
            effects.push(MasteryEffect::FireLevel3);
        }
        if self.fire >= 5 {
            effects.push(MasteryEffect::FireLevel5);
        }

        // 木
        if self.wood >= 3 {
            effects.push(MasteryEffect::WoodLevel3);
        }
        if self.wood >= 5 {
            effects.push(MasteryEffect::WoodLevel5);
        }

        // 雷
        if self.thunder >= 3 {
            effects.push(MasteryEffect::ThunderLevel3);
        }
        if self.thunder >= 5 {
            effects.push(MasteryEffect::ThunderLevel5);
        }

        // 水
        if self.water >= 3 {
            effects.push(MasteryEffect::WaterLevel3);
        }
        if self.water >= 5 {
            effects.push(MasteryEffect::WaterLevel5);
        }

        // 風
        if self.wind >= 2 {
            effects.push(MasteryEffect::WindLevel2);
        }
        if self.wind >= 5 {
            effects.push(MasteryEffect::WindLevel5);
        }

        // 毒
        if self.poison >= 2 {
            effects.push(MasteryEffect::PoisonLevel2);
        }
        if self.poison >= 5 {
            effects.push(MasteryEffect::PoisonLevel5);
        }

        effects
    }
}

impl Default for AttributePoints {
    fn default() -> Self {
        Self::new()
    }
}
