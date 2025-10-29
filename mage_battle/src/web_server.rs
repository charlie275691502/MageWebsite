use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dashmap::DashMap;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

use crate::api_types::*;
use crate::attribute::AttributeType;
use crate::card::CardSide;
use crate::game::Game;
use crate::lobby::{GameRoom, RoomCode};
use crate::lobby_types::{
    CreateRoomRequest, CreateRoomResponse, JoinRoomRequest, JoinRoomResponse,
    LeaveRoomRequest, SelectCharacterRequest, ReadyRequest, StartGameRequest,
    StartGameResponse, RoomDto,
};

/// 全局遊戲狀態
pub struct GameStore {
    games: DashMap<String, Arc<tokio::sync::Mutex<Game>>>,
    rooms: DashMap<RoomCode, Arc<tokio::sync::Mutex<GameRoom>>>,
}

impl GameStore {
    pub fn new() -> Self {
        Self {
            games: DashMap::new(),
            rooms: DashMap::new(),
        }
    }

    pub fn create_game(&self, player_names: Vec<String>, characters: Vec<crate::character::CharacterType>) -> String {
        let game_id = Uuid::new_v4().to_string();
        let game = Game::new(player_names, characters);
        self.games.insert(game_id.clone(), Arc::new(tokio::sync::Mutex::new(game)));
        game_id
    }

    pub fn get_game(&self, game_id: &str) -> Option<Arc<tokio::sync::Mutex<Game>>> {
        self.games.get(game_id).map(|entry| entry.value().clone())
    }

    pub fn remove_game(&self, game_id: &str) {
        self.games.remove(game_id);
    }

    // Lobby methods
    pub fn create_room(&self, player_name: String) -> (RoomCode, String) {
        let room_code = GameRoom::generate_room_code();
        let connection_id = Uuid::new_v4().to_string();
        let room = GameRoom::new(room_code.clone(), player_name, connection_id.clone());
        self.rooms.insert(room_code.clone(), Arc::new(tokio::sync::Mutex::new(room)));
        (room_code, connection_id)
    }

    pub fn get_room(&self, room_code: &str) -> Option<Arc<tokio::sync::Mutex<GameRoom>>> {
        self.rooms.get(room_code).map(|entry| entry.value().clone())
    }

    pub fn remove_room(&self, room_code: &str) {
        self.rooms.remove(room_code);
    }
}

pub type AppState = Arc<GameStore>;

/// 創建路由
pub fn create_router() -> Router {
    let store = Arc::new(GameStore::new());

    Router::new()
        // Lobby
        .route("/api/lobby/create", post(create_lobby_room))
        .route("/api/lobby/join", post(join_lobby_room))
        .route("/api/lobby/:room_code", get(get_lobby_room))
        .route("/api/lobby/:room_code/character", post(select_character_in_lobby))
        .route("/api/lobby/:room_code/ready", post(set_ready_in_lobby))
        .route("/api/lobby/:room_code/start", post(start_game_from_lobby))
        .route("/api/lobby/:room_code/leave", post(leave_lobby_room))

        // 遊戲管理
        .route("/api/game/new", post(create_game))
        .route("/api/game/:game_id", get(get_game_info))
        .route("/api/game/:game_id", post(delete_game))

        // 遊戲動作
        .route("/api/game/:game_id/allocate", post(allocate_attribute))
        .route("/api/game/:game_id/play_bolt", post(play_attribute_bolt))
        .route("/api/game/:game_id/play_card", post(play_spell_card))
        .route("/api/game/:game_id/liberate", post(use_liberation))
        .route("/api/game/:game_id/draw", post(draw_card))

        // 查詢
        .route("/api/game/:game_id/players", get(get_players))
        .route("/api/game/:game_id/players/:player_id", get(get_player))

        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any))
        .with_state(store)
}

