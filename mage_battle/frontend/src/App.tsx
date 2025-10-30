import React, { useState, useEffect } from "react";
import { gameApi, GameInfo, ActionResult } from "./api/gameApi";
import {
  lobbyApi,
  RoomDto,
  CreateRoomResponse,
  JoinRoomResponse,
  StartGameResponse,
} from "./api/lobbyApi";
import cardData from "./cardData.json";
import "./App.css";

type AttributeType = "Fire" | "Wood" | "Thunder" | "Water" | "Wind" | "Poison";

const attributeNames: Record<AttributeType, string> = {
  Fire: "火",
  Wood: "木",
  Thunder: "雷",
  Water: "水",
  Wind: "風",
  Poison: "毒",
};

const attributeNamesReverse: Record<string, AttributeType> = {
  火: "Fire",
  木: "Wood",
  雷: "Thunder",
  水: "Water",
  風: "Wind",
  毒: "Poison",
};

const characterTypes = [
  { id: "FlamePoison", name: "火毒法師" },
  { id: "WoodWind", name: "木風法師" },
  { id: "ThunderPoison", name: "雷毒法師" },
  { id: "WaterWind", name: "水風法師" },
];

// Helper function to get card information
const getCardInfo = (cardId: number) => {
  return cardData.find((card) => card.id === cardId);
};

// Helper function to check if spell requirements are met
const checkSpellRequirement = (
  cost: string,
  playerAttributes: any
): boolean => {
  // Parse cost like "火1", "風5", etc.
  const match = cost.match(/^(.+?)(\d+)$/);
  if (!match) return true; // If can't parse, assume it can be cast

  const [, attrChar, levelStr] = match;
  const requiredLevel = parseInt(levelStr);
  const attrType = attributeNamesReverse[attrChar];

  if (!attrType) return true; // Unknown attribute, assume can be cast

  const playerAttrLevel =
    playerAttributes[attrType.toLowerCase() as keyof typeof playerAttributes];
  return playerAttrLevel >= requiredLevel;
};

// Random username generator for testing
const generateRandomUsername = () => {
  const adjectives = [
    "Swift",
    "Brave",
    "Mystic",
    "Dark",
    "Fire",
    "Ice",
    "Storm",
    "Shadow",
    "Light",
    "Crimson",
  ];
  const nouns = [
    "Mage",
    "Wizard",
    "Sorcerer",
    "Sage",
    "Warlock",
    "Enchanter",
    "Spellcaster",
    "Mystic",
    "Conjurer",
    "Archmage",
  ];
  const adj = adjectives[Math.floor(Math.random() * adjectives.length)];
  const noun = nouns[Math.floor(Math.random() * nouns.length)];
  const num = Math.floor(Math.random() * 999) + 1;
  return `${adj}${noun}${num}`;
};

type AppScreen = "lobby" | "waiting_room" | "game";

