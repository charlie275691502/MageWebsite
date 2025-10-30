use crate::attribute::AttributeType;
use crate::card::{AttributeBolt, Card, CardDatabase, CardId, CardSide};
use crate::character::{Character, CharacterType};
use crate::effect::{EffectType, TargetPool};
use crate::player::{Player, PlayerId, TeamId};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

/// 遊戲狀態
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameState {
    WaitingForPlayers,
    InProgress,
    GameOver(TeamId),  // 獲勝隊伍
}

/// 回合階段
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TurnPhase {
    AllocateAttribute,  // 分配屬性點
    PlayCard,           // 打出卡片
    DrawCard,           // 抽卡
    TurnEnd,            // 回合結束
}

/// 遊戲主體
pub struct Game {
    pub players: Vec<Player>,
    pub card_db: CardDatabase,
    pub deck: Vec<CardId>,
    pub discard_pile: Vec<CardId>,
    pub state: GameState,
    pub current_player_index: usize,
    pub turn_phase: TurnPhase,
    pub turn_number: u32,
}

impl Game {
    pub fn new(player_names: Vec<String>, characters: Vec<CharacterType>) -> Self {
        assert!(player_names.len() >= 2 && player_names.len() <= 4,
                "Game requires 2-4 players, got {}", player_names.len());
        assert_eq!(player_names.len(), characters.len(),
                   "Player names and characters count mismatch");

        let mut players = Vec::new();
        for (i, (name, char_type)) in player_names.iter().zip(characters.iter()).enumerate() {
            let character = Character::new(*char_type);
            players.push(Player::new(i, name.clone(), character));
        }

        let card_db = CardDatabase::new();
        let mut deck = card_db.create_deck();

        // 洗牌
        let mut rng = thread_rng();
        deck.shuffle(&mut rng);

        let mut game = Self {
            players,
            card_db,
            deck,
            discard_pile: Vec::new(),
            state: GameState::WaitingForPlayers,
            current_player_index: 0,
            turn_phase: TurnPhase::AllocateAttribute,
            turn_number: 1,
        };

        // 初始發牌（每人5張）
        game.initial_deal();
        game.state = GameState::InProgress;

        game
    }

    /// 初始發牌
    fn initial_deal(&mut self) {
        for _ in 0..5 {
            for player in &mut self.players {
                if let Some(card_id) = self.deck.pop() {
                    player.draw_card(card_id);
                }
            }
        }
    }

    /// 獲取當前玩家
    pub fn current_player(&self) -> &Player {
        &self.players[self.current_player_index]
    }

