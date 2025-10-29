use crate::attribute::AttributeType;
use crate::buff::{Buff, BuffDuration, BuffType};
use serde::{Deserialize, Serialize};

/// 效果類型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    // === 傷害類 ===
    /// 造成傷害
    Damage(u32),
    /// 造成2次傷害
    DoubleDamage(u32),
    /// 造成3次傷害
    TripleDamage(u32),
    /// 造成屬性等級+N點傷害
    IncreaseDamage(u32),
    /// 造成等同於目標N倍輔助屬性點數和的傷害
    SubMTDDamage(u32),
    /// 造成傷害，每點護盾使傷害+2
    ShieldDDamage(u32),

    // === 屬性重置類 ===
    /// 重置自身火屬性點
    FlameDeallocation,
    /// 重置自身木屬性點
    WoodDeallocation,
    /// 重置自身雷屬性點
    SparkDeallocation,
    /// 重置自身水屬性點
    WaterDeallocation,

    // === 治療類 ===
    /// 回復目標生命
    Heal(u32),
    /// 回復自身生命
    HealSelf(u32),

    // === 護盾類 ===
    /// 賦予目標護盾
    Shield(u32),
    /// 賦予自身護盾
    ShieldSelf(u32),
    /// 移除所有玩家的護盾
    DestroyAllShield,

    // === Buff類 ===
    /// 賦予目標一回合Buff
    BuffOne(BuffType),
    /// 賦予目標永久Buff
    BuffForever(BuffType),
    /// 賦予自身一回合Buff
    BuffSelfOne(BuffType),
    /// 賦予自身二回合Buff
    BuffSelfTwo(BuffType),
    /// 賦予自身四回合Buff
    BuffSelfFour(BuffType),
    /// 賦予自身六回合Buff
    BuffSelfSix(BuffType),
    /// 賦予目標永久生命汲取
    HealthDrain,
    /// 賦予友軍守護木雕（受到3次攻擊消失）
    GuardWoodCarving(BuffType),
    /// 解除所有玩家的異常狀態
    RemoveAllBuff,

    // === 屬性點操作類 ===
    /// 獲得N點屬性點
    GainAPSelf(u32),
    /// 移動目標N點屬性點（需要互動）
    MoveAP(u32),
    /// 移動自身N點屬性點（需要互動）
    MoveAPSelf(u32),
    /// 移除目標N點主要屬性點並隨機捨棄其一張手牌
    RemoveMainMTAP(u32),
    /// 移除目標N點屬性點
    RemoveAP(u32),

    // === 卡牌操作類 ===
    /// 將棄牌堆中隨機一張奧義卡加入手牌
    GetAoyiSelf,
    /// 捨棄目標隨機一張手牌
    RandomDiscard,
    /// 檢視公牌最上面的卡片，可以將它捨棄或放回去
    ViewTopDrawPile,
    /// 檢視目標手牌並捨棄指定一張牌，其後目標抽一張牌
    ViewAndDiscard(u32),
    /// 可捨棄自身任意數量之手牌，並抽相同數量
    DiscardAndDraw,

    // === 特殊類 ===
    /// 將目標角色改為未解放狀態
    Dearouse,
    /// 同時發動另一主要屬性的精通效果
    AddOneMagicType,
}

impl EffectType {
    /// 是否需要用戶互動
    pub fn requires_interaction(&self) -> bool {
        matches!(
            self,
            EffectType::MoveAP(_) |
            EffectType::MoveAPSelf(_) |
            EffectType::ViewTopDrawPile |
            EffectType::ViewAndDiscard(_) |
            EffectType::DiscardAndDraw |
            EffectType::AddOneMagicType
        )
    }