/// 啟動服務器
pub async fn start_server() {
    let app = create_router();

    let addr = "0.0.0.0:3000".parse().unwrap();

    println!("🚀 Server running on http://localhost:3000");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// ==================== API 處理器 ====================

/// 創建新遊戲
async fn create_game(
    State(store): State<AppState>,
    Json(req): Json<CreateGameRequest>,
) -> impl IntoResponse {
    if req.player_names.len() != 4 || req.characters.len() != 4 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<GameInfoResponse>::error(
                "INVALID_REQUEST".to_string(),
                "需要4個玩家和4個角色".to_string(),
            )),
        );
    }

    let game_id = store.create_game(req.player_names, req.characters);

    if let Some(game_mutex) = store.get_game(&game_id) {
        let game = game_mutex.lock().await;
        let response = GameInfoResponse {
            game_id: game_id.clone(),
            state: format!("{:?}", game.state),
            current_player_index: game.current_player_index,
            turn_number: game.turn_number,
            turn_phase: format!("{:?}", game.turn_phase),
            players: game.players.iter().map(|p| p.into()).collect(),
        };
        (StatusCode::OK, Json(ApiResponse::ok(response)))
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<GameInfoResponse>::error(
                "INTERNAL_ERROR".to_string(),
                "創建遊戲失敗".to_string(),
            )),
        )
    }
}

/// 獲取遊戲信息
async fn get_game_info(
    State(store): State<AppState>,
    Path(game_id): Path<String>,
) -> impl IntoResponse {
    if let Some(game_mutex) = store.get_game(&game_id) {
        let game = game_mutex.lock().await;
        let response = GameInfoResponse {
            game_id: game_id.clone(),
            state: format!("{:?}", game.state),
            current_player_index: game.current_player_index,
            turn_number: game.turn_number,
            turn_phase: format!("{:?}", game.turn_phase),
            players: game.players.iter().map(|p| p.into()).collect(),
        };
        (StatusCode::OK, Json(ApiResponse::ok(response)))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<GameInfoResponse>::error(
                "GAME_NOT_FOUND".to_string(),
                "遊戲不存在".to_string(),
            )),
        )
    }
}

/// 刪除遊戲
async fn delete_game(
    State(store): State<AppState>,
    Path(game_id): Path<String>,
) -> impl IntoResponse {
    store.remove_game(&game_id);
    (StatusCode::OK, Json(ApiResponse::ok("遊戲已刪除".to_string())))
}

// ==================== Lobby 處理器 ====================

/// 創建房間
async fn create_lobby_room(
    State(store): State<AppState>,
    Json(req): Json<CreateRoomRequest>,
) -> impl IntoResponse {
    let (room_code, connection_id) = store.create_room(req.player_name);

    let response = CreateRoomResponse {
        room_code,
        slot_id: 0,
        connection_id,
    };

    (StatusCode::OK, Json(ApiResponse::ok(response)))
}

/// 加入房間
async fn join_lobby_room(
    State(store): State<AppState>,
    Json(req): Json<JoinRoomRequest>,
) -> impl IntoResponse {
    if let Some(room_mutex) = store.get_room(&req.room_code) {
        let mut room = room_mutex.lock().await;
        let connection_id = Uuid::new_v4().to_string();

        match room.join(req.player_name, connection_id.clone()) {
            Ok(slot_id) => {
                let response = JoinRoomResponse {
                    room_code: req.room_code,
                    slot_id,
                    connection_id,
                    room: (&*room).into(),
                };
                (StatusCode::OK, Json(ApiResponse::ok(response)))
            }
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<JoinRoomResponse>::error(
                    "JOIN_FAILED".to_string(),
                    e,
                )),
            ),
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<JoinRoomResponse>::error(
                "ROOM_NOT_FOUND".to_string(),
                "房間不存在".to_string(),
            )),
        )
    }
}

/// 獲取房間信息
async fn get_lobby_room(
    State(store): State<AppState>,
    Path(room_code): Path<String>,
) -> impl IntoResponse {
    if let Some(room_mutex) = store.get_room(&room_code) {
        let room = room_mutex.lock().await;
        let response: RoomDto = (&*room).into();
        (StatusCode::OK, Json(ApiResponse::ok(response)))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<RoomDto>::error(
                "ROOM_NOT_FOUND".to_string(),
                "房間不存在".to_string(),
            )),
        )
    }
}

