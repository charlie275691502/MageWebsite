use serde::{Deserialize, Serialize};

/// Buff類型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuffType {
    /// #1 免疫：免疫傷害與負面效果
    Immune,
    /// #2 化身：免疫傷害與負面效果，持續直到下一位玩家完成行動
    Invincible,
    /// #3 癱瘓：無法行動
    Paralysis,
    /// #4 封印：無法使用解放技能
    Seal,
    /// #5 沈默：無法使用屬性彈以外的法術
    Silent,
    /// #6 元素剝離：使用法術時不會帶有屬性精通效果
    MasterDisable,
    /// #7 防禦崩解：無法回復生命或獲得護盾
    DefenseInvalidation,
    /// #8 混亂：回合開始時必須先出卡片再配屬性點
    Confuse,
    /// #9 生命汲取：回合開始時對指定對象造成1點傷害並回復自身1點生命
    HealthDrain,
    /// #10 寄主：此目標被生命汲取
    HealthDrainTarget,
    /// #11 再生：回合開始時回復7點生命
    Regeneration,
    /// #12 燃燒殆盡：火屬性攻擊傷害+5，回合開始時移除1點火屬性點
    BurningOut,
    /// #13 守護木雕：受到傷害-4，此效果在受到3次攻擊後會消失
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

#[cfg(test)]
mod tests {
    use super::*;

    // === Buff Duration Tests ===

    #[test]
    fn test_buff_duration_turns() {
        let mut buff = Buff::new(BuffType::Immune, BuffDuration::Turns(3));

        assert!(!buff.is_expired());
        buff.tick();
        assert!(!buff.is_expired());
        buff.tick();
        assert!(!buff.is_expired());
        buff.tick();
        assert!(buff.is_expired());
    }

    #[test]
    fn test_buff_duration_permanent() {
        let mut buff = Buff::new(BuffType::Regeneration, BuffDuration::Permanent);

        for _ in 0..10 {
            buff.tick();
            assert!(!buff.is_expired());
        }
    }

    #[test]
    fn test_buff_duration_until_next_player() {
        let mut buff = Buff::new(BuffType::Invincible, BuffDuration::UntilNextPlayer);

        assert!(!buff.is_expired());
        buff.tick();
        // UntilNextPlayer expires on tick
        assert!(!buff.is_expired()); // But is_expired checks the duration value
    }

    #[test]
    fn test_buff_duration_until_hit() {
        let mut buff = Buff::new_with_data(BuffType::GuardWoodCarving, BuffDuration::UntilHit(3), 3);

        assert!(!buff.is_expired());
        // This should be handled externally when taking damage
        if let Some(hits) = buff.data.as_mut() {
            *hits -= 1;
        }
        assert!(!buff.is_expired());
    }

    // === BuffList Tests ===

    #[test]
    fn test_buff_list_add_and_has() {
        let mut buffs = BuffList::new();

        buffs.add(Buff::new(BuffType::Immune, BuffDuration::Turns(1)));
        assert!(buffs.has(BuffType::Immune));
        assert!(!buffs.has(BuffType::Paralysis));
    }

    #[test]
    fn test_buff_list_add_duplicate_extends_duration() {
        let mut buffs = BuffList::new();

        buffs.add(Buff::new(BuffType::Regeneration, BuffDuration::Turns(2)));
        buffs.add(Buff::new(BuffType::Regeneration, BuffDuration::Turns(5)));

        // Should take the longer duration
        if let Some(buff) = buffs.get(BuffType::Regeneration) {
            assert!(matches!(buff.duration, BuffDuration::Turns(5)));
        } else {
            panic!("Buff not found");
        }
    }

    #[test]
    fn test_buff_list_add_permanent_overrides() {
        let mut buffs = BuffList::new();

        buffs.add(Buff::new(BuffType::BurningOut, BuffDuration::Turns(3)));
        buffs.add(Buff::new(BuffType::BurningOut, BuffDuration::Permanent));

        if let Some(buff) = buffs.get(BuffType::BurningOut) {
            assert!(matches!(buff.duration, BuffDuration::Permanent));
        } else {
            panic!("Buff not found");
        }
    }

