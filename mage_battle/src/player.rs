use crate::attribute::AttributePoints;
use crate::buff::BuffList;
use crate::card::{AttributeBolt, CardId};
use crate::character::Character;
use serde::{Deserialize, Serialize};

/// 玩家ID（0-3）
pub type PlayerId = usize;

/// 隊伍ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeamId {
    Team0,  // 玩家0和2
    Team1,  // 玩家1和3
}

impl TeamId {
    pub fn from_player_id(player_id: PlayerId) -> Self {
        match player_id {
            0 | 2 => TeamId::Team0,
            1 | 3 => TeamId::Team1,
            _ => panic!("Invalid player_id"),
        }
    }

    pub fn get_teammate_id(player_id: PlayerId) -> PlayerId {
        match player_id {
            0 => 2,
            1 => 3,
            2 => 0,
            3 => 1,
            _ => panic!("Invalid player_id"),
        }
    }

    pub fn get_enemies(player_id: PlayerId) -> Vec<PlayerId> {
        match player_id {
            0 | 2 => vec![1, 3],
            1 | 3 => vec![0, 2],
            _ => panic!("Invalid player_id"),
        }
    }

    pub fn is_teammate(player_a: PlayerId, player_b: PlayerId) -> bool {
        Self::from_player_id(player_a) == Self::from_player_id(player_b)
    }
}

/// 玩家狀態
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub character: Character,
    pub team: TeamId,

    // 生命值
    pub hp: i32,
    pub max_hp: i32,

    // 護盾
    pub shield: u32,

    // 屬性點
    pub attributes: AttributePoints,

    // Buff列表
    pub buffs: BuffList,

    // 手牌
    pub hand: Vec<CardId>,

    // 棄牌堆
    pub discard_pile: Vec<CardId>,

    // 死亡狀態
    pub is_dead: bool,
    pub death_turns: u8,  // 已死亡回合數

    // 回合狀態
    pub has_acted_this_turn: bool,
}

impl Player {
    pub fn new(id: PlayerId, name: String, character: Character) -> Self {
        let team = TeamId::from_player_id(id);
        let mut attributes = AttributePoints::new();

        // 設置角色初始屬性
        let initial_attrs = character.character_type.initial_attributes();
        attributes.from_array(initial_attrs);

        Self {
            id,
            name,
            character,
            team,
            hp: 50,
            max_hp: 50,
            shield: 0,
            attributes,
            buffs: BuffList::new(),
            hand: Vec::new(),
            discard_pile: Vec::new(),
            is_dead: false,
            death_turns: 0,
            has_acted_this_turn: false,
        }
    }

    /// 檢查玩家是否存活
    pub fn is_alive(&self) -> bool {
        !self.is_dead
    }

    /// 造成傷害
    pub fn take_damage(&mut self, mut damage: u32) -> u32 {
        // 檢查免疫
        if self.buffs.is_immune_to_damage() {
            return 0;
        }

        // 木Lv5效果：卡片傷害-1
        if self.attributes.wood >= 5 && damage > 0 {
            damage = damage.saturating_sub(1);
        }

        // 守護木雕效果：受到傷害-4
        if self.buffs.has(crate::buff::BuffType::GuardWoodCarving) {
            damage = damage.saturating_sub(4);

            // 減少守護木雕的剩餘次數
            if let Some(buff) = self.buffs.get_mut(crate::buff::BuffType::GuardWoodCarving) {
                if let Some(hits) = buff.data.as_mut() {
                    *hits -= 1;
                    if *hits <= 0 {
                        self.buffs.remove(crate::buff::BuffType::GuardWoodCarving);
                    }
                }
            }
        }

        let actual_damage = if damage > self.shield {
            let remaining = damage - self.shield;
            self.shield = 0;
            self.hp -= remaining as i32;
            remaining
        } else {
            self.shield -= damage;
            0
        };

        // 檢查是否死亡
        if self.hp <= 0 {
            self.hp = 0;
            self.die();
        }

        actual_damage
    }

    /// 回復生命
    pub fn heal(&mut self, amount: u32) -> u32 {
        if !self.buffs.can_heal_or_shield() {
            return 0;
        }

        let old_hp = self.hp;
        self.hp = (self.hp + amount as i32).min(self.max_hp);
        (self.hp - old_hp) as u32
    }

    /// 獲得護盾
    pub fn gain_shield(&mut self, amount: u32) -> u32 {
        if !self.buffs.can_heal_or_shield() {
            return 0;
        }

        self.shield += amount;
        amount
    }

    /// 死亡
    fn die(&mut self) {
        self.is_dead = true;
        self.death_turns = 0;
        self.hand.clear();
        self.buffs.clear_all();
    }

    /// 復活
    pub fn revive(&mut self) {
        if self.is_dead {
            self.is_dead = false;
            self.death_turns = 0;
            self.hp = self.max_hp / 2;  // 復活時50%血量
        }
    }

    /// 增加死亡回合數
    pub fn increment_death_turns(&mut self) {
        if self.is_dead {
            self.death_turns += 1;
            // 死亡2回合後復活
            if self.death_turns >= 2 {
                self.revive();
            }
        }
    }