/// 選擇角色
async fn select_character_in_lobby(
    State(store): State<AppState>,
    Path(room_code): Path<String>,
    Json(req): Json<SelectCharacterRequest>,
) -> impl IntoResponse {
    if let Some(room_mutex) = store.get_room(&room_code) {
        let mut room = room_mutex.lock().await;

        if let Some(slot_id) = room.find_slot_by_connection(&req.connection_id) {
            match room.select_character(slot_id, req.character) {
                Ok(_) => {
                    let response: RoomDto = (&*room).into();
                    (StatusCode::OK, Json(ApiResponse::ok(response)))
                }
                Err(e) => (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<RoomDto>::error(
                        "SELECT_FAILED".to_string(),
                        e,
                    )),
                ),
            }
        } else {
            (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::<RoomDto>::error(
                    "INVALID_CONNECTION".to_string(),
                    "無效的連接ID".to_string(),
                )),
            )
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<RoomDto>::error(
                "ROOM_NOT_FOUND".to_string(),
                "房間不存在".to_string(),
            )),
        )
    }
}

/// 設置準備狀態
async fn set_ready_in_lobby(
    State(store): State<AppState>,
    Path(room_code): Path<String>,
    Json(req): Json<ReadyRequest>,
) -> impl IntoResponse {
    if let Some(room_mutex) = store.get_room(&room_code) {
        let mut room = room_mutex.lock().await;

        if let Some(slot_id) = room.find_slot_by_connection(&req.connection_id) {
            match room.set_ready(slot_id, req.ready) {
                Ok(_) => {
                    let response: RoomDto = (&*room).into();
                    (StatusCode::OK, Json(ApiResponse::ok(response)))
                }
                Err(e) => (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<RoomDto>::error(
                        "READY_FAILED".to_string(),
                        e,
                    )),
                ),
            }
        } else {
            (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::<RoomDto>::error(
                    "INVALID_CONNECTION".to_string(),
                    "無效的連接ID".to_string(),
                )),
            )
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<RoomDto>::error(
                "ROOM_NOT_FOUND".to_string(),
                "房間不存在".to_string(),
            )),
        )
    }
}

/// 開始遊戲
async fn start_game_from_lobby(
    State(store): State<AppState>,
    Path(room_code): Path<String>,
    Json(req): Json<StartGameRequest>,
) -> impl IntoResponse {
    if let Some(room_mutex) = store.get_room(&room_code) {
        let mut room = room_mutex.lock().await;

        if let Some(slot_id) = room.find_slot_by_connection(&req.connection_id) {
            // 只有房主可以開始遊戲
            if slot_id != room.host_slot {
                return (
                    StatusCode::FORBIDDEN,
                    Json(ApiResponse::<StartGameResponse>::error(
                        "NOT_HOST".to_string(),
                        "只有房主可以開始遊戲".to_string(),
                    )),
                );
            }

            match room.get_game_init_data() {
                Ok((player_names, characters)) => {
                    // 創建遊戲
                    let game_id = store.create_game(player_names, characters);

                    // 更新房間狀態
                    if let Err(e) = room.start_game(game_id.clone()) {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ApiResponse::<StartGameResponse>::error(
                                "START_FAILED".to_string(),
                                e,
                            )),
                        );
                    }

                    let response = StartGameResponse {
                        game_id,
                        your_slot_id: slot_id,
                    };

                    (StatusCode::OK, Json(ApiResponse::ok(response)))
                }
                Err(e) => (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<StartGameResponse>::error(
                        "NOT_READY".to_string(),
                        e,
                    )),
                ),
            }
        } else {
            (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::<StartGameResponse>::error(
                    "INVALID_CONNECTION".to_string(),
                    "無效的連接ID".to_string(),
                )),
            )
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<StartGameResponse>::error(
                "ROOM_NOT_FOUND".to_string(),
                "房間不存在".to_string(),
            )),
        )
    }
}

