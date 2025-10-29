use serde::{Deserialize, Serialize};

/// Buff類型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuffType {
    /// ＃１ 免疫：免疫傷害與負面效果
    Immune,
    /// ＃２ 化身：免疫傷害與負面效果，持續直到下一位玩家完成行動
    Invincible,
    /// ＃３ 癱瘓：無法行動
    Paralysis,
    /// ＃４ 封印：無法使用解放技能
    Seal,
    /// ＃５ 沈默：無法使用屬性彈以外的法術
    Silent,
    /// ＃６ 元素剝離：使用法術時不會帶有屬性精通效果
    MasterDisable,
    /// ＃７ 防禦崩解：無法回復生命或獲得護盾
    DefenseInvalidation,
    /// ＃８ 混亂：回合開始時必須先出卡片再配屬性點
    Confuse,
    /// ＃９ 生命汲取：回合開始時對指定對象造成1點傷害並回復自身1點生命
    HealthDrain,
    /// ＃１０ 寄主：此目標被生命汲取
    HealthDrainTarget,
    /// ＃１１ 再生：回合開始時回復7點生命
    Regeneration,
    /// ＃１２ 燃燒殆盡：火屬性攻擊傷害+5，回合開始時移除1點火屬性點
    BurningOut,
    /// ＃１３ 守護木雕：受到傷害-4，此效果在受到3次攻擊後會消失
    GuardWoodCarving,
}

impl BuffType {
    /// 是否為負面效果
    pub fn is_debuff(&self) -> bool {
        matches!(self,
            BuffType::Paralysis |
            BuffType::Seal |
            BuffType::Silent |
            BuffType::MasterDisable |
            BuffType::DefenseInvalidation |
            BuffType::Confuse |
            BuffType::HealthDrainTarget
        )
    }

    /// 是否為正面效果
    pub fn is_buff(&self) -> bool {
        !self.is_debuff()
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            BuffType::Immune => "免疫",
            BuffType::Invincible => "化身",
            BuffType::Paralysis => "癱瘓",
            BuffType::Seal => "封印",
            BuffType::Silent => "沈默",
            BuffType::MasterDisable => "元素剝離",
            BuffType::DefenseInvalidation => "防禦崩解",
            BuffType::Confuse => "混亂",
            BuffType::HealthDrain => "生命汲取",
            BuffType::HealthDrainTarget => "寄主",
            BuffType::Regeneration => "再生",
            BuffType::BurningOut => "燃燒殆盡",
            BuffType::GuardWoodCarving => "守護木雕",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            BuffType::Immune => "免疫傷害與負面效果",
            BuffType::Invincible => "免疫傷害與負面效果，持續直到下一位玩家完成行動",
            BuffType::Paralysis => "無法行動",
            BuffType::Seal => "無法使用解放技能",
            BuffType::Silent => "無法使用屬性彈以外的法術",
            BuffType::MasterDisable => "使用法術時不會帶有屬性精通效果",
            BuffType::DefenseInvalidation => "無法回復生命或獲得護盾",
            BuffType::Confuse => "回合開始時必須先出卡片再配屬性點",
            BuffType::HealthDrain => "回合開始時對指定對象造成1點傷害並回復自身1點生命",
            BuffType::HealthDrainTarget => "此目標被生命汲取",
            BuffType::Regeneration => "回合開始時回復7點生命",
            BuffType::BurningOut => "火屬性攻擊傷害+5，回合開始時移除1點火屬性點",
            BuffType::GuardWoodCarving => "受到傷害-4，此效果在受到3次攻擊後會消失",
        }
    }
}

/// Buff實例
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Buff {
    pub buff_type: BuffType,
    pub duration: BuffDuration,
    /// 特殊數據（例如守護木雕的剩餘次數，生命汲取的目標）
    pub data: Option<i32>,
}

impl Buff {
    pub fn new(buff_type: BuffType, duration: BuffDuration) -> Self {
        Self {
            buff_type,
            duration,
            data: None,
        }
    }

    pub fn new_with_data(buff_type: BuffType, duration: BuffDuration, data: i32) -> Self {
        Self {
            buff_type,
            duration,
            data: Some(data),
        }
    }