function App() {
  // App state
  const [screen, setScreen] = useState<AppScreen>("lobby");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Lobby state
  const [playerName, setPlayerName] = useState<string>("");
  const [roomCodeInput, setRoomCodeInput] = useState<string>("");

  // Room state
  const [roomCode, setRoomCode] = useState<string | null>(
    localStorage.getItem("mage_room_code")
  );
  const [connectionId, setConnectionId] = useState<string | null>(
    localStorage.getItem("mage_connection_id")
  );
  const [mySlotId, setMySlotId] = useState<number | null>(
    localStorage.getItem("mage_slot_id")
      ? parseInt(localStorage.getItem("mage_slot_id")!)
      : null
  );
  const [roomInfo, setRoomInfo] = useState<RoomDto | null>(null);

  // Game state
  const [gameId, setGameId] = useState<string | null>(
    localStorage.getItem("mage_game_id")
  );
  const [gameInfo, setGameInfo] = useState<GameInfo | null>(null);

  // Auto-refresh room info when in waiting room
  useEffect(() => {
    if (screen === "waiting_room" && roomCode) {
      loadRoomInfo();
      const interval = setInterval(loadRoomInfo, 2000);
      return () => clearInterval(interval);
    }
  }, [screen, roomCode]);

  // Auto-refresh game info when in game
  useEffect(() => {
    if (screen === "game" && gameId) {
      loadGameInfo();
      const interval = setInterval(loadGameInfo, 2000);
      return () => clearInterval(interval);
    }
  }, [screen, gameId]);

  // Check if we should restore a previous session
  useEffect(() => {
    if (gameId) {
      setScreen("game");
    } else if (roomCode && connectionId && mySlotId !== null) {
      setScreen("waiting_room");
    }
  }, []);

  const loadRoomInfo = async () => {
    if (!roomCode) return;
    try {
      const info = await lobbyApi.getRoomInfo(roomCode);
      setRoomInfo(info);

      // If game has started, transition to game screen
      if (info.game_id) {
        setGameId(info.game_id);
        localStorage.setItem("mage_game_id", info.game_id);
        setScreen("game");
      }

      setError(null);
    } catch (err: any) {
      setError(err.message);
    }
  };

  const loadGameInfo = async () => {
    if (!gameId) return;
    try {
      const info = await gameApi.getGameInfo(gameId);
      setGameInfo(info);
      setError(null);
    } catch (err: any) {
      setError(err.message);
    }
  };

  const handleCreateRoom = async () => {
    if (!playerName.trim()) {
      setError("請輸入你的名字");
      return;
    }

    try {
      setLoading(true);
      const response: CreateRoomResponse = await lobbyApi.createRoom(
        playerName.trim()
      );

      setRoomCode(response.room_code);
      setConnectionId(response.connection_id);
      setMySlotId(response.slot_id);

      localStorage.setItem("mage_room_code", response.room_code);
      localStorage.setItem("mage_connection_id", response.connection_id);
      localStorage.setItem("mage_slot_id", response.slot_id.toString());

      setScreen("waiting_room");
      setError(null);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleJoinRoom = async () => {
    if (!playerName.trim()) {
      setError("請輸入你的名字");
      return;
    }
    if (!roomCodeInput.trim()) {
      setError("請輸入房間代碼");
      return;
    }

    try {
      setLoading(true);
      const response: JoinRoomResponse = await lobbyApi.joinRoom(
        roomCodeInput.trim(),
        playerName.trim()
      );

      setRoomCode(response.room_code);
      setConnectionId(response.connection_id);
      setMySlotId(response.slot_id);
      setRoomInfo(response.room);

      localStorage.setItem("mage_room_code", response.room_code);
      localStorage.setItem("mage_connection_id", response.connection_id);
      localStorage.setItem("mage_slot_id", response.slot_id.toString());

      setScreen("waiting_room");
      setError(null);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleSelectCharacter = async (characterId: string) => {
    if (!roomCode || !connectionId) return;

    try {
      setLoading(true);
      const updatedRoom = await lobbyApi.selectCharacter(
        roomCode,
        connectionId,
        characterId
      );
      setRoomInfo(updatedRoom);
      setError(null);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleSelectTeam = async (team: string) => {
    if (!roomCode || !connectionId) return;

    try {
      setLoading(true);
      const updatedRoom = await lobbyApi.selectTeam(
        roomCode,
        connectionId,
        team
      );
      setRoomInfo(updatedRoom);
      setError(null);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleToggleReady = async () => {
    if (!roomCode || !connectionId || !roomInfo || mySlotId === null) return;

    const mySlot = roomInfo.player_slots[mySlotId];
    const newReadyState = !mySlot.is_ready;

    try {
      setLoading(true);
      const updatedRoom = await lobbyApi.setReady(
        roomCode,
        connectionId,
        newReadyState
      );
      setRoomInfo(updatedRoom);
      setError(null);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleStartGame = async () => {
    if (!roomCode || !connectionId) return;

    try {
      setLoading(true);
      const response: StartGameResponse = await lobbyApi.startGame(
        roomCode,
        connectionId
      );

      setGameId(response.game_id);
      localStorage.setItem("mage_game_id", response.game_id);

      setScreen("game");
      setError(null);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleLeaveRoom = async () => {
    if (!roomCode || !connectionId) return;

    if (!window.confirm("確定要離開房間嗎？")) return;

    try {
      await lobbyApi.leaveRoom(roomCode, connectionId);

      localStorage.removeItem("mage_room_code");
      localStorage.removeItem("mage_connection_id");
      localStorage.removeItem("mage_slot_id");

      setRoomCode(null);
      setConnectionId(null);
      setMySlotId(null);
      setRoomInfo(null);
      setScreen("lobby");
    } catch (err: any) {
      setError(err.message);
    }
  };

  const resetAll = () => {
    if (!window.confirm("確定要重置所有數據嗎？")) return;

    localStorage.removeItem("mage_room_code");
    localStorage.removeItem("mage_connection_id");
    localStorage.removeItem("mage_slot_id");
    localStorage.removeItem("mage_game_id");

    setRoomCode(null);
    setConnectionId(null);
    setMySlotId(null);
    setRoomInfo(null);
    setGameId(null);
    setGameInfo(null);
    setScreen("lobby");
  };

  // Game actions
  const addLog = (msg: string) => {
    const timestamp = new Date().toLocaleTimeString();
    setGameLog((prev) => [...prev, `[${timestamp}] ${msg}`]);
  };

  const allocateAttr = async (attr: AttributeType) => {
    if (!gameId) return;
    try {
      setLoading(true);
      const result = await gameApi.allocateAttribute(gameId, attr);
      addLog(result.message);
      await loadGameInfo();
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  // Card selection state
  const [selectedCard, setSelectedCard] = useState<number | null>(null);
  const [cardAction, setCardAction] = useState<
    "top" | "bottom" | "bolt" | null
  >(null);
  const [selectedTargets, setSelectedTargets] = useState<number[]>([]);

  // Card hover state for popup
  const [hoveredCard, setHoveredCard] = useState<number | null>(null);
  const [hoverTimeout, setHoverTimeout] = useState<NodeJS.Timeout | null>(null);

  // Game log state
  const [gameLog, setGameLog] = useState<string[]>([]);

  const playBoltWithCard = async (cardId: number, attr: AttributeType) => {
    if (!gameId) return;
    try {
      setLoading(true);
      const result = await gameApi.playAttributeBolt(gameId, cardId, attr);
      addLog(result.message);
      setSelectedCard(null);
      setCardAction(null);
      setSelectedTargets([]);
      await loadGameInfo();
      // Auto-draw card
      await drawCardAction();
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const playSpell = async (
    cardId: number,
    side: "Top" | "Bottom",
    targets: number[]
  ) => {
    if (!gameId) return;
    try {
      setLoading(true);
      const result = await gameApi.playSpellCard(gameId, cardId, side, targets);
      addLog(result.message);
      setSelectedCard(null);
      setCardAction(null);
      setSelectedTargets([]);
      await loadGameInfo();
      // Auto-draw card
      await drawCardAction();
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const useLiberationSkill = async () => {
    if (!gameId) return;
    try {
      setLoading(true);
      const result = await gameApi.useLiberationSkill(gameId, []);
      addLog(result.message);
      await loadGameInfo();
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const drawCardAction = async () => {
    if (!gameId) return;
    try {
      setLoading(true);
      const result = await gameApi.drawCard(gameId);
      addLog(result.message);
      await loadGameInfo();
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  // Card hover handlers
  const handleCardMouseEnter = (cardId: number) => {
    const timeout = setTimeout(() => {
      setHoveredCard(cardId);
    }, 1000); // 1 second delay
    setHoverTimeout(timeout);
  };

  const handleCardMouseLeave = () => {
    if (hoverTimeout) {
      clearTimeout(hoverTimeout);
      setHoverTimeout(null);
    }
    setHoveredCard(null);
  };

  // ==================== LOBBY SCREEN ====================
  if (screen === "lobby") {
    return (
      <div className="App">
        <div className="welcome">
          {error && <div className="error">{error}</div>}
          <h1>🧙‍♂️ MageBattle 法師對戰 ⚔️</h1>
          <p>4人2v2團隊對戰卡牌遊戲</p>

          <div className="name-input-container">
            <input
              type="text"
              placeholder="輸入你的名字"
              value={playerName}
              onChange={(e) =>
                setPlayerName(e.target.value ?? generateRandomUsername())
              }
              className="name-input-merged"
            />
          </div>

          <div className="lobby-container">
            <div className="lobby-card">
              <button
                onClick={handleCreateRoom}
                disabled={loading}
                className="btn-primary"
              >
                {loading ? "創建中..." : "創建新房間"}
              </button>
            </div>

            <div className="lobby-divider">或</div>

            <div className="lobby-card">
              <input
                type="text"
                placeholder="輸入6位房間代碼"
                value={roomCodeInput}
                onChange={(e) => setRoomCodeInput(e.target.value)}
                onKeyPress={(e) => e.key === "Enter" && handleJoinRoom()}
                maxLength={6}
              />
              <button
                onClick={handleJoinRoom}
                disabled={loading}
                className="btn-primary"
              >
                {loading ? "加入中..." : "加入房間"}
              </button>
            </div>
          </div>

          <div className="instructions">
            <h3>📖 遊戲說明</h3>
            <ul>
              <li>創建房間或使用代碼加入房間</li>
              <li>每個瀏覽器控制一個角色</li>
              <li>4位玩家分成2隊（P1&P3 vs P2&P4）</li>
              <li>每回合：分配屬性點 → 出牌/使用解放 → 抽卡</li>
              <li>擊敗對方隊伍的兩位玩家獲勝</li>
            </ul>
          </div>
        </div>
      </div>
    );
  }

  // ==================== WAITING ROOM SCREEN ====================
  if (screen === "waiting_room" && roomInfo && mySlotId !== null) {
    const mySlot = roomInfo.player_slots[mySlotId];
    const isHost = mySlotId === roomInfo.host_slot;

    // Check if all players ready
    const allReady = roomInfo.player_slots
      .filter((s) => s.is_occupied)
      .every((s) => s.is_ready);

    // Check if all players are on the same team
    const occupiedSlots = roomInfo.player_slots.filter((s) => s.is_occupied);
    const teams = new Set(
      occupiedSlots.filter((s) => s.team).map((s) => s.team)
    );
    const allSameTeam = teams.size === 1;

    return (
      <div className="App">
        <div className="waiting-room">
          <div className="waiting-room-header">
            <h1>🧙‍♂️ 等待室</h1>
            <div className="room-code-compact">
              房間代碼: <span className="code">{roomCode}</span>
            </div>
          </div>

          {error && <div className="error">{error}</div>}

          {/* Character Selection Area */}
          <div className="character-selection-area">
            <h3>選擇你的角色</h3>
            <div className="character-grid">
              {characterTypes.map((char) => (
                <button
                  key={char.id}
                  onClick={() => handleSelectCharacter(char.id)}
                  disabled={
                    loading || mySlot.character === char.id || mySlot.is_ready
                  }
                  className={`character-card ${
                    mySlot.character === char.id ? "selected" : ""
                  }`}
                >
                  <div className="character-name">{char.name}</div>
                  {mySlot.character === char.id && (
                    <div className="selected-mark">✓ 已選擇</div>
                  )}
                </button>
              ))}
            </div>
          </div>

          {/* Player List Table */}
          <div className="players-table-container">
            <h3>玩家列表</h3>
            <table className="players-table">
              <thead>
                <tr>
                  <th>玩家名稱</th>
                  <th>角色</th>
                  <th>隊伍</th>
                  <th>準備</th>
                </tr>
              </thead>
              <tbody>
                {roomInfo.player_slots
                  .filter((slot) => slot.is_occupied)
                  .map((slot) => {
                    const isMe = slot.slot_id === mySlotId;
                    return (
                      <tr
                        key={slot.slot_id}
                        className={`${isMe ? "my-row" : ""} ${
                          slot.slot_id === roomInfo.host_slot ? "host-row" : ""
                        }`}
                      >
                        <td>
                          {slot.player_name}
                          {slot.slot_id === roomInfo.host_slot && (
                            <span className="host-badge-inline">👑</span>
                          )}
                        </td>
                        <td>
                          {slot.character_title || (
                            <span className="not-selected">未選擇</span>
                          )}
                        </td>
                        <td>
                          {isMe ? (
                            <div className="team-select-inline">
                              {["A", "B", "C", "D"].map((team) => (
                                <button
                                  key={team}
                                  onClick={() => handleSelectTeam(team)}
                                  disabled={
                                    loading ||
                                    mySlot.team === team ||
                                    mySlot.is_ready
                                  }
                                  className={`team-btn-mini team-${team} ${
                                    mySlot.team === team ? "selected" : ""
                                  }`}
                                  title={`隊伍 ${team}`}
                                >
                                  {team}
                                </button>
                              ))}
                            </div>
                          ) : slot.team ? (
                            <span
                              className={`team-badge-inline team-${slot.team}`}
                            >
                              {slot.team}
                            </span>
                          ) : (
                            <span className="not-selected">未選擇</span>
                          )}
                        </td>
                        <td>
                          {isMe ? (
                            <button
                              onClick={handleToggleReady}
                              disabled={
                                loading || !mySlot.character || !mySlot.team
                              }
                              className={`ready-btn-inline ${
                                mySlot.is_ready ? "ready" : "not-ready"
                              }`}
                              title={
                                !mySlot.character && !mySlot.team
                                  ? "請先選擇角色和隊伍"
                                  : !mySlot.character
                                  ? "請先選擇角色"
                                  : !mySlot.team
                                  ? "請先選擇隊伍"
                                  : ""
                              }
                            >
                              {mySlot.is_ready ? "已準備" : "準備"}
                            </button>
                          ) : (
                            <span
                              className={`status-badge ${
                                slot.is_ready ? "ready" : "not-ready"
                              }`}
                            >
                              {slot.is_ready ? "已準備" : "未準備"}
                            </span>
                          )}
                        </td>
                      </tr>
                    );
                  })}
              </tbody>
            </table>
          </div>

          <div className="waiting-room-actions">
            {isHost && (
              <button
                onClick={handleStartGame}
                disabled={loading || !allReady || allSameTeam}
                className="btn-success"
                title={
                  !allReady
                    ? "等待所有玩家準備"
                    : allSameTeam
                    ? "需要至少2個不同隊伍"
                    : ""
                }
              >
                開始遊戲
              </button>
            )}

            <button onClick={handleLeaveRoom} className="btn-secondary">
              離開房間
            </button>
          </div>
        </div>
      </div>
    );
  }

  // ==================== GAME SCREEN ====================
  if (screen === "game" && gameInfo && mySlotId !== null) {
    const currentPlayer = gameInfo.players[gameInfo.current_player_index];
    const myPlayer = gameInfo.players[mySlotId];
    const isMyTurn = gameInfo.current_player_index === mySlotId;

    return (
      <div className="App">
        <header className="game-header">
          <div className="header-left">
            <h1>🧙‍♂️ MageBattle</h1>
          </div>
          <div className="game-stats">
            <span>回合 {gameInfo.turn_number}</span>
            <span>階段: {gameInfo.turn_phase}</span>
            <span className={isMyTurn ? "current-turn" : ""}>
              當前: {currentPlayer.name}
            </span>
            <span>🗑️ 棄牌: {myPlayer.discard_pile_count}</span>
            <span>
              📚 牌庫: {60 - myPlayer.hand.length - myPlayer.discard_pile_count}
            </span>
          </div>
          <div className="header-actions">
            <button onClick={resetAll} className="btn-small btn-warning">
              重置
            </button>
          </div>
        </header>

        {error && <div className="error">{error}</div>}

        <div className="game-layout">
          {/* LEFT: Players List & Log */}
          <div className="players-list-panel">
            <h3>玩家列表</h3>
            <div className="players-list">
              {gameInfo.players.map((player) => {
                const isCurrentTurn =
                  player.id === gameInfo.current_player_index;
                const isAllocatingPhase =
                  gameInfo.turn_phase === "AllocateAttribute" &&
                  isCurrentTurn &&
                  isMyTurn;
                const canAllocate = isAllocatingPhase;

                return (
                  <div
                    key={player.id}
                    className={`player-row ${
                      player.id === mySlotId ? "my-player-row" : ""
                    }`}
                  >
                    <div className="player-row-header">
                      <div className="player-name-hp">
                        <div className="player-name-char">
                          <strong>{player.name}</strong>
                          <div className="char-title-small">
                            {player.character.title}
                          </div>
                        </div>
                        <div className="hp-shield-inline">
                          <span className="hp-text">
                            ❤️ {player.hp}/{player.max_hp}
                          </span>
                          {player.shield > 0 && (
                            <span className="shield-text">
                              | 🛡️ {player.shield}
                            </span>
                          )}
                        </div>
                      </div>
                      <div className="player-meta-info">
                        {player.character.is_liberated && (
                          <span className="liberated-badge-small">✨</span>
                        )}
                        <span className="card-count-badge">
                          🎴 {player.hand.length}
                        </span>
                      </div>
                    </div>

                    <div className="player-attributes-row">
                      {(
                        [
                          "fire",
                          "wood",
                          "thunder",
                          "water",
                          "wind",
                          "poison",
                        ] as const
                      ).map((attr) => {
                        const attrCap = (attr.charAt(0).toUpperCase() +
                          attr.slice(1)) as AttributeType;
                        return (
                          <div
                            key={attr}
                            className={`attr-inline ${
                              canAllocate ? "clickable" : ""
                            }`}
                            onClick={() => canAllocate && allocateAttr(attrCap)}
                            title={
                              canAllocate
                                ? `點擊分配到${attributeNames[attrCap]}`
                                : ""
                            }
                          >
                            {attr === "fire" && "🔥"}
                            {attr === "wood" && "🌳"}
                            {attr === "thunder" && "⚡"}
                            {attr === "water" && "💧"}
                            {attr === "wind" && "🌪️"}
                            {attr === "poison" && "☠️"}
                            <span>
                              {
                                player.attributes[
                                  attr as keyof typeof player.attributes
                                ]
                              }
                            </span>
                          </div>
                        );
                      })}
                    </div>

                    {player.is_dead && (
                      <div className="death-status-inline">
                        💀 已死亡 ({player.death_turns}/2)
                      </div>
                    )}
                  </div>
                );
              })}
            </div>
          </div>

          {/* RIGHT: Card Deck & Actions */}
          <div className="cards-panel">
            <div className="my-hand-section">
              <h3>🎴 我的手牌 ({myPlayer.hand.length}/5)</h3>
              <div className="hand-cards-grid">
                {myPlayer.hand.length > 0 ? (
                  myPlayer.hand.map((cardId) => {
                    const cardInfo = getCardInfo(cardId);
                    const canClick =
                      isMyTurn && gameInfo.turn_phase === "PlayCard";
                    return (
                      <div
                        key={cardId}
                        className={`hand-card-clickable ${
                          canClick ? "can-select" : "disabled"
                        }`}
                        onClick={() => canClick && setSelectedCard(cardId)}
                        onMouseEnter={() =>
                          canClick && handleCardMouseEnter(cardId)
                        }
                        onMouseLeave={handleCardMouseLeave}
                      >
                        {cardInfo ? (
                          <>
                            <div className="card-spell-name">
                              {cardInfo.top_spell.name}
                            </div>
                            <div className="card-cost">
                              {cardInfo.top_spell.cost}
                            </div>
                            {cardInfo.bottom_spell && (
                              <>
                                <div className="card-divider">---</div>
                                <div className="card-spell-name">
                                  {cardInfo.bottom_spell.name}
                                </div>
                                <div className="card-cost">
                                  {cardInfo.bottom_spell.cost}
                                </div>
                              </>
                            )}
                          </>
                        ) : (
                          <div className="card-label">卡片</div>
                        )}

                        {/* Hover Popup */}
                        {hoveredCard === cardId && cardInfo && (
                          <div className="card-hover-popup">
                            <div className="popup-section">
                              <strong>上: {cardInfo.top_spell.name}</strong>
                              <div>消耗: {cardInfo.top_spell.cost}</div>
                              <div>效果: {cardInfo.top_spell.effect}</div>
                            </div>
                            {cardInfo.bottom_spell && (
                              <div className="popup-section">
                                <strong>
                                  下: {cardInfo.bottom_spell.name}
                                </strong>
                                <div>消耗: {cardInfo.bottom_spell.cost}</div>
                                <div>效果: {cardInfo.bottom_spell.effect}</div>
                              </div>
                            )}
                          </div>
                        )}
                      </div>
                    );
                  })
                ) : (
                  <div className="no-cards">手牌為空</div>
                )}
              </div>
            </div>

            {/* OLD PLAYERS GRID REMOVED */}
            <div style={{ display: "none" }} className="players-grid-old">
              {gameInfo.players.map((player) => (
                <div
                  key={player.id}
                  className={`player-card ${
                    player.id === mySlotId ? "my-player" : ""
                  } ${
                    player.id === gameInfo.current_player_index
                      ? "current-turn-player"
                      : ""
                  } ${player.is_dead ? "dead" : ""}`}
                >
                  <div className="player-header">
                    <h3>{player.name}</h3>
                    <span
                      className={`team-badge ${
                        player.team === "Team0" ? "team-a" : "team-b"
                      }`}
                    >
                      {player.team === "Team0" ? "隊伍A" : "隊伍B"}
                    </span>
                  </div>

                  <div className="character-info">
                    <div className="character-title">
                      {player.character.title}
                    </div>
                    {player.character.is_liberated && (
                      <div className="liberated-badge">✨ 已解放</div>
                    )}
                  </div>

                  <div className="hp-bar">
                    <div className="hp-text">
                      ❤️ {player.hp}/{player.max_hp}
                    </div>
                    <div className="bar">
                      <div
                        className="bar-fill hp-fill"
                        style={{
                          width: `${(player.hp / player.max_hp) * 100}%`,
                        }}
                      ></div>
                    </div>
                    {player.shield > 0 && (
                      <div className="shield-text">🛡️ {player.shield}</div>
                    )}
                  </div>

                  {player.is_dead && (
                    <div className="death-status">
                      💀 已死亡 ({player.death_turns}/2 回合後復活)
                    </div>
                  )}

                  <div className="attributes-grid">
                    <div className="attr">🔥 火{player.attributes.fire}</div>
                    <div className="attr">🌳 木{player.attributes.wood}</div>
                    <div className="attr">⚡ 雷{player.attributes.thunder}</div>
                    <div className="attr">💧 水{player.attributes.water}</div>
                    <div className="attr">🌪️ 風{player.attributes.wind}</div>
                    <div className="attr">☠️ 毒{player.attributes.poison}</div>
                  </div>

                  <div className="hand-info">
                    🎴 手牌: {player.hand_count}張
                    {player.id === mySlotId && player.hand.length > 0 && (
                      <div className="my-hand">
                        {player.hand.map((cardId, idx) => (
                          <div key={idx} className="card-mini">
                            #{cardId}
                          </div>
                        ))}
                      </div>
                    )}
                  </div>

                  {player.buffs.length > 0 && (
                    <div className="buffs">
                      {player.buffs.map((buff, idx) => (
                        <div key={idx} className="buff-badge">
                          {buff.name}
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              ))}
            </div>

            {/* Card Actions (only for PlayCard phase) */}
            {isMyTurn &&
              myPlayer.can_act &&
              gameInfo.turn_phase === "PlayCard" && (
                <div className="card-actions-section">
                  {selectedCard ? (
                    <div>
                      <h4>已選擇卡片</h4>
                      {(() => {
                        const cardInfo = getCardInfo(selectedCard);
                        if (!cardInfo) return null;

                        // If no action chosen yet, show 3-part popup
                        if (!cardAction) {
                          return (
                            <div className="three-part-card-popup">
                              <div className="card-popup-parts">
                                {/* Left Part: Top Spell */}
                                <div className="card-part spell-part">
                                  <h5>{cardInfo.top_spell.name}</h5>
                                  <div className="spell-requirement">
                                    需求: {cardInfo.top_spell.cost}
                                  </div>
                                  <div className="spell-effect">
                                    {cardInfo.top_spell.effect}
                                  </div>
                                  <button
                                    onClick={() => setCardAction("top")}
                                    disabled={
                                      loading ||
                                      !checkSpellRequirement(
                                        cardInfo.top_spell.cost,
                                        myPlayer.attributes
                                      )
                                    }
                                    className="btn-spell-confirm"
                                    title={
                                      !checkSpellRequirement(
                                        cardInfo.top_spell.cost,
                                        myPlayer.attributes
                                      )
                                        ? "屬性等級不足"
                                        : ""
                                    }
                                  >
                                    確認
                                  </button>
                                </div>

                                {/* Middle Part: Bottom Spell (if exists) */}
                                {cardInfo.bottom_spell && (
                                  <div className="card-part spell-part">
                                    <h5>{cardInfo.bottom_spell.name}</h5>
                                    <div className="spell-requirement">
                                      需求: {cardInfo.bottom_spell.cost}
                                    </div>
                                    <div className="spell-effect">
                                      {cardInfo.bottom_spell.effect}
                                    </div>
                                    <button
                                      onClick={() => setCardAction("bottom")}
                                      disabled={
                                        loading ||
                                        !checkSpellRequirement(
                                          cardInfo.bottom_spell.cost,
                                          myPlayer.attributes
                                        )
                                      }
                                      className="btn-spell-confirm"
                                      title={
                                        !checkSpellRequirement(
                                          cardInfo.bottom_spell.cost,
                                          myPlayer.attributes
                                        )
                                          ? "屬性等級不足"
                                          : ""
                                      }
                                    >
                                      確認
                                    </button>
                                  </div>
                                )}

                                {/* Right Part: Attribute Bolts */}
                                <div className="card-part bolts-part">
                                  <h5>屬性彈</h5>
                                  <div className="bolt-buttons-grid">
                                    {(
                                      [
                                        "Fire",
                                        "Wood",
                                        "Thunder",
                                        "Water",
                                        "Wind",
                                        "Poison",
                                      ] as AttributeType[]
                                    ).map((attr) => {
                                      const attrValue =
                                        myPlayer.attributes[
                                          attr.toLowerCase() as keyof typeof myPlayer.attributes
                                        ];
                                      return (
                                        <button
                                          key={attr}
                                          onClick={() =>
                                            playBoltWithCard(selectedCard, attr)
                                          }
                                          disabled={loading || attrValue === 0}
                                          className="btn-bolt"
                                          title={
                                            attrValue === 0
                                              ? "此屬性等級為0"
                                              : `使用${attributeNames[attr]}彈 (Lv${attrValue})`
                                          }
                                        >
                                          {attributeNames[attr]}
                                          <span className="bolt-level">
                                            Lv{attrValue}
                                          </span>
                                        </button>
                                      );
                                    })}
                                  </div>
                                </div>
                              </div>

                              <button
                                onClick={() => {
                                  setSelectedCard(null);
                                  setCardAction(null);
                                  setSelectedTargets([]);
                                }}
                                className="btn-secondary btn-small"
                              >
                                取消
                              </button>
                            </div>
                          );
                        }

                        // If spell action chosen, show target selection
                        if (cardAction === "top" || cardAction === "bottom") {
                          const spell =
                            cardAction === "top"
                              ? cardInfo.top_spell
                              : cardInfo.bottom_spell;
                          if (!spell) return null;

                          return (
                            <div className="target-selection-popup">
                              <h4>選擇目標 - {spell.name}</h4>
                              <div className="spell-info-compact">
                                <div>需求: {spell.cost}</div>
                                <div>效果: {spell.effect}</div>
                              </div>

                              <div className="target-grid">
                                {gameInfo.players.map((player) => {
                                  const isValidTarget = !player.is_dead; // Simplified - adjust based on spell targeting rules
                                  const isSelected = selectedTargets.includes(
                                    player.id
                                  );

                                  return (
                                    <div
                                      key={player.id}
                                      className={`target-option ${
                                        isValidTarget ? "valid" : "invalid"
                                      } ${isSelected ? "selected" : ""}`}
                                      onClick={() => {
                                        if (isValidTarget) {
                                          // Toggle selection
                                          if (isSelected) {
                                            setSelectedTargets(
                                              selectedTargets.filter(
                                                (id) => id !== player.id
                                              )
                                            );
                                          } else {
                                            setSelectedTargets([player.id]); // Single target for now
                                          }
                                        }
                                      }}
                                    >
                                      <span>{player.name}</span>
                                      <span className="target-hp">
                                        ❤️ {player.hp}/{player.max_hp}
                                      </span>
                                      {isSelected && (
                                        <span className="selected-mark">✓</span>
                                      )}
                                    </div>
                                  );
                                })}
                              </div>

                              <div className="action-buttons">
                                <button
                                  onClick={() =>
                                    playSpell(
                                      selectedCard,
                                      cardAction === "top" ? "Top" : "Bottom",
                                      selectedTargets
                                    )
                                  }
                                  disabled={
                                    loading || selectedTargets.length === 0
                                  }
                                  className="btn-primary"
                                >
                                  確認使用
                                </button>
                                <button
                                  onClick={() => {
                                    setCardAction(null);
                                    setSelectedTargets([]);
                                  }}
                                  className="btn-secondary btn-small"
                                >
                                  返回
                                </button>
                              </div>
                            </div>
                          );
                        }

                        return null;
                      })()}
                    </div>
                  ) : null}
                </div>
              )}

            {/* Liberation Section */}
            {myPlayer.character.is_liberated && myPlayer.can_liberate && (
              <div className="liberation-section">
                <h3>✨ 解放技能</h3>
                <p>你的角色已解放，可使用特殊技能！</p>
                <button
                  onClick={useLiberationSkill}
                  disabled={loading}
                  className="btn-liberation"
                >
                  使用解放技能
                </button>
              </div>
            )}
          </div>

          {/* RIGHT: Game Log Column */}
          <div className="game-log-panel">
            <h3>遊戲記錄</h3>
            <div className="game-log-scroll">
              {gameLog.length === 0 ? (
                <div className="log-empty">等待行動...</div>
              ) : (
                gameLog.map((log, idx) => (
                  <div key={idx} className="log-entry">
                    {log}
                  </div>
                ))
              )}
            </div>
          </div>
        </div>
      </div>
    );
  }

  // Loading/Error state
  return (
    <div className="App">
      <div className="welcome">
        <h1>載入中...</h1>
        {error && <div className="error">{error}</div>}
        <button onClick={resetAll} className="btn-warning">
          重置
        </button>
      </div>
    </div>
  );
}

export default App;