/// 離開房間
async fn leave_lobby_room(
    State(store): State<AppState>,
    Path(room_code): Path<String>,
    Json(req): Json<LeaveRoomRequest>,
) -> impl IntoResponse {
    if let Some(room_mutex) = store.get_room(&room_code) {
        let mut room = room_mutex.lock().await;

        if let Some(slot_id) = room.find_slot_by_connection(&req.connection_id) {
            // 如果是房主離開，解散房間
            if slot_id == room.host_slot {
                drop(room); // 釋放鎖
                store.remove_room(&room_code);
                return (
                    StatusCode::OK,
                    Json(ApiResponse::ok("房間已解散".to_string())),
                );
            }

            match room.leave(slot_id) {
                Ok(_) => {
                    (StatusCode::OK, Json(ApiResponse::ok("已離開房間".to_string())))
                }
                Err(e) => (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<String>::error(
                        "LEAVE_FAILED".to_string(),
                        e,
                    )),
                ),
            }
        } else {
            (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::<String>::error(
                    "INVALID_CONNECTION".to_string(),
                    "無效的連接ID".to_string(),
                )),
            )
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<String>::error(
                "ROOM_NOT_FOUND".to_string(),
                "房間不存在".to_string(),
            )),
        )
    }
}

// ==================== API 處理器 ====================

/// 分配屬性點
async fn allocate_attribute(
    State(store): State<AppState>,
    Path(game_id): Path<String>,
    Json(req): Json<AllocateAttributeRequest>,
) -> impl IntoResponse {
    if let Some(game_mutex) = store.get_game(&game_id) {
        let mut game = game_mutex.lock().await;

        let attr_type = match req.attribute.as_str() {
            "Fire" => AttributeType::Fire,
            "Wood" => AttributeType::Wood,
            "Thunder" => AttributeType::Thunder,
            "Water" => AttributeType::Water,
            "Wind" => AttributeType::Wind,
            "Poison" => AttributeType::Poison,
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<ActionResultDto>::error(
                        "INVALID_ATTRIBUTE".to_string(),
                        "無效的屬性類型".to_string(),
                    )),
                );
            }
        };

        match game.allocate_attribute(attr_type) {
            Ok(_) => {
                let result = ActionResultDto {
                    success: true,
                    message: format!("分配了{}屬性點", attr_type.to_string()),
                    events: vec![],
                };
                (StatusCode::OK, Json(ApiResponse::ok(result)))
            }
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<ActionResultDto>::error(
                    "ACTION_FAILED".to_string(),
                    e,
                )),
            ),
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<ActionResultDto>::error(
                "GAME_NOT_FOUND".to_string(),
                "遊戲不存在".to_string(),
            )),
        )
    }
}

/// 使用屬性彈
async fn play_attribute_bolt(
    State(store): State<AppState>,
    Path(game_id): Path<String>,
    Json(req): Json<PlayAttributeBoltRequest>,
) -> impl IntoResponse {
    if let Some(game_mutex) = store.get_game(&game_id) {
        let mut game = game_mutex.lock().await;

        let attr_type = match req.attribute.as_str() {
            "Fire" => AttributeType::Fire,
            "Wood" => AttributeType::Wood,
            "Thunder" => AttributeType::Thunder,
            "Water" => AttributeType::Water,
            "Wind" => AttributeType::Wind,
            "Poison" => AttributeType::Poison,
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<ActionResultDto>::error(
                        "INVALID_ATTRIBUTE".to_string(),
                        "無效的屬性類型".to_string(),
                    )),
                );
            }
        };

        match game.play_attribute_bolt(req.card_id, attr_type) {
            Ok(_) => {
                let current_id = game.current_player_index;
                let target_id = game.players[current_id].left_player_id();
                let result = ActionResultDto {
                    success: true,
                    message: format!("使用{}屬性彈攻擊前一位玩家", attr_type.to_string()),
                    events: vec![],
                };
                (StatusCode::OK, Json(ApiResponse::ok(result)))
            }
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<ActionResultDto>::error(
                    "ACTION_FAILED".to_string(),
                    e,
                )),
            ),
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<ActionResultDto>::error(
                "GAME_NOT_FOUND".to_string(),
                "遊戲不存在".to_string(),
            )),
        )
    }
}

