use serde::{Deserialize, Serialize};
use crate::attribute::AttributePoints;
use crate::buff::{Buff, BuffType};
use crate::card::CardId;
use crate::character::{Character, CharacterType};
use crate::player::{Player, PlayerId};

/// API響應包裝
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(code: String, message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ApiError {
                code,
                message,
                details: None,
            }),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}

/// 創建遊戲請求
#[derive(Debug, Deserialize)]
pub struct CreateGameRequest {
    pub player_names: Vec<String>,
    pub characters: Vec<CharacterType>,
}

/// 遊戲信息響應
#[derive(Debug, Serialize)]
pub struct GameInfoResponse {
    pub game_id: String,
    pub state: String,
    pub current_player_index: usize,
    pub turn_number: u32,
    pub turn_phase: String,
    pub players: Vec<PlayerDto>,
    pub deck_remaining: usize,  // 共享牌庫剩餘卡片數
}

/// 玩家DTO
#[derive(Debug, Serialize, Clone)]
pub struct PlayerDto {
    pub id: PlayerId,
    pub name: String,
    pub character: CharacterDto,
    pub team: String,
    pub hp: i32,
    pub max_hp: i32,
    pub shield: u32,
    pub attributes: AttributePointsDto,
    pub buffs: Vec<BuffDto>,
    pub hand: Vec<CardId>,
    pub hand_count: usize,  // 手牌數量（可能不公開具體卡片）
    pub discard_pile_count: usize,  // 棄牌堆數量
    pub is_dead: bool,
    pub death_turns: u8,
    pub can_act: bool,
    pub can_liberate: bool,
}

impl From<&Player> for PlayerDto {
    fn from(player: &Player) -> Self {
        Self {
            id: player.id,
            name: player.name.clone(),
            character: CharacterDto {
                character_type: format!("{:?}", player.character.character_type),
                name: player.character.character_type.name().to_string(),
                title: player.character.character_type.title().to_string(),
                is_liberated: player.character.is_liberated,
                liberation_name: player.character.character_type.liberation_name().to_string(),
            },
            team: format!("{:?}", player.team),
            hp: player.hp,
            max_hp: player.max_hp,
            shield: player.shield,
            attributes: AttributePointsDto {
                fire: player.attributes.fire,
                wood: player.attributes.wood,
                thunder: player.attributes.thunder,
                water: player.attributes.water,
                wind: player.attributes.wind,
                poison: player.attributes.poison,
            },
            buffs: player.buffs.buffs.iter().map(|b| BuffDto {
                buff_type: format!("{:?}", b.buff_type),
                name: b.buff_type.to_string().to_string(),
                duration: format!("{:?}", b.duration),
                data: b.data,
            }).collect(),
            hand: player.hand.clone(),
            hand_count: player.hand.len(),
            discard_pile_count: player.discard_pile.len(),
            is_dead: player.is_dead,
            death_turns: player.death_turns,
            can_act: player.can_act(),
            can_liberate: player.can_use_liberation(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct CharacterDto {
    pub character_type: String,
    pub name: String,
    pub title: String,
    pub is_liberated: bool,
    pub liberation_name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct AttributePointsDto {
    pub fire: u8,
    pub wood: u8,
    pub thunder: u8,
    pub water: u8,
    pub wind: u8,
    pub poison: u8,
}

#[derive(Debug, Serialize, Clone)]
pub struct BuffDto {
    pub buff_type: String,
    pub name: String,
    pub duration: String,
    pub data: Option<i32>,
}

/// 分配屬性請求
#[derive(Debug, Deserialize)]
pub struct AllocateAttributeRequest {
    pub attribute: String,  // "Fire", "Wood", etc.
}

/// 使用屬性彈請求
#[derive(Debug, Deserialize)]
pub struct PlayAttributeBoltRequest {
    pub card_id: CardId,  // 使用哪張卡（任何卡都可以）
    pub attribute: String,  // 使用哪個屬性彈 "Fire", "Wood", "Thunder", "Water", "Wind", "Poison"
}

/// 打出法術卡請求
#[derive(Debug, Deserialize)]
pub struct PlaySpellCardRequest {
    pub card_id: CardId,
    pub side: String,  // "Top" or "Bottom"
    pub targets: Vec<PlayerId>,
}

/// 使用解放技能請求
#[derive(Debug, Deserialize)]
pub struct UseLiberationRequest {
    pub targets: Vec<PlayerId>,
}

/// 卡片信息DTO
#[derive(Debug, Serialize, Clone)]
pub struct CardDto {
    pub id: CardId,
    pub current_side: String,
    pub current_spell: SpellDto,
    pub other_spell: SpellDto,
}

#[derive(Debug, Serialize, Clone)]
pub struct SpellDto {
    pub spell_id: String,
    pub name: String,
    pub requirements: String,
    pub effect_description: String,
    pub is_aoyi: bool,
}

/// 遊戲動作結果
#[derive(Debug, Serialize)]
pub struct ActionResultDto {
    pub success: bool,
    pub message: String,
    pub events: Vec<GameEventDto>,
}

#[derive(Debug, Serialize, Clone)]
pub struct GameEventDto {
    pub event_type: String,
    pub message: String,
    pub player_id: Option<PlayerId>,
    pub value: Option<i32>,
}