    /// 獲取當前玩家（可變）
    pub fn current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.current_player_index]
    }

    /// 進入下一位玩家
    fn next_player(&mut self) {
        self.current_player_index = (self.current_player_index + 1) % self.players.len();

        // 檢查是否新的一輪
        if self.current_player_index == 0 {
            self.turn_number += 1;
        }

        // 處理玩家回合開始效果
        self.handle_turn_start();
    }

    /// 處理回合開始
    fn handle_turn_start(&mut self) {
        let current_id = self.current_player_index;
        self.players[current_id].turn_start();

        // 如果玩家死亡，跳過
        if !self.players[current_id].can_act() {
            self.next_player();
            return;
        }

        // 火Lv5效果：回合開始時對所有敵人造成1點傷害
        let has_fire_lv5 = self.players[current_id].attributes.fire >= 5;
        if has_fire_lv5 {
            let enemy_ids = self.players[current_id].enemy_ids();
            for &enemy_id in &enemy_ids {
                self.players[enemy_id].take_damage(1);
            }
        }

        // 處理生命汲取效果
        let health_drain_target = if self.players[current_id].buffs.has(crate::buff::BuffType::HealthDrain) {
            self.players[current_id].buffs.get(crate::buff::BuffType::HealthDrain)
                .and_then(|buff| buff.data)
        } else {
            None
        };

        if let Some(target_id) = health_drain_target {
            if target_id >= 0 && (target_id as usize) < self.players.len() {
                self.players[target_id as usize].take_damage(1);
                self.players[current_id].heal(1);
            }
        }

        self.turn_phase = TurnPhase::AllocateAttribute;
    }

    /// 處理回合結束
    fn handle_turn_end(&mut self) {
        let player = &mut self.players[self.current_player_index];
        player.turn_end();

        self.next_player();
    }

    /// 分配屬性點
    pub fn allocate_attribute(&mut self, attr_type: AttributeType) -> Result<(), String> {
        if self.turn_phase != TurnPhase::AllocateAttribute {
            return Err("不是分配屬性點階段".to_string());
        }

        let player = self.current_player_mut();

        // 檢查是否已達上限
        if player.attributes.get(attr_type) >= 5 {
            return Err(format!("{}屬性已達上限", attr_type.to_string()));
        }

        player.attributes.add(attr_type, 1);
        self.turn_phase = TurnPhase::PlayCard;

        Ok(())
    }

    /// 使用屬性彈（需要打出一張卡片）
    pub fn play_attribute_bolt(&mut self, card_id: CardId, attr_type: AttributeType) -> Result<(), String> {
        if self.turn_phase != TurnPhase::PlayCard {
            return Err("不是出牌階段".to_string());
        }

        let current_id = self.current_player_index;

        // 檢查是否有這張卡
        if !self.players[current_id].hand.contains(&card_id) {
            return Err("手牌中沒有這張卡".to_string());
        }

        // 獲取屬性等級
        let level = self.players[current_id].attributes.get(attr_type);
        if level == 0 {
            return Err(format!("沒有{}屬性點", attr_type.to_string()));
        }

        // 打出卡片並丟到棄牌堆
        if !self.players[current_id].discard_card(card_id) {
            return Err("打出卡片失敗".to_string());
        }

        // 屬性彈只能攻擊前一位玩家（左側）
        let target_id = self.players[current_id].left_player_id(self.players.len());

        // 檢查目標是否存活
        if self.players[target_id].is_dead {
            return Err("目標已死亡".to_string());
        }

        let bolt = AttributeBolt::new(attr_type, level);
        let mut damage = bolt.base_damage();

        // 火Lv3效果：所有屬性彈+1
        if self.players[current_id].attributes.fire >= 3 {
            damage += 1;
        }

        // 對目標造成傷害
        self.players[target_id].take_damage(damage);

        self.turn_phase = TurnPhase::DrawCard;
        Ok(())
    }

    /// 打出法術卡
    pub fn play_spell_card(&mut self, card_id: CardId, side: CardSide, targets: Vec<PlayerId>) -> Result<(), String> {
        if self.turn_phase != TurnPhase::PlayCard {
            return Err("不是出牌階段".to_string());
        }

        let current_id = self.current_player_index;

        // 檢查是否有這張卡
        if !self.players[current_id].hand.contains(&card_id) {
            return Err("手牌中沒有這張卡".to_string());
        }

        // 獲取卡片
        let card = self.card_db.get_card(card_id).ok_or("卡片不存在")?;

        // 檢查是否有要求的面（單面卡只有Top）
        if side == CardSide::Bottom && card.bottom_spell.is_none() {
            return Err("這張卡只有一個法術".to_string());
        }

        // 檢查是否滿足屬性需求
        if !card.can_play(side, &self.players[current_id].attributes) {
            return Err("不滿足屬性需求".to_string());
        }

        // 打出卡片並丟到棄牌堆
        if !self.players[current_id].discard_card(card_id) {
            return Err("打出卡片失敗".to_string());
        }

        // 執行效果
        // TODO: 實現完整的效果執行邏輯
        // let spell = match side {
        //     CardSide::Top => &card.top_spell,
        //     CardSide::Bottom => card.bottom_spell.as_ref().unwrap(),
        // };
        // self.execute_spell_effect(spell, targets)?;

        self.turn_phase = TurnPhase::DrawCard;
        Ok(())
    }

    /// 使用解放技能
    pub fn use_liberation(&mut self, targets: Vec<PlayerId>) -> Result<(), String> {
        if self.turn_phase != TurnPhase::PlayCard {
            return Err("不是出牌階段".to_string());
        }

        let current_id = self.current_player_index;
        let player = &mut self.players[current_id];

        if !player.can_use_liberation() {
            return Err("無法使用解放技能".to_string());
        }

        // 執行解放技能
        let char_type = player.character.character_type;
        player.use_liberation();

        // 執行解放效果
        match char_type {
            CharacterType::FlamePoison => {
                // 燃燒殆盡：火屬性攻擊傷害+5
                let buff = crate::buff::Buff::new(
                    crate::buff::BuffType::BurningOut,
                    crate::buff::BuffDuration::Permanent,
                );
                self.players[current_id].buffs.add(buff);
            }
            CharacterType::WoodWind => {
                // 守護木雕：所有友軍受到傷害-4，受到3次攻擊後消失
                let buff = crate::buff::Buff::new_with_data(
                    crate::buff::BuffType::GuardWoodCarving,
                    crate::buff::BuffDuration::UntilHit(3),
                    3,
                );
                self.players[current_id].buffs.add(buff.clone());
                let teammate_id = self.players[current_id].teammate_id();
                self.players[teammate_id].buffs.add(buff);
            }
            CharacterType::ThunderPoison => {
                // 鏈鎖電擊：對所有其他玩家造成10點傷害
                for i in 0..self.players.len() {
                    if i != current_id {
                        self.players[i].take_damage(10);
                    }
                }
            }
            CharacterType::WaterWind => {
                // 颶風之眼：回合開始時回復7點生命，持續4回合
                let buff = crate::buff::Buff::new(
                    crate::buff::BuffType::Regeneration,
                    crate::buff::BuffDuration::Turns(4),
                );
                self.players[current_id].buffs.add(buff);
            }
        }

        self.turn_phase = TurnPhase::DrawCard;
        Ok(())
    }

    /// 抽卡
    pub fn draw_card(&mut self) -> Result<(), String> {
        if self.turn_phase != TurnPhase::DrawCard {
            return Err("不是抽卡階段".to_string());
        }

        // 如果牌組為空，洗回棄牌堆
        if self.deck.is_empty() && !self.discard_pile.is_empty() {
            self.deck = self.discard_pile.drain(..).collect();
            let mut rng = thread_rng();
            self.deck.shuffle(&mut rng);
        }

        if let Some(card_id) = self.deck.pop() {
            self.current_player_mut().draw_card(card_id);
        }

        self.turn_phase = TurnPhase::TurnEnd;
        self.handle_turn_end();

        Ok(())
    }

    /// 檢查遊戲是否結束
    pub fn check_game_over(&mut self) -> bool {
        let team0_alive = self.players.iter().any(|p| p.team == TeamId::Team0 && p.is_alive());
        let team1_alive = self.players.iter().any(|p| p.team == TeamId::Team1 && p.is_alive());

        // 檢查是否有隊伍全滅
        if !team0_alive {
            self.state = GameState::GameOver(TeamId::Team1);
            return true;
        }

        if !team1_alive {
            self.state = GameState::GameOver(TeamId::Team0);
            return true;
        }

        false
    }

    /// 獲取獲勝隊伍
    pub fn get_winner(&self) -> Option<TeamId> {
        match self.state {
            GameState::GameOver(team) => Some(team),
            _ => None,
        }
    }

    /// 執行一個完整回合（用於測試）
    pub fn play_turn(&mut self) {
        if self.state != GameState::InProgress {
            return;
        }

        let player_id = self.current_player_index;
        let player = &self.players[player_id];

        println!("\n===== 回合 {} - {} 的回合 =====", self.turn_number, player.name);
        println!("血量: {}/{}", player.hp, player.max_hp);
        println!("護盾: {}", player.shield);
        println!("屬性: 火{} 木{} 雷{} 水{} 風{} 毒{}",
            player.attributes.fire,
            player.attributes.wood,
            player.attributes.thunder,
            player.attributes.water,
            player.attributes.wind,
            player.attributes.poison,
        );
        println!("手牌數: {}", player.hand.len());

        if !player.can_act() {
            println!("玩家無法行動");
            self.handle_turn_end();
            return;
        }

        // 簡單的AI：隨機分配屬性，使用屬性彈攻擊
        let attr = *[
            AttributeType::Fire,
            AttributeType::Wood,
            AttributeType::Thunder,
            AttributeType::Water,
            AttributeType::Wind,
            AttributeType::Poison,
        ]
        .choose(&mut thread_rng())
        .unwrap();

        let _ = self.allocate_attribute(attr);
        println!("分配屬性點: {}", attr.to_string());

        // TODO: CLI模式需要重構以支持卡牌系統
        // 使用屬性彈攻擊需要選擇一張卡片
        // let target = (player_id + 3) % 4;  // 右側敵人
        // let _ = self.play_attribute_bolt(card_id, AttributeType::Fire);
        // println!("使用屬性彈攻擊 {}", self.players[target].name);

        let _ = self.draw_card();
        println!("抽卡");

        self.check_game_over();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_initialization() {
        let names = vec!["P1".to_string(), "P2".to_string(), "P3".to_string(), "P4".to_string()];
        let chars = vec![
            CharacterType::FlamePoison,
            CharacterType::WoodWind,
            CharacterType::ThunderPoison,
            CharacterType::WaterWind,
        ];

        let game = Game::new(names, chars);

        assert_eq!(game.players.len(), 4);
        assert_eq!(game.state, GameState::InProgress);

        // 檢查初始手牌
        for player in &game.players {
            assert_eq!(player.hand.len(), 4);
        }
    }
}
