use serde::{Deserialize, Serialize};
use crate::character::CharacterType;
use crate::lobby::{GameRoom, PlayerSlot, RoomState, Team};

/// 創建房間請求
#[derive(Debug, Deserialize)]
pub struct CreateRoomRequest {
    pub player_name: String,
}

/// 創建房間響應
#[derive(Debug, Serialize)]
pub struct CreateRoomResponse {
    pub room_code: String,
    pub slot_id: usize,
    pub connection_id: String,
}

/// 加入房間請求
#[derive(Debug, Deserialize)]
pub struct JoinRoomRequest {
    pub room_code: String,
    pub player_name: String,
}

/// 加入房間響應
#[derive(Debug, Serialize)]
pub struct JoinRoomResponse {
    pub room_code: String,
    pub slot_id: usize,
    pub connection_id: String,
    pub room: RoomDto,
}

/// 離開房間請求
#[derive(Debug, Deserialize)]
pub struct LeaveRoomRequest {
    pub connection_id: String,
}

/// 選擇角色請求
#[derive(Debug, Deserialize)]
pub struct SelectCharacterRequest {
    pub connection_id: String,
    pub character: CharacterType,
}

/// 選擇隊伍請求
#[derive(Debug, Deserialize)]
pub struct SelectTeamRequest {
    pub connection_id: String,
    pub team: Team,
}

/// 準備請求
#[derive(Debug, Deserialize)]
pub struct ReadyRequest {
    pub connection_id: String,
    pub ready: bool,
}

/// 開始遊戲請求
#[derive(Debug, Deserialize)]
pub struct StartGameRequest {
    pub connection_id: String,
}

/// 開始遊戲響應
#[derive(Debug, Serialize)]
pub struct StartGameResponse {
    pub game_id: String,
    pub your_slot_id: usize,
}

/// 房間DTO
#[derive(Debug, Serialize, Clone)]
pub struct RoomDto {
    pub room_code: String,
    pub host_slot: usize,
    pub state: String,
    pub player_slots: Vec<PlayerSlotDto>,
    pub game_id: Option<String>,
    pub player_count: usize,
    pub can_start: bool,
}

impl From<&GameRoom> for RoomDto {
    fn from(room: &GameRoom) -> Self {
        Self {
            room_code: room.room_code.clone(),
            host_slot: room.host_slot,
            state: format!("{:?}", room.state),
            player_slots: room.player_slots.iter().map(|s| s.into()).collect(),
            game_id: room.game_id.clone(),
            player_count: room.player_count(),
            can_start: room.can_start_game(),
        }
    }
}

/// 玩家槽位DTO
#[derive(Debug, Serialize, Clone)]
pub struct PlayerSlotDto {
    pub slot_id: usize,
    pub player_name: Option<String>,
    pub character: Option<String>,
    pub character_title: Option<String>,
    pub team: Option<String>,
    pub is_ready: bool,
    pub is_occupied: bool,
}

impl From<&PlayerSlot> for PlayerSlotDto {
    fn from(slot: &PlayerSlot) -> Self {
        Self {
            slot_id: slot.slot_id,
            player_name: slot.player_name.clone(),
            character: slot.character.as_ref().map(|c| format!("{:?}", c)),
            character_title: slot.character.as_ref().map(|c| c.title().to_string()),
            team: slot.team.as_ref().map(|t| format!("{:?}", t)),
            is_ready: slot.is_ready,
            is_occupied: slot.is_occupied(),
        }
    }
}
