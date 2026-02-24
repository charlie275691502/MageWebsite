use crate::character::CharacterType;
use crate::game::Game;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// 房間代碼（6位數字）
pub type RoomCode = String;

/// 隊伍選擇
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Team {
    A,
    B,
    C,
    D,
}

/// 玩家槽位狀態
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSlot {
    pub slot_id: usize,
    pub player_name: Option<String>,
    pub character: Option<CharacterType>,
    pub team: Option<Team>,  // 隊伍選擇
    pub is_ready: bool,
    pub connection_id: Option<String>,  // 用於追蹤連接
}

impl PlayerSlot {
    pub fn new(slot_id: usize) -> Self {
        Self {
            slot_id,
            player_name: None,
            character: None,
            team: None,
            is_ready: false,
            connection_id: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.player_name.is_none()
    }

    pub fn is_occupied(&self) -> bool {
        !self.is_empty()
    }
}

/// 遊戲房間狀態
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoomState {
    Waiting,    // 等待玩家加入
    Ready,      // 可以開始（4人已就緒）
    InGame,     // 遊戲進行中
    Finished,   // 遊戲結束
}

/// 遊戲房間
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRoom {
    pub room_code: RoomCode,
    pub host_slot: usize,  // 房主的槽位ID
    pub state: RoomState,
    pub player_slots: Vec<PlayerSlot>,  // 4個槽位
    pub game_id: Option<String>,  // 遊戲開始後的ID
}

impl GameRoom {
    pub fn new(room_code: RoomCode, host_name: String, host_connection_id: String) -> Self {
        let mut slots = vec![
            PlayerSlot::new(0),
            PlayerSlot::new(1),
            PlayerSlot::new(2),
            PlayerSlot::new(3),
        ];

        // 房主占據第一個槽位
        slots[0].player_name = Some(host_name);
        slots[0].connection_id = Some(host_connection_id);

        Self {
            room_code,
            host_slot: 0,
            state: RoomState::Waiting,
            player_slots: slots,
            game_id: None,
        }
    }

    /// 生成隨機6位數房間代碼
    pub fn generate_room_code() -> RoomCode {
        let mut rng = rand::thread_rng();
        format!("{:06}", rng.gen_range(0..1000000))
    }

    /// 加入房間
    pub fn join(&mut self, player_name: String, connection_id: String) -> Result<usize, String> {
        // 找到第一個空槽位
        for slot in &mut self.player_slots {
            if slot.is_empty() {
                slot.player_name = Some(player_name);
                slot.connection_id = Some(connection_id);
                return Ok(slot.slot_id);
            }
        }
        Err("房間已滿".to_string())
    }

    /// 離開房間
    pub fn leave(&mut self, slot_id: usize) -> Result<(), String> {
        if slot_id >= self.player_slots.len() {
            return Err("無效的槽位ID".to_string());
        }

        if slot_id == self.host_slot {
            return Err("房主不能離開（請解散房間）".to_string());
        }

        let slot = &mut self.player_slots[slot_id];
        slot.player_name = None;
        slot.character = None;
        slot.team = None;
        slot.is_ready = false;
        slot.connection_id = None;

        // 更新房間狀態
        self.update_state();

        Ok(())
    }

    /// 選擇隊伍
    pub fn select_team(&mut self, slot_id: usize, team: Team) -> Result<(), String> {
        if slot_id >= self.player_slots.len() {
            return Err("無效的槽位ID".to_string());
        }

        if self.player_slots[slot_id].is_empty() {
            return Err("槽位未被占用".to_string());
        }

        self.player_slots[slot_id].team = Some(team);
        self.update_state();

        Ok(())
    }

    /// 選擇角色
    pub fn select_character(&mut self, slot_id: usize, character: CharacterType) -> Result<(), String> {
        if slot_id >= self.player_slots.len() {
            return Err("無效的槽位ID".to_string());
        }

        if self.player_slots[slot_id].is_empty() {
            return Err("槽位未被占用".to_string());
        }

        self.player_slots[slot_id].character = Some(character);
        self.update_state();

        Ok(())
    }

    /// 設置準備狀態
    pub fn set_ready(&mut self, slot_id: usize, ready: bool) -> Result<(), String> {
        if slot_id >= self.player_slots.len() {
            return Err("無效的槽位ID".to_string());
        }

        let slot = &mut self.player_slots[slot_id];

        if slot.is_empty() {
            return Err("槽位未被占用".to_string());
        }

        if slot.character.is_none() {
            return Err("請先選擇角色".to_string());
        }

        if slot.team.is_none() {
            return Err("請先選擇隊伍".to_string());
        }

        slot.is_ready = ready;
        self.update_state();

        Ok(())
    }