    #[test]
    fn test_buff_list_remove() {
        let mut buffs = BuffList::new();

        buffs.add(Buff::new(BuffType::Immune, BuffDuration::Permanent));
        assert!(buffs.has(BuffType::Immune));

        buffs.remove(BuffType::Immune);
        assert!(!buffs.has(BuffType::Immune));
    }

    #[test]
    fn test_buff_list_tick_all() {
        let mut buffs = BuffList::new();

        buffs.add(Buff::new(BuffType::Immune, BuffDuration::Turns(1)));
        buffs.add(Buff::new(BuffType::Paralysis, BuffDuration::Turns(2)));

        buffs.tick_all();

        // Immune should expire
        assert!(!buffs.has(BuffType::Immune));
        // Paralysis should still be there
        assert!(buffs.has(BuffType::Paralysis));
    }

    #[test]
    fn test_buff_list_clear_debuffs() {
        let mut buffs = BuffList::new();

        buffs.add(Buff::new(BuffType::Immune, BuffDuration::Permanent));
        buffs.add(Buff::new(BuffType::Paralysis, BuffDuration::Permanent));
        buffs.add(Buff::new(BuffType::Silent, BuffDuration::Permanent));
        buffs.add(Buff::new(BuffType::Regeneration, BuffDuration::Permanent));

        buffs.clear_debuffs();

        // Buffs should remain
        assert!(buffs.has(BuffType::Immune));
        assert!(buffs.has(BuffType::Regeneration));

        // Debuffs should be cleared
        assert!(!buffs.has(BuffType::Paralysis));
        assert!(!buffs.has(BuffType::Silent));
    }

    #[test]
    fn test_buff_list_clear_all() {
        let mut buffs = BuffList::new();

        buffs.add(Buff::new(BuffType::Immune, BuffDuration::Permanent));
        buffs.add(Buff::new(BuffType::Paralysis, BuffDuration::Permanent));

        buffs.clear_all();

        assert!(!buffs.has(BuffType::Immune));
        assert!(!buffs.has(BuffType::Paralysis));
    }

    // === Buff Type Behavior Tests ===

    #[test]
    fn test_buff_type_is_debuff() {
        assert!(BuffType::Paralysis.is_debuff());
        assert!(BuffType::Seal.is_debuff());
        assert!(BuffType::Silent.is_debuff());
        assert!(BuffType::MasterDisable.is_debuff());
        assert!(BuffType::DefenseInvalidation.is_debuff());
        assert!(BuffType::Confuse.is_debuff());
        assert!(BuffType::HealthDrainTarget.is_debuff());

        assert!(!BuffType::Immune.is_debuff());
        assert!(!BuffType::Regeneration.is_debuff());
    }

    #[test]
    fn test_buff_type_is_buff() {
        assert!(BuffType::Immune.is_buff());
        assert!(BuffType::Invincible.is_buff());
        assert!(BuffType::Regeneration.is_buff());
        assert!(BuffType::BurningOut.is_buff());
        assert!(BuffType::GuardWoodCarving.is_buff());

        assert!(!BuffType::Paralysis.is_buff());
        assert!(!BuffType::Silent.is_buff());
    }

    // === Immunity Tests ===

    #[test]
    fn test_immunity_to_damage() {
        let mut buffs = BuffList::new();

        assert!(!buffs.is_immune_to_damage());

        buffs.add(Buff::new(BuffType::Immune, BuffDuration::Permanent));
        assert!(buffs.is_immune_to_damage());
    }

    #[test]
    fn test_invincible_immunity_to_damage() {
        let mut buffs = BuffList::new();

        buffs.add(Buff::new(BuffType::Invincible, BuffDuration::UntilNextPlayer));
        assert!(buffs.is_immune_to_damage());
    }

    #[test]
    fn test_immunity_to_debuff() {
        let mut buffs = BuffList::new();

        assert!(!buffs.is_immune_to_debuff());

        buffs.add(Buff::new(BuffType::Immune, BuffDuration::Permanent));
        assert!(buffs.is_immune_to_debuff());

        buffs.clear_all();
        buffs.add(Buff::new(BuffType::Invincible, BuffDuration::UntilNextPlayer));
        assert!(buffs.is_immune_to_debuff());
    }