    /// 減少持續時間
    pub fn tick(&mut self) -> bool {
        match &mut self.duration {
            BuffDuration::Turns(ref mut turns) => {
                if *turns > 0 {
                    *turns -= 1;
                }
                *turns == 0
            }
            BuffDuration::UntilNextPlayer => true,
            BuffDuration::Permanent => false,
            BuffDuration::UntilHit(ref mut hits) => {
                // 這個在受到攻擊時處理
                *hits == 0
            }
        }
    }

    /// 是否已過期
    pub fn is_expired(&self) -> bool {
        match self.duration {
            BuffDuration::Turns(turns) => turns == 0,
            BuffDuration::UntilNextPlayer => false,
            BuffDuration::Permanent => false,
            BuffDuration::UntilHit(hits) => hits == 0,
        }
    }
}

/// Buff持續時間
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuffDuration {
    /// 持續N回合
    Turns(u8),
    /// 持續到下一位玩家
    UntilNextPlayer,
    /// 永久
    Permanent,
    /// 持續到受到N次攻擊
    UntilHit(u8),
}

/// 玩家的Buff集合
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuffList {
    pub buffs: Vec<Buff>,
}

impl BuffList {
    pub fn new() -> Self {
        Self { buffs: Vec::new() }
    }

    /// 添加Buff
    pub fn add(&mut self, buff: Buff) {
        // 檢查是否已經有相同類型的buff
        if let Some(existing) = self.buffs.iter_mut().find(|b| b.buff_type == buff.buff_type) {
            // 更新持續時間（取較長的）
            match (&existing.duration, &buff.duration) {
                (BuffDuration::Turns(e), BuffDuration::Turns(n)) => {
                    existing.duration = BuffDuration::Turns((*e).max(*n));
                }
                (BuffDuration::Permanent, _) | (_, BuffDuration::Permanent) => {
                    existing.duration = BuffDuration::Permanent;
                }
                _ => {}
            }
        } else {
            self.buffs.push(buff);
        }
    }

    /// 移除Buff
    pub fn remove(&mut self, buff_type: BuffType) {
        self.buffs.retain(|b| b.buff_type != buff_type);
    }

    /// 檢查是否有指定Buff
    pub fn has(&self, buff_type: BuffType) -> bool {
        self.buffs.iter().any(|b| b.buff_type == buff_type)
    }

    /// 獲取指定Buff
    pub fn get(&self, buff_type: BuffType) -> Option<&Buff> {
        self.buffs.iter().find(|b| b.buff_type == buff_type)
    }

    /// 獲取指定Buff（可變）
    pub fn get_mut(&mut self, buff_type: BuffType) -> Option<&mut Buff> {
        self.buffs.iter_mut().find(|b| b.buff_type == buff_type)
    }

    /// 更新所有Buff（回合結束時調用）
    pub fn tick_all(&mut self) {
        for buff in &mut self.buffs {
            buff.tick();
        }
        self.buffs.retain(|b| !b.is_expired());
    }

    /// 清除所有負面效果
    pub fn clear_debuffs(&mut self) {
        self.buffs.retain(|b| !b.buff_type.is_debuff());
    }

    /// 清除所有效果
    pub fn clear_all(&mut self) {
        self.buffs.clear();
    }

    /// 檢查是否免疫傷害
    pub fn is_immune_to_damage(&self) -> bool {
        self.has(BuffType::Immune) || self.has(BuffType::Invincible)
    }

    /// 檢查是否免疫負面效果
    pub fn is_immune_to_debuff(&self) -> bool {
        self.has(BuffType::Immune) || self.has(BuffType::Invincible)
    }

    /// 檢查是否可以行動
    pub fn can_act(&self) -> bool {
        !self.has(BuffType::Paralysis)
    }

    /// 檢查是否可以使用解放
    pub fn can_liberate(&self) -> bool {
        !self.has(BuffType::Seal)
    }

    /// 檢查是否只能使用屬性彈
    pub fn only_basic_spell(&self) -> bool {
        self.has(BuffType::Silent)
    }

    /// 檢查是否有精通效果
    pub fn has_mastery(&self) -> bool {
        !self.has(BuffType::MasterDisable)
    }

    /// 檢查是否可以回復或獲得護盾
    pub fn can_heal_or_shield(&self) -> bool {
        !self.has(BuffType::DefenseInvalidation)
    }
}

impl Default for BuffList {
    fn default() -> Self {
        Self::new()
    }
}