    /// 檢查是否可以開始遊戲
    pub fn can_start_game(&self) -> bool {
        // 獲取所有已占用且準備好的槽位
        let ready_players: Vec<&PlayerSlot> = self.player_slots.iter()
            .filter(|s| s.is_occupied() && s.character.is_some() && s.team.is_some() && s.is_ready)
            .collect();

        // 至少需要2個玩家
        if ready_players.len() < 2 {
            return false;
        }

        // 需要至少2個不同的隊伍
        let mut teams = std::collections::HashSet::new();
        for player in &ready_players {
            if let Some(team) = player.team {
                teams.insert(team);
            }
        }

        teams.len() >= 2
    }

    /// 更新房間狀態
    fn update_state(&mut self) {
        if self.state == RoomState::InGame || self.state == RoomState::Finished {
            return;
        }

        if self.can_start_game() {
            self.state = RoomState::Ready;
        } else {
            self.state = RoomState::Waiting;
        }
    }

    /// 開始遊戲
    pub fn start_game(&mut self, game_id: String) -> Result<(), String> {
        if !self.can_start_game() {
            return Err("還有玩家未準備好".to_string());
        }

        self.game_id = Some(game_id);
        self.state = RoomState::InGame;

        Ok(())
    }

    /// 獲取玩家數量
    pub fn player_count(&self) -> usize {
        self.player_slots.iter().filter(|s| s.is_occupied()).count()
    }

    /// 根據連接ID查找槽位
    pub fn find_slot_by_connection(&self, connection_id: &str) -> Option<usize> {
        self.player_slots.iter()
            .find(|s| s.connection_id.as_deref() == Some(connection_id))
            .map(|s| s.slot_id)
    }

    /// 獲取遊戲初始化數據
    pub fn get_game_init_data(&self) -> Result<(Vec<String>, Vec<CharacterType>), String> {
        if !self.can_start_game() {
            return Err("房間未準備好".to_string());
        }

        let mut names = Vec::new();
        let mut characters = Vec::new();

        // 只包含已占用、已選角色、已選隊伍且準備好的玩家
        for slot in &self.player_slots {
            if slot.is_occupied() && slot.character.is_some() && slot.team.is_some() && slot.is_ready {
                names.push(slot.player_name.clone().unwrap());
                characters.push(slot.character.unwrap());
            }
        }

        Ok((names, characters))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_creation() {
        let code = GameRoom::generate_room_code();
        assert_eq!(code.len(), 6);

        let room = GameRoom::new(code.clone(), "Player1".to_string(), "conn1".to_string());
        assert_eq!(room.room_code, code);
        assert_eq!(room.host_slot, 0);
        assert_eq!(room.state, RoomState::Waiting);
        assert_eq!(room.player_count(), 1);
    }

    #[test]
    fn test_join_room() {
        let mut room = GameRoom::new("123456".to_string(), "Host".to_string(), "conn1".to_string());

        let slot = room.join("Player2".to_string(), "conn2".to_string()).unwrap();
        assert_eq!(slot, 1);
        assert_eq!(room.player_count(), 2);
    }

    #[test]
    fn test_ready_and_start() {
        let mut room = GameRoom::new("123456".to_string(), "Host".to_string(), "conn1".to_string());
        room.join("Player2".to_string(), "conn2".to_string()).unwrap();
        room.join("Player3".to_string(), "conn3".to_string()).unwrap();
        room.join("Player4".to_string(), "conn4".to_string()).unwrap();

        // 選擇角色
        room.select_character(0, CharacterType::FlamePoison).unwrap();
        room.select_character(1, CharacterType::WoodWind).unwrap();
        room.select_character(2, CharacterType::ThunderPoison).unwrap();
        room.select_character(3, CharacterType::WaterWind).unwrap();

        // 選擇隊伍 (P0和P2在Team A，P1和P3在Team B)
        room.select_team(0, Team::A).unwrap();
        room.select_team(1, Team::B).unwrap();
        room.select_team(2, Team::C).unwrap();
        room.select_team(3, Team::D).unwrap();

        assert!(!room.can_start_game());

        // 所有人準備
        room.set_ready(0, true).unwrap();
        room.set_ready(1, true).unwrap();
        room.set_ready(2, true).unwrap();
        room.set_ready(3, true).unwrap();

        assert!(room.can_start_game());
        assert_eq!(room.state, RoomState::Ready);

        room.start_game("game123".to_string()).unwrap();
        assert_eq!(room.state, RoomState::InGame);
    }
}