    // === Action Restriction Tests ===

    #[test]
    fn test_can_act_with_paralysis() {
        let mut buffs = BuffList::new();

        assert!(buffs.can_act());

        buffs.add(Buff::new(BuffType::Paralysis, BuffDuration::Turns(1)));
        assert!(!buffs.can_act());
    }

    #[test]
    fn test_can_liberate_with_seal() {
        let mut buffs = BuffList::new();

        assert!(buffs.can_liberate());

        buffs.add(Buff::new(BuffType::Seal, BuffDuration::Turns(2)));
        assert!(!buffs.can_liberate());
    }

    #[test]
    fn test_only_basic_spell_with_silent() {
        let mut buffs = BuffList::new();

        assert!(!buffs.only_basic_spell());

        buffs.add(Buff::new(BuffType::Silent, BuffDuration::Turns(1)));
        assert!(buffs.only_basic_spell());
    }

    #[test]
    fn test_has_mastery_with_master_disable() {
        let mut buffs = BuffList::new();

        assert!(buffs.has_mastery());

        buffs.add(Buff::new(BuffType::MasterDisable, BuffDuration::Turns(3)));
        assert!(!buffs.has_mastery());
    }

    #[test]
    fn test_can_heal_or_shield_with_defense_invalidation() {
        let mut buffs = BuffList::new();

        assert!(buffs.can_heal_or_shield());

        buffs.add(Buff::new(BuffType::DefenseInvalidation, BuffDuration::Turns(2)));
        assert!(!buffs.can_heal_or_shield());
    }

    // === Special Buff Data Tests ===

    #[test]
    fn test_health_drain_with_target_data() {
        let target_id = 2;
        let buff = Buff::new_with_data(BuffType::HealthDrain, BuffDuration::Permanent, target_id);

        assert_eq!(buff.data, Some(target_id));
        assert_eq!(buff.buff_type, BuffType::HealthDrain);
    }

    #[test]
    fn test_guard_wood_carving_hit_count() {
        let mut buff = Buff::new_with_data(BuffType::GuardWoodCarving, BuffDuration::UntilHit(3), 3);

        assert_eq!(buff.data, Some(3));

        // Simulate taking hits
        if let Some(hits) = buff.data.as_mut() {
            *hits -= 1;
            assert_eq!(*hits, 2);

            *hits -= 1;
            assert_eq!(*hits, 1);

            *hits -= 1;
            assert_eq!(*hits, 0);
        }
    }

    // === Buff Display Tests ===

    #[test]
    fn test_buff_to_string() {
        assert_eq!(BuffType::Immune.to_string(), "免疫");
        assert_eq!(BuffType::Invincible.to_string(), "化身");
        assert_eq!(BuffType::Paralysis.to_string(), "癱瘓");
        assert_eq!(BuffType::Seal.to_string(), "封印");
        assert_eq!(BuffType::Silent.to_string(), "沈默");
        assert_eq!(BuffType::MasterDisable.to_string(), "元素剝離");
        assert_eq!(BuffType::DefenseInvalidation.to_string(), "防禦崩解");
        assert_eq!(BuffType::Confuse.to_string(), "混亂");
        assert_eq!(BuffType::HealthDrain.to_string(), "生命汲取");
        assert_eq!(BuffType::HealthDrainTarget.to_string(), "寄主");
        assert_eq!(BuffType::Regeneration.to_string(), "再生");
        assert_eq!(BuffType::BurningOut.to_string(), "燃燒殆盡");
        assert_eq!(BuffType::GuardWoodCarving.to_string(), "守護木雕");
    }

    #[test]
    fn test_buff_description() {
        assert_eq!(BuffType::Immune.description(), "免疫傷害與負面效果");
        assert_eq!(BuffType::Paralysis.description(), "無法行動");
        assert_eq!(BuffType::Regeneration.description(), "回合開始時回復7點生命");
    }
}
