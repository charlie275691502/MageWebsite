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
        use crate::buff::{Buff, BuffDuration};
        use crate::damage::DamageType;

        let caster_attrs = &self.players[caster_id].attributes;

        match effect {
            // === 傷害類 ===
            EffectType::Damage(base_damage) => {
                self.apply_damage_effect(caster_id, *base_damage, targets, enchantments)
            }
            EffectType::DoubleDamage(base_damage) => {
                // 造成2次傷害
                self.apply_damage_effect(caster_id, *base_damage, targets, enchantments)?;
                self.apply_damage_effect(caster_id, *base_damage, targets, enchantments)
            }
            EffectType::TripleDamage(base_damage) => {
                // 造成3次傷害
                self.apply_damage_effect(caster_id, *base_damage, targets, enchantments)?;
                self.apply_damage_effect(caster_id, *base_damage, targets, enchantments)?;
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
            EffectType::SubMTDDamage(multiplier) => {
                // 造成等同於目標N倍輔助屬性點數和的傷害
                for &target_id in targets {
                    if target_id >= self.players.len() || self.players[target_id].is_dead {
                        continue;
                    }
                    let target_attrs = &self.players[target_id].attributes;
                    let sub_attrs_sum = target_attrs.wind + target_attrs.poison;
                    let damage = sub_attrs_sum as u32 * multiplier;
                    self.players[target_id].take_damage(damage, DamageType::Spell);
                }
                Ok(())
            }
            EffectType::ShieldDDamage(base_damage) => {
                // 造成傷害，每點護盾使傷害+2
                let caster_shield = self.players[caster_id].shield;
                let bonus_damage = caster_shield * 2;
                let total_damage = base_damage + bonus_damage;
                self.apply_damage_effect(caster_id, total_damage, targets, enchantments)
            }

            // === 屬性重置類 ===
            EffectType::FlameDeallocation => {
                self.players[caster_id].attributes.fire = 0;
                Ok(())
            }
            EffectType::WoodDeallocation => {
                self.players[caster_id].attributes.wood = 0;
                Ok(())
            }
            EffectType::SparkDeallocation => {
                self.players[caster_id].attributes.thunder = 0;
                Ok(())
            }
            EffectType::WaterDeallocation => {
                self.players[caster_id].attributes.water = 0;
                Ok(())
            }

            // === 治療類 ===
            EffectType::Heal(amount) => {
                self.apply_heal_effect(caster_id, *amount, targets, enchantments)
            }
            EffectType::HealSelf(amount) => {
                self.apply_heal_effect(caster_id, *amount, &[caster_id], enchantments)
            }

            // === 護盾類 ===
            EffectType::Shield(amount) => {
                self.apply_shield_effect(caster_id, *amount, targets, enchantments)
            }
            EffectType::ShieldSelf(amount) => {
                self.apply_shield_effect(caster_id, *amount, &[caster_id], enchantments)
            }
            EffectType::DestroyAllShield => {
                // 移除所有玩家的護盾
                for player in &mut self.players {
                    player.shield = 0;
                }
                Ok(())
            }

            // === Buff類 ===
            EffectType::BuffOne(buff_type) => {
                for &target_id in targets {
                    if target_id >= self.players.len() {
                        continue;
                    }
                    // 檢查免疫負面效果
                    if buff_type.is_debuff() && self.players[target_id].buffs.is_immune_to_debuff() {
                        continue;
                    }
                    self.players[target_id].buffs.add(Buff::new(*buff_type, BuffDuration::Turns(1)));
                }
                Ok(())
            }
            EffectType::BuffForever(buff_type) => {
                for &target_id in targets {
                    if target_id >= self.players.len() {
                        continue;
                    }
                    if buff_type.is_debuff() && self.players[target_id].buffs.is_immune_to_debuff() {
                        continue;
                    }
                    self.players[target_id].buffs.add(Buff::new(*buff_type, BuffDuration::Permanent));
                }
                Ok(())
            }
            EffectType::BuffSelfOne(buff_type) => {
                self.players[caster_id].buffs.add(Buff::new(*buff_type, BuffDuration::Turns(1)));
                Ok(())
            }
            EffectType::BuffSelfTwo(buff_type) => {
                self.players[caster_id].buffs.add(Buff::new(*buff_type, BuffDuration::Turns(2)));
                Ok(())
            }
            EffectType::BuffSelfFour(buff_type) => {
                self.players[caster_id].buffs.add(Buff::new(*buff_type, BuffDuration::Turns(4)));
                Ok(())
            }
            EffectType::BuffSelfSix(buff_type) => {
                self.players[caster_id].buffs.add(Buff::new(*buff_type, BuffDuration::Turns(6)));
                Ok(())
            }
            EffectType::HealthDrain => {
                // 賦予目標永久生命汲取
                for &target_id in targets {
                    if target_id >= self.players.len() {
                        continue;
                    }
                    if self.players[target_id].buffs.is_immune_to_debuff() {
                        continue;
                    }
                    // 生命汲取需要記錄目標ID
                    self.players[caster_id].buffs.add(Buff::new_with_data(
                        crate::buff::BuffType::HealthDrain,
                        BuffDuration::Permanent,
                        target_id as i32,
                    ));
                    // 目標獲得寄主debuff
                    self.players[target_id].buffs.add(Buff::new(
                        crate::buff::BuffType::HealthDrainTarget,
                        BuffDuration::Permanent,
                    ));
                }
                Ok(())
            }
            EffectType::GuardWoodCarving(buff_type) => {
                // 賦予友軍守護木雕（受到3次攻擊消失）
                for &target_id in targets {
                    if target_id >= self.players.len() {
                        continue;
                    }
                    self.players[target_id].buffs.add(Buff::new_with_data(
                        *buff_type,
                        BuffDuration::UntilHit(3),
                        3, // 剩餘3次
                    ));
                }
                Ok(())
            }
            EffectType::RemoveAllBuff => {
                // 解除所有玩家的異常狀態
                for player in &mut self.players {
                    player.buffs.clear_debuffs();
                }
                Ok(())
            }

            // === 屬性點操作類 ===
            EffectType::GainAPSelf(amount) => {
                // 獲得N點屬性點 - 這需要在前端處理選擇
                // 這裡只是標記效果，實際分配在互動階段完成
                Ok(())
            }
            EffectType::MoveAP(amount) => {
                // 移動目標N點屬性點 - 需要互動
                Ok(())
            }
            EffectType::MoveAPSelf(amount) => {
                // 移動自身N點屬性點 - 需要互動
                Ok(())
            }
            EffectType::RemoveMainMTAP(amount) => {
                // 移除目標N點主要屬性點並隨機捨棄其一張手牌
                for &target_id in targets {
                    if target_id >= self.players.len() || self.players[target_id].is_dead {
                        continue;
                    }
                    let target = &mut self.players[target_id];

                    // 移除主要屬性點（火木雷水）
                    let mut remaining = *amount;
                    for attr_type in &[AttributeType::Fire, AttributeType::Wood,
                                      AttributeType::Thunder, AttributeType::Water] {
                        if remaining == 0 {
                            break;
                        }
                        let current = target.attributes.get(*attr_type);
                        let to_remove = current.min(remaining as u8);
                        target.attributes.remove(*attr_type, to_remove);
                        remaining -= to_remove as u32;
                    }

                    // 隨機捨棄一張手牌
                    target.discard_random_card();
                }
                Ok(())
            }
            EffectType::RemoveAP(amount) => {
                // 移除目標N點屬性點（任意屬性）
                for &target_id in targets {
                    if target_id >= self.players.len() || self.players[target_id].is_dead {
                        continue;
                    }
                    let target = &mut self.players[target_id];

                    // 按順序移除所有屬性
                    let mut remaining = *amount;
                    for attr_type in &[AttributeType::Fire, AttributeType::Wood,
                                      AttributeType::Thunder, AttributeType::Water,
                                      AttributeType::Wind, AttributeType::Poison] {
                        if remaining == 0 {
                            break;
                        }
                        let current = target.attributes.get(*attr_type);
                        let to_remove = current.min(remaining as u8);
                        target.attributes.remove(*attr_type, to_remove);
                        remaining -= to_remove as u32;
                    }
                }
                Ok(())
            }

            // === 卡牌操作類 ===
            EffectType::GetAoyiSelf => {
                // 將棄牌堆中隨機一張奧義卡加入手牌
                let aoyi_cards: Vec<CardId> = self.players[caster_id]
                    .discard_pile
                    .iter()
                    .copied()
                    .filter(|&card_id| {
                        if let Some(card) = self.card_db.get_card(card_id) {
                            card.top_spell.is_aoyi || card.bottom_spell.as_ref().map_or(false, |s| s.is_aoyi)
                        } else {
                            false
                        }
                    })
                    .collect();

                if let Some(&card_id) = aoyi_cards.choose(&mut thread_rng()) {
                    // 從棄牌堆移除並加入手牌
                    self.players[caster_id].discard_pile.retain(|&id| id != card_id);
                    self.players[caster_id].draw_card(card_id);
                }
                Ok(())
            }
            EffectType::RandomDiscard => {
                // 捨棄目標隨機一張手牌
                for &target_id in targets {
                    if target_id >= self.players.len() || self.players[target_id].is_dead {
                        continue;
                    }
                    if let Some(card_id) = self.players[target_id].discard_random_card() {
                        self.players[target_id].discard_pile.push(card_id);
                    }
                }
                Ok(())
            }
            EffectType::ViewTopDrawPile => {
                // 檢視公牌最上面的卡片 - 需要互動
                Ok(())
            }
            EffectType::ViewAndDiscard(_count) => {
                // 檢視目標手牌並捨棄指定牌 - 需要互動
                Ok(())
            }
            EffectType::DiscardAndDraw => {
                // 可捨棄自身任意數量之手牌，並抽相同數量 - 需要互動
                Ok(())
            }

            // === 特殊類 ===
            EffectType::Dearouse => {
                // 將目標角色改為未解放狀態
                for &target_id in targets {
                    if target_id >= self.players.len() || self.players[target_id].is_dead {
                        continue;
                    }
                    self.players[target_id].character.is_liberated = false;
                }
                Ok(())
            }
            EffectType::AddOneMagicType => {
                // 同時發動另一主要屬性的精通效果 - 需要互動選擇屬性
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
    use crate::buff::{Buff, BuffDuration, BuffType};

    fn setup_test_game() -> Game {
        let names = vec!["P1".to_string(), "P2".to_string(), "P3".to_string(), "P4".to_string()];
        let chars = vec![
            CharacterType::FlamePoison,
            CharacterType::WoodWind,
            CharacterType::ThunderPoison,
            CharacterType::WaterWind,
        ];
        Game::new(names, chars)
    }

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

    // === 傷害類效果測試 ===

    #[test]
    fn test_effect_damage() {
        let mut game = setup_test_game();
        let target = 1;
        let initial_hp = game.players[target].hp;

        game.apply_effect(0, &EffectType::Damage(10), &[target], &[]).unwrap();

        assert_eq!(game.players[target].hp, initial_hp - 10);
    }

    #[test]
    fn test_effect_double_damage() {
        let mut game = setup_test_game();
        let target = 1;
        let initial_hp = game.players[target].hp;

        game.apply_effect(0, &EffectType::DoubleDamage(5), &[target], &[]).unwrap();

        // Should apply 5 damage twice = 10 total
        assert_eq!(game.players[target].hp, initial_hp - 10);
    }

    #[test]
    fn test_effect_triple_damage() {
        let mut game = setup_test_game();
        let target = 1;
        let initial_hp = game.players[target].hp;

        game.apply_effect(0, &EffectType::TripleDamage(3), &[target], &[]).unwrap();

        // Should apply 3 damage three times = 9 total
        assert_eq!(game.players[target].hp, initial_hp - 9);
    }

    #[test]
    fn test_effect_increase_damage() {
        let mut game = setup_test_game();
        let caster = 0;
        let target = 1;

        // Set fire attribute to level 3
        game.players[caster].attributes.fire = 3;
        let initial_hp = game.players[target].hp;

        // IncreaseDamage(2) with fire level 3 = 3 + 2 = 5 damage
        // Plus fire Lv3 proficiency adds +1 damage = 6 total
        game.apply_effect(caster, &EffectType::IncreaseDamage(2), &[target], &[AttributeType::Fire]).unwrap();

        assert_eq!(game.players[target].hp, initial_hp - 6);
    }

    #[test]
    fn test_effect_sub_mtd_damage() {
        let mut game = setup_test_game();
        let target = 1;

        // Set target's wind and poison attributes
        game.players[target].attributes.wind = 2;
        game.players[target].attributes.poison = 3;
        let initial_hp = game.players[target].hp;

        // SubMTDDamage(2) with wind(2) + poison(3) = 5 * 2 = 10 damage
        game.apply_effect(0, &EffectType::SubMTDDamage(2), &[target], &[]).unwrap();

        assert_eq!(game.players[target].hp, initial_hp - 10);
    }

    #[test]
    fn test_effect_shield_d_damage() {
        let mut game = setup_test_game();
        let caster = 0;
        let target = 1;

        // Give caster 5 shield
        game.players[caster].shield = 5;
        let initial_hp = game.players[target].hp;

        // ShieldDDamage(3) with 5 shield = 3 + (5 * 2) = 13 damage
        game.apply_effect(caster, &EffectType::ShieldDDamage(3), &[target], &[]).unwrap();

        assert_eq!(game.players[target].hp, initial_hp - 13);
    }

    // === 屬性重置類效果測試 ===

    #[test]
    fn test_effect_flame_deallocation() {
        let mut game = setup_test_game();
        let caster = 0;

        game.players[caster].attributes.fire = 5;
        game.apply_effect(caster, &EffectType::FlameDeallocation, &[], &[]).unwrap();

        assert_eq!(game.players[caster].attributes.fire, 0);
    }

    #[test]
    fn test_effect_wood_deallocation() {
        let mut game = setup_test_game();
        let caster = 0;

        game.players[caster].attributes.wood = 4;
        game.apply_effect(caster, &EffectType::WoodDeallocation, &[], &[]).unwrap();

        assert_eq!(game.players[caster].attributes.wood, 0);
    }

    #[test]
    fn test_effect_spark_deallocation() {
        let mut game = setup_test_game();
        let caster = 0;

        game.players[caster].attributes.thunder = 3;
        game.apply_effect(caster, &EffectType::SparkDeallocation, &[], &[]).unwrap();

        assert_eq!(game.players[caster].attributes.thunder, 0);
    }

    #[test]
    fn test_effect_water_deallocation() {
        let mut game = setup_test_game();
        let caster = 0;

        game.players[caster].attributes.water = 6;
        game.apply_effect(caster, &EffectType::WaterDeallocation, &[], &[]).unwrap();

        assert_eq!(game.players[caster].attributes.water, 0);
    }

    // === 治療類效果測試 ===

    #[test]
    fn test_effect_heal() {
        let mut game = setup_test_game();
        let target = 1;

        // Damage target first
        game.players[target].hp = 30;

        game.apply_effect(0, &EffectType::Heal(10), &[target], &[]).unwrap();

        assert_eq!(game.players[target].hp, 40);
    }

    #[test]
    fn test_effect_heal_self() {
        let mut game = setup_test_game();
        let caster = 0;

        game.players[caster].hp = 20;

        game.apply_effect(caster, &EffectType::HealSelf(15), &[], &[]).unwrap();

        assert_eq!(game.players[caster].hp, 35);
    }

    #[test]
    fn test_effect_heal_capped_at_max() {
        let mut game = setup_test_game();
        let target = 1;

        game.players[target].hp = 45;
        let max_hp = game.players[target].max_hp;

        game.apply_effect(0, &EffectType::Heal(20), &[target], &[]).unwrap();

        assert_eq!(game.players[target].hp, max_hp);
    }

    // === 護盾類效果測試 ===

    #[test]
    fn test_effect_shield() {
        let mut game = setup_test_game();
        let target = 1;

        game.apply_effect(0, &EffectType::Shield(8), &[target], &[]).unwrap();

        assert_eq!(game.players[target].shield, 8);
    }

    #[test]
    fn test_effect_shield_self() {
        let mut game = setup_test_game();
        let caster = 0;

        game.apply_effect(caster, &EffectType::ShieldSelf(5), &[], &[]).unwrap();

        assert_eq!(game.players[caster].shield, 5);
    }

    #[test]
    fn test_effect_destroy_all_shield() {
        let mut game = setup_test_game();

        // Give everyone shields
        for player in &mut game.players {
            player.shield = 10;
        }

        game.apply_effect(0, &EffectType::DestroyAllShield, &[], &[]).unwrap();

        for player in &game.players {
            assert_eq!(player.shield, 0);
        }
    }

    // === Buff類效果測試 ===

    #[test]
    fn test_effect_buff_one() {
        let mut game = setup_test_game();
        let target = 1;

        game.apply_effect(0, &EffectType::BuffOne(BuffType::Immune), &[target], &[]).unwrap();

        assert!(game.players[target].buffs.has(BuffType::Immune));
    }

    #[test]
    fn test_effect_buff_forever() {
        let mut game = setup_test_game();
        let target = 1;

        game.apply_effect(0, &EffectType::BuffForever(BuffType::Regeneration), &[target], &[]).unwrap();

        assert!(game.players[target].buffs.has(BuffType::Regeneration));
        if let Some(buff) = game.players[target].buffs.get(BuffType::Regeneration) {
            assert!(matches!(buff.duration, BuffDuration::Permanent));
        }
    }

    #[test]
    fn test_effect_buff_self_turns() {
        let mut game = setup_test_game();
        let caster = 0;

        game.apply_effect(caster, &EffectType::BuffSelfTwo(BuffType::BurningOut), &[], &[]).unwrap();

        assert!(game.players[caster].buffs.has(BuffType::BurningOut));
        if let Some(buff) = game.players[caster].buffs.get(BuffType::BurningOut) {
            assert!(matches!(buff.duration, BuffDuration::Turns(2)));
        }
    }

    #[test]
    fn test_effect_buff_immunity_blocks_debuff() {
        let mut game = setup_test_game();
        let target = 1;

        // Give target immunity first
        game.players[target].buffs.add(Buff::new(BuffType::Immune, BuffDuration::Permanent));

        // Try to apply debuff
        game.apply_effect(0, &EffectType::BuffOne(BuffType::Paralysis), &[target], &[]).unwrap();

        // Should not have paralysis due to immunity
        assert!(!game.players[target].buffs.has(BuffType::Paralysis));
    }

    #[test]
    fn test_effect_health_drain() {
        let mut game = setup_test_game();
        let caster = 0;
        let target = 1;

        game.apply_effect(caster, &EffectType::HealthDrain, &[target], &[]).unwrap();

        // Caster should have HealthDrain buff
        assert!(game.players[caster].buffs.has(BuffType::HealthDrain));
        // Target should have HealthDrainTarget debuff
        assert!(game.players[target].buffs.has(BuffType::HealthDrainTarget));
    }

    #[test]
    fn test_effect_guard_wood_carving() {
        let mut game = setup_test_game();
        let target = 2; // Ally

        game.apply_effect(0, &EffectType::GuardWoodCarving(BuffType::GuardWoodCarving), &[target], &[]).unwrap();

        assert!(game.players[target].buffs.has(BuffType::GuardWoodCarving));
        if let Some(buff) = game.players[target].buffs.get(BuffType::GuardWoodCarving) {
            assert_eq!(buff.data, Some(3));
        }
    }

    #[test]
    fn test_effect_remove_all_buff() {
        let mut game = setup_test_game();

        // Give everyone some debuffs
        for player in &mut game.players {
            player.buffs.add(Buff::new(BuffType::Paralysis, BuffDuration::Permanent));
            player.buffs.add(Buff::new(BuffType::Silent, BuffDuration::Permanent));
        }

        game.apply_effect(0, &EffectType::RemoveAllBuff, &[], &[]).unwrap();

        // All debuffs should be cleared
        for player in &game.players {
            assert!(!player.buffs.has(BuffType::Paralysis));
            assert!(!player.buffs.has(BuffType::Silent));
        }
    }

    // === 屬性點操作類效果測試 ===

    #[test]
    fn test_effect_remove_main_mt_ap() {
        let mut game = setup_test_game();
        let target = 1;

        // Set up target attributes
        game.players[target].attributes.fire = 3;
        game.players[target].attributes.wood = 2;
        game.players[target].hand.push(1);
        game.players[target].hand.push(2);
        let initial_hand_size = game.players[target].hand.len();

        game.apply_effect(0, &EffectType::RemoveMainMTAP(4), &[target], &[]).unwrap();

        // Should have removed 4 attribute points total
        let remaining = game.players[target].attributes.fire + game.players[target].attributes.wood;
        assert_eq!(remaining, 1);

        // Should have discarded 1 card
        assert_eq!(game.players[target].hand.len(), initial_hand_size - 1);
    }

    #[test]
    fn test_effect_remove_ap() {
        let mut game = setup_test_game();
        let target = 1;

        game.players[target].attributes.fire = 2;
        game.players[target].attributes.wood = 3;
        game.players[target].attributes.wind = 1;

        game.apply_effect(0, &EffectType::RemoveAP(5), &[target], &[]).unwrap();

        // Should have removed 5 points total from all attributes
        let total = game.players[target].attributes.fire +
                    game.players[target].attributes.wood +
                    game.players[target].attributes.wind;
        assert_eq!(total, 1);
    }

    // === 卡牌操作類效果測試 ===

    #[test]
    fn test_effect_random_discard() {
        let mut game = setup_test_game();
        let target = 1;

        game.players[target].hand.push(10);
        game.players[target].hand.push(11);
        game.players[target].hand.push(12);
        let initial_hand_size = game.players[target].hand.len();

        game.apply_effect(0, &EffectType::RandomDiscard, &[target], &[]).unwrap();

        assert_eq!(game.players[target].hand.len(), initial_hand_size - 1);
    }

    // === 特殊類效果測試 ===

    #[test]
    fn test_effect_dearouse() {
        let mut game = setup_test_game();
        let target = 1;

        // Liberate the character first
        game.players[target].character.is_liberated = true;

        game.apply_effect(0, &EffectType::Dearouse, &[target], &[]).unwrap();

        assert!(!game.players[target].character.is_liberated);
    }
}