/// 打出法術卡
async fn play_spell_card(
    State(store): State<AppState>,
    Path(game_id): Path<String>,
    Json(req): Json<PlaySpellCardRequest>,
) -> impl IntoResponse {
    if let Some(game_mutex) = store.get_game(&game_id) {
        let mut game = game_mutex.lock().await;

        let side = match req.side.as_str() {
            "Top" => CardSide::Top,
            "Bottom" => CardSide::Bottom,
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<ActionResultDto>::error(
                        "INVALID_SIDE".to_string(),
                        "無效的卡片面".to_string(),
                    )),
                );
            }
        };

        match game.play_spell_card(req.card_id, side, req.targets) {
            Ok(_) => {
                let result = ActionResultDto {
                    success: true,
                    message: format!("打出法術卡 {}", req.card_id),
                    events: vec![],
                };
                (StatusCode::OK, Json(ApiResponse::ok(result)))
            }
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<ActionResultDto>::error(
                    "ACTION_FAILED".to_string(),
                    e,
                )),
            ),
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<ActionResultDto>::error(
                "GAME_NOT_FOUND".to_string(),
                "遊戲不存在".to_string(),
            )),
        )
    }
}

/// 使用解放技能
async fn use_liberation(
    State(store): State<AppState>,
    Path(game_id): Path<String>,
    Json(req): Json<UseLiberationRequest>,
) -> impl IntoResponse {
    if let Some(game_mutex) = store.get_game(&game_id) {
        let mut game = game_mutex.lock().await;

        match game.use_liberation(req.targets) {
            Ok(_) => {
                let result = ActionResultDto {
                    success: true,
                    message: "使用解放技能".to_string(),
                    events: vec![],
                };
                (StatusCode::OK, Json(ApiResponse::ok(result)))
            }
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<ActionResultDto>::error(
                    "ACTION_FAILED".to_string(),
                    e,
                )),
            ),
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<ActionResultDto>::error(
                "GAME_NOT_FOUND".to_string(),
                "遊戲不存在".to_string(),
            )),
        )
    }
}

/// 抽卡
async fn draw_card(
    State(store): State<AppState>,
    Path(game_id): Path<String>,
) -> impl IntoResponse {
    if let Some(game_mutex) = store.get_game(&game_id) {
        let mut game = game_mutex.lock().await;

        match game.draw_card() {
            Ok(_) => {
                let result = ActionResultDto {
                    success: true,
                    message: "抽卡".to_string(),
                    events: vec![],
                };
                (StatusCode::OK, Json(ApiResponse::ok(result)))
            }
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<ActionResultDto>::error(
                    "ACTION_FAILED".to_string(),
                    e,
                )),
            ),
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<ActionResultDto>::error(
                "GAME_NOT_FOUND".to_string(),
                "遊戲不存在".to_string(),
            )),
        )
    }
}

/// 獲取所有玩家信息
async fn get_players(
    State(store): State<AppState>,
    Path(game_id): Path<String>,
) -> impl IntoResponse {
    if let Some(game_mutex) = store.get_game(&game_id) {
        let game = game_mutex.lock().await;
        let players: Vec<PlayerDto> = game.players.iter().map(|p| p.into()).collect();
        (StatusCode::OK, Json(ApiResponse::ok(players)))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<Vec<PlayerDto>>::error(
                "GAME_NOT_FOUND".to_string(),
                "遊戲不存在".to_string(),
            )),
        )
    }
}

/// 獲取單個玩家信息
async fn get_player(
    State(store): State<AppState>,
    Path((game_id, player_id)): Path<(String, usize)>,
) -> impl IntoResponse {
    if let Some(game_mutex) = store.get_game(&game_id) {
        let game = game_mutex.lock().await;
        if let Some(player) = game.players.get(player_id) {
            let dto: PlayerDto = player.into();
            (StatusCode::OK, Json(ApiResponse::ok(dto)))
        } else {
            (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::<PlayerDto>::error(
                    "PLAYER_NOT_FOUND".to_string(),
                    "玩家不存在".to_string(),
                )),
            )
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<PlayerDto>::error(
                "GAME_NOT_FOUND".to_string(),
                "遊戲不存在".to_string(),
            )),
        )
    }
}
