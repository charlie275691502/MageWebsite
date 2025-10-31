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

        // 火Lv5效果：回合開始時對所有敵人造成1點技能傷害（不受木Lv5影響）
        let has_fire_lv5 = self.players[current_id].attributes.fire >= 5;
        if has_fire_lv5 {
            let enemy_ids = self.players[current_id].enemy_ids();
            for &enemy_id in &enemy_ids {
                self.players[enemy_id].take_damage(1, crate::damage::DamageType::Skill);
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
                self.players[target_id as usize].take_damage(1, crate::damage::DamageType::Skill);
                self.players[current_id].heal(1);
            }
        }

        self.turn_phase = TurnPhase::AllocateAttribute;
    }

    /// 獲取最遠的存活敵人
    /// 根據回合順序向前計算距離，返回距離最遠的存活敵人
    fn get_furthest_alive_enemy(&self, player_id: PlayerId) -> Option<PlayerId> {
        let current_team = self.players[player_id].team;
        let num_players = self.players.len();

        // 計算每個敵人的距離（在回合順序中向前的距離）
        let mut furthest_enemy = None;
        let mut max_distance = 0;

        // Iterate through all players and find enemies (different team)
        for (enemy_id, enemy) in self.players.iter().enumerate() {
            // Skip if same player, dead, or same team
            if enemy_id == player_id || !enemy.is_alive() || enemy.team == current_team {
                continue;
            }

            // 計算順時針距離
            let distance = if enemy_id > player_id {
                enemy_id - player_id
            } else {
                num_players - player_id + enemy_id
            };

            if distance > max_distance {
                max_distance = distance;
                furthest_enemy = Some(enemy_id);
            }
        }

        furthest_enemy
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

        // 屬性彈攻擊最遠的存活敵人
        let target_id = self.get_furthest_alive_enemy(current_id)
            .ok_or("沒有存活的敵人".to_string())?;

        let bolt = AttributeBolt::new(attr_type, level);
        let mut damage = bolt.base_damage();

        // 屬性彈也有屬性附魔，應用專精加成
        let caster_fire = self.players[current_id].attributes.fire;
        let caster_thunder = self.players[current_id].attributes.thunder;
        let caster_wood = self.players[current_id].attributes.wood;

        // 木Lv3專精：使用木屬性彈時，減少1點生命並獲得1點護盾（不受木Lv5影響，不受護盾影響）
        if attr_type == AttributeType::Wood && caster_wood >= 3 {
            self.players[current_id].hp -= 1;
            if self.players[current_id].hp <= 0 {
                self.players[current_id].hp = 0;
                self.players[current_id].is_dead = true;
                self.players[current_id].death_turns = 0;
                self.players[current_id].hand.clear();
                self.players[current_id].buffs.clear_all();
            }
            self.players[current_id].shield += 1;
        }

        // 火Lv3: 所有屬性彈+1
        if caster_fire >= 3 {
            damage += 1;
        }

        // 雷屬性專精
        if attr_type == AttributeType::Thunder {
            // 雷Lv3: 雷屬性法術傷害+1
            if caster_thunder >= 3 {
                damage += 1;
            }
            // 雷Lv5: 雷屬性法術傷害+2 (合計+3)
            if caster_thunder >= 5 {
                damage += 2;
            }
        }

        // NOTE: Wood Lv5 reduction is now handled in take_damage()

        // 對目標造成傷害（屬性彈視為法術，受木Lv5影響）
        self.players[target_id].take_damage(damage, crate::damage::DamageType::Spell);

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

        // 執行效果（clone spell 以避免借用衝突）
        let spell = match side {
            CardSide::Top => card.top_spell.clone(),
            CardSide::Bottom => card.bottom_spell.as_ref().unwrap().clone(),
        };
        self.execute_spell_effect(current_id, &spell, targets)?;

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
                // 鏈鎖電擊：對所有其他玩家造成10點法術傷害
                for i in 0..self.players.len() {
                    if i != current_id {
                        self.players[i].take_damage(10, crate::damage::DamageType::Spell);
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

    /// 執行法術效果（包含屬性專精加成）
    fn execute_spell_effect(
        &mut self,
        caster_id: PlayerId,
        spell: &crate::card::SpellSide,
        targets: Vec<PlayerId>,
    ) -> Result<(), String> {
        // 獲取法術的所有屬性需求
        let enchantments: Vec<AttributeType> = spell
            .requirements
            .iter()
            .map(|(attr, _)| *attr)
            .collect();

        // 木Lv3專精：使用木屬性法術時，減少1點生命並獲得1點護盾（不受木Lv5影響，不受護盾影響）
        let has_wood = enchantments.contains(&AttributeType::Wood);
        let caster_wood = self.players[caster_id].attributes.wood;
        if has_wood && caster_wood >= 3 {
            self.players[caster_id].hp -= 1;
            if self.players[caster_id].hp <= 0 {
                self.players[caster_id].hp = 0;
                self.players[caster_id].is_dead = true;
                self.players[caster_id].death_turns = 0;
                self.players[caster_id].hand.clear();
                self.players[caster_id].buffs.clear_all();
            }
            self.players[caster_id].shield += 1;
        }

        // 執行 effect1
        self.apply_effect(caster_id, &spell.effect.effect1, &targets, &enchantments)?;

        // 執行 effect2（如果存在）
        if let Some(effect2) = &spell.effect.effect2 {
            self.apply_effect(caster_id, effect2, &targets, &enchantments)?;
        }

        Ok(())
    }

    /// 應用單個效果
    fn apply_effect(
        &mut self,
        caster_id: PlayerId,
        effect: &EffectType,
        targets: &[PlayerId],
        enchantments: &[AttributeType],
    ) -> Result<(), String> {
        let caster_attrs = &self.players[caster_id].attributes;

        match effect {
            EffectType::Damage(base_damage) => {
                self.apply_damage_effect(caster_id, *base_damage, targets, enchantments)
            }
            EffectType::IncreaseDamage(bonus) => {
                // 造成屬性等級+N點傷害
                // 使用主要屬性等級（需求最高的屬性）
                let main_attr_level = enchantments
                    .iter()
                    .map(|attr| caster_attrs.get(*attr) as u32)
                    .max()
                    .unwrap_or(0);
                let total_damage = main_attr_level + bonus;
                self.apply_damage_effect(caster_id, total_damage, targets, enchantments)
            }
            EffectType::Heal(amount) => {
                self.apply_heal_effect(caster_id, *amount, targets, enchantments)
            }
            EffectType::HealSelf(amount) => {
                self.apply_heal_effect(caster_id, *amount, &[caster_id], enchantments)
            }
            EffectType::Shield(amount) => {
                self.apply_shield_effect(caster_id, *amount, targets, enchantments)
            }
            EffectType::ShieldSelf(amount) => {
                self.apply_shield_effect(caster_id, *amount, &[caster_id], enchantments)
            }
            _ => {
                // 其他效果類型尚未實現
                Ok(())
            }
        }
    }

    /// 應用傷害效果（包含屬性專精加成）
    fn apply_damage_effect(
        &mut self,
        caster_id: PlayerId,
        mut base_damage: u32,
        targets: &[PlayerId],
        enchantments: &[AttributeType],
    ) -> Result<(), String> {
        // 複製施法者的屬性值以避免借用衝突
        let caster_fire = self.players[caster_id].attributes.fire;
        let caster_thunder = self.players[caster_id].attributes.thunder;

        // 應用屬性專精加成
        for attr in enchantments {
            match attr {
                AttributeType::Fire => {
                    // 火Lv3: 所有屬性彈+1 (也適用於火屬性法術)
                    if caster_fire >= 3 {
                        base_damage += 1;
                    }
                }
                AttributeType::Thunder => {
                    // 雷Lv3: 雷屬性卡片傷害+1
                    if caster_thunder >= 3 {
                        base_damage += 1;
                    }
                    // 雷Lv5: 雷屬性卡片傷害+2 (合計+3)
                    if caster_thunder >= 5 {
                        base_damage += 2;
                    }
                }
                _ => {}
            }
        }

        // 對每個目標造成傷害
        // NOTE: Wood Lv5 reduction is now handled in take_damage(), no need to calculate here
        for &target_id in targets {
            if target_id >= self.players.len() {
                continue;
            }

            if self.players[target_id].is_dead {
                continue;
            }

            self.players[target_id].take_damage(base_damage, crate::damage::DamageType::Spell);
        }

        Ok(())
    }

    /// 應用治療效果（包含屬性專精加成）
    fn apply_heal_effect(
        &mut self,
        caster_id: PlayerId,
        heal_amount: u32,
        targets: &[PlayerId],
        enchantments: &[AttributeType],
    ) -> Result<(), String> {
        // 複製施法者的屬性值以避免借用衝突
        let caster_water = self.players[caster_id].attributes.water;
        let teammate_id = self.players[caster_id].teammate_id();

        // 檢查是否有水屬性附魔
        let has_water = enchantments.contains(&AttributeType::Water);

        // 應用治療
        for &target_id in targets {
            if target_id >= self.players.len() {
                continue;
            }

            let mut actual_heal = heal_amount;

            // 水Lv3: 使用水屬卡片時，回復自身1點生命
            if has_water && caster_water >= 3 && target_id == caster_id {
                actual_heal += 1;
            }

            self.players[target_id].heal(actual_heal);
        }

        // 水Lv5: 使用水屬卡片時，回復自己與隊友1點生命
        if has_water && caster_water >= 5 && teammate_id < self.players.len() {
            // 如果隊友不在目標列表中，額外回復
            if !targets.contains(&teammate_id) {
                self.players[teammate_id].heal(1);
            }
        }

        Ok(())
    }

    /// 應用護盾效果
    fn apply_shield_effect(
        &mut self,
        caster_id: PlayerId,
        shield_amount: u32,
        targets: &[PlayerId],
        _enchantments: &[AttributeType],
    ) -> Result<(), String> {
        // 木Lv3專精已在execute_spell_effect中處理

        // 應用護盾
        for &target_id in targets {
            if target_id >= self.players.len() {
                continue;
            }

            self.players[target_id].shield += shield_amount;
        }

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

        // 檢查初始手牌 (Battle_Logic.txt Line 343: "Starting Hand: 5 cards per player")
        for player in &game.players {
            assert_eq!(player.hand.len(), 5);
        }
    }
}