    /// 效果的文字描述
    pub fn description(&self) -> String {
        match self {
            EffectType::Damage(n) => format!("造成{}點傷害", n),
            EffectType::DoubleDamage(n) => format!("造成2次{}點傷害", n),
            EffectType::TripleDamage(n) => format!("造成3次{}點傷害", n),
            EffectType::IncreaseDamage(n) => format!("造成屬性等級+{}點傷害", n),
            EffectType::SubMTDDamage(n) => format!("造成等同於目標{}倍輔助屬性點數和的傷害", n),
            EffectType::ShieldDDamage(n) => format!("造成{}點傷害，每點護盾使此傷害+2", n),
            EffectType::FlameDeallocation => "重置自身所有火屬性點".to_string(),
            EffectType::WoodDeallocation => "重置自身所有木屬性點".to_string(),
            EffectType::SparkDeallocation => "重置自身所有雷屬性點".to_string(),
            EffectType::WaterDeallocation => "重置自身所有水屬性點".to_string(),
            EffectType::Heal(n) => format!("回復目標{}點生命", n),
            EffectType::HealSelf(n) => format!("回復自身{}點生命", n),
            EffectType::Shield(n) => format!("賦予目標{}點護盾", n),
            EffectType::ShieldSelf(n) => format!("賦予自身{}點護盾", n),
            EffectType::DestroyAllShield => "移除所有玩家的護盾".to_string(),
            EffectType::BuffOne(buff) => format!("賦予目標一回合{}", buff.to_string()),
            EffectType::BuffForever(buff) => format!("賦予目標永久{}", buff.to_string()),
            EffectType::BuffSelfOne(buff) => format!("賦予自身一回合{}", buff.to_string()),
            EffectType::BuffSelfTwo(buff) => format!("賦予自身二回合{}", buff.to_string()),
            EffectType::BuffSelfFour(buff) => format!("賦予自身四回合{}", buff.to_string()),
            EffectType::BuffSelfSix(buff) => format!("賦予自身六回合{}", buff.to_string()),
            EffectType::HealthDrain => "賦予目標永久生命汲取".to_string(),
            EffectType::GuardWoodCarving(_) => "賦予友軍守護木雕".to_string(),
            EffectType::RemoveAllBuff => "解除所有玩家的異常狀態".to_string(),
            EffectType::GainAPSelf(n) => format!("獲得{}點屬性點", n),
            EffectType::MoveAP(n) => format!("移動目標{}點屬性點", n),
            EffectType::MoveAPSelf(n) => format!("移動自身{}點屬性點", n),
            EffectType::RemoveMainMTAP(n) => format!("移除目標{}點主要屬性點並隨機捨棄其一張手牌", n),
            EffectType::RemoveAP(n) => format!("移除目標{}點屬性點", n),
            EffectType::GetAoyiSelf => "將棄牌堆中隨機一張奧義卡加入手牌".to_string(),
            EffectType::RandomDiscard => "捨棄目標隨機一張手牌".to_string(),
            EffectType::ViewTopDrawPile => "檢視公牌最上面的卡片，可以將它捨棄或放回去".to_string(),
            EffectType::ViewAndDiscard(n) => format!("檢視目標手牌並捨棄指定{}張牌，其後目標抽一張牌", n),
            EffectType::DiscardAndDraw => "可捨棄自身任意數量之手牌，並抽相同數量".to_string(),
            EffectType::Dearouse => "將目標角色改為未解放狀態".to_string(),
            EffectType::AddOneMagicType => "同時發動另一主要屬性的精通效果".to_string(),
        }
    }
}

/// 目標池類型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetPool {
    /// 默認：敵方右側玩家
    Default,
    /// 自己
    Self_,
    /// 所有敵人
    Enemies,
    /// 所有友軍
    Allies,
    /// 所有玩家
    All,
    /// 所有其他玩家
    AllOtherPlayers,
    /// 從敵人中選擇
    ChooseFromEnemies,
    /// 從友軍中選擇
    ChooseFromAllies,
    /// 從所有玩家中選擇
    ChooseFromAll,
}

impl TargetPool {
    /// 獲取可選目標列表（player_id: 0=自己, 1=右邊敵人, 2=隊友, 3=左邊敵人）
    pub fn get_targets(&self, player_id: usize) -> Vec<usize> {
        match self {
            TargetPool::Default => vec![3], // 默認攻擊右邊的敵人
            TargetPool::Self_ => vec![0],
            TargetPool::Enemies => vec![1, 3],
            TargetPool::Allies => vec![0, 2],
            TargetPool::All => vec![0, 1, 2, 3],
            TargetPool::AllOtherPlayers => vec![1, 2, 3],
            TargetPool::ChooseFromEnemies => vec![1, 3],
            TargetPool::ChooseFromAllies => vec![0, 2],
            TargetPool::ChooseFromAll => vec![0, 1, 2, 3],
        }
    }

    /// 獲取默認目標
    pub fn get_default_target(&self, player_id: usize) -> Option<usize> {
        match self {
            TargetPool::Default => Some(3),
            TargetPool::Self_ => Some(0),
            TargetPool::Enemies => Some(3),
            TargetPool::Allies => Some(0),
            TargetPool::All => Some(0),
            TargetPool::AllOtherPlayers => Some(1),
            TargetPool::ChooseFromEnemies => Some(3),
            TargetPool::ChooseFromAllies => Some(0),
            TargetPool::ChooseFromAll => Some(0),
        }
    }

    /// 是否需要用戶選擇目標
    pub fn requires_choice(&self) -> bool {
        matches!(
            self,
            TargetPool::ChooseFromEnemies |
            TargetPool::ChooseFromAllies |
            TargetPool::ChooseFromAll
        )
    }
}

/// 法術效果組
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellEffect {
    pub target_pool: TargetPool,
    pub effect1: EffectType,
    pub effect2: Option<EffectType>,
}

impl SpellEffect {
    pub fn new(target_pool: TargetPool, effect1: EffectType) -> Self {
        Self {
            target_pool,
            effect1,
            effect2: None,
        }
    }

    pub fn with_second_effect(mut self, effect2: EffectType) -> Self {
        self.effect2 = Some(effect2);
        self
    }
}