    /// 添加卡片到手牌
    pub fn draw_card(&mut self, card_id: CardId) {
        self.hand.push(card_id);
    }

    /// 從手牌移除卡片
    pub fn remove_card_from_hand(&mut self, card_id: CardId) -> bool {
        if let Some(pos) = self.hand.iter().position(|&id| id == card_id) {
            self.hand.remove(pos);
            true
        } else {
            false
        }
    }

    /// 打出卡片並丟到棄牌堆
    pub fn discard_card(&mut self, card_id: CardId) -> bool {
        if self.remove_card_from_hand(card_id) {
            self.discard_pile.push(card_id);
            true
        } else {
            false
        }
    }

    /// 隨機丟棄一張手牌
    pub fn discard_random_card(&mut self) -> Option<CardId> {
        if self.hand.is_empty() {
            return None;
        }

        let index = rand::random::<usize>() % self.hand.len();
        Some(self.hand.remove(index))
    }

    /// 獲取手牌上限
    pub fn hand_limit(&self) -> usize {
        4 + self.character.extra_hand_size()
    }

    /// 檢查是否可以行動
    pub fn can_act(&self) -> bool {
        self.is_alive() && self.buffs.can_act()
    }

    /// 檢查是否可以使用解放
    pub fn can_use_liberation(&self) -> bool {
        self.is_alive()
            && self.buffs.can_liberate()
            && self.character.can_liberate(&self.attributes)
    }

    /// 使用解放技能
    pub fn use_liberation(&mut self) -> bool {
        if self.can_use_liberation() {
            self.character.liberate();
            true
        } else {
            false
        }
    }

    /// 回合開始處理
    pub fn turn_start(&mut self) {
        self.has_acted_this_turn = false;

        if !self.is_alive() {
            self.increment_death_turns();
            return;
        }

        // 處理燃燒殆盡效果
        if self.buffs.has(crate::buff::BuffType::BurningOut) {
            if !self.attributes.remove(crate::attribute::AttributeType::Fire, 1) {
                self.buffs.remove(crate::buff::BuffType::BurningOut);
            }
        }

        // 處理再生效果
        if self.buffs.has(crate::buff::BuffType::Regeneration) {
            self.heal(7);
        }

        // 處理生命汲取效果
        if self.buffs.has(crate::buff::BuffType::HealthDrain) {
            // 這個需要在game中處理，因為涉及到其他玩家
        }

        // 火Lv5效果：回合開始時對所有敵人造成1點傷害
        // 這個也需要在game中處理
    }

    /// 回合結束處理
    pub fn turn_end(&mut self) {
        self.has_acted_this_turn = true;

        // 更新buff持續時間
        self.buffs.tick_all();
    }

    /// 獲取右側玩家ID
    pub fn right_player_id(&self) -> PlayerId {
        (self.id + 1) % 4
    }

    /// 獲取左側玩家ID
    pub fn left_player_id(&self) -> PlayerId {
        (self.id + 3) % 4
    }

    /// 獲取隊友ID
    pub fn teammate_id(&self) -> PlayerId {
        TeamId::get_teammate_id(self.id)
    }

    /// 獲取敵人ID列表
    pub fn enemy_ids(&self) -> Vec<PlayerId> {
        TeamId::get_enemies(self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character::CharacterType;

    #[test]
    fn test_player_damage() {
        let character = Character::new(CharacterType::FlamePoison);
        let mut player = Player::new(0, "Test".to_string(), character);

        player.take_damage(10);
        assert_eq!(player.hp, 40);

        player.gain_shield(5);
        player.take_damage(10);
        assert_eq!(player.shield, 0);
        assert_eq!(player.hp, 35);
    }

    #[test]
    fn test_player_death_and_revival() {
        let character = Character::new(CharacterType::FlamePoison);
        let mut player = Player::new(0, "Test".to_string(), character);

        player.take_damage(50);
        assert!(player.is_dead);
        assert_eq!(player.death_turns, 0);

        player.increment_death_turns();
        assert_eq!(player.death_turns, 1);
        assert!(player.is_dead);

        player.increment_death_turns();
        assert_eq!(player.death_turns, 0);
        assert!(!player.is_dead);
        assert_eq!(player.hp, 25);  // 50% of max_hp
    }

    #[test]
    fn test_team_relationships() {
        assert_eq!(TeamId::from_player_id(0), TeamId::Team0);
        assert_eq!(TeamId::from_player_id(2), TeamId::Team0);
        assert_eq!(TeamId::from_player_id(1), TeamId::Team1);
        assert_eq!(TeamId::from_player_id(3), TeamId::Team1);

        assert!(TeamId::is_teammate(0, 2));
        assert!(TeamId::is_teammate(1, 3));
        assert!(!TeamId::is_teammate(0, 1));

        assert_eq!(TeamId::get_teammate_id(0), 2);
        assert_eq!(TeamId::get_enemies(0), vec![1, 3]);
    }
}
