import React, { useState, useEffect } from 'react';
import { gameApi, GameInfo, ActionResult } from './api/gameApi';
import { lobbyApi, RoomDto, CreateRoomResponse, JoinRoomResponse, StartGameResponse } from './api/lobbyApi';
import './App.css';

type AttributeType = 'Fire' | 'Wood' | 'Thunder' | 'Water' | 'Wind' | 'Poison';

const attributeNames: Record<AttributeType, string> = {
  Fire: '火',
  Wood: '木',
  Thunder: '雷',
  Water: '水',
  Wind: '風',
  Poison: '毒',
};

const characterTypes = [
  { id: 'FlamePoison', name: '火毒法師' },
  { id: 'WoodWind', name: '木風法師' },
  { id: 'ThunderPoison', name: '雷毒法師' },
  { id: 'WaterWind', name: '水風法師' },
];

type AppScreen = 'lobby' | 'waiting_room' | 'game';

function App() {
  // App state
  const [screen, setScreen] = useState<AppScreen>('lobby');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [message, setMessage] = useState<string | null>(null);

  // Lobby state
  const [playerName, setPlayerName] = useState<string>('');
  const [roomCodeInput, setRoomCodeInput] = useState<string>('');

  // Room state
  const [roomCode, setRoomCode] = useState<string | null>(
    localStorage.getItem('mage_room_code')
  );
  const [connectionId, setConnectionId] = useState<string | null>(
    localStorage.getItem('mage_connection_id')
  );
  const [mySlotId, setMySlotId] = useState<number | null>(
    localStorage.getItem('mage_slot_id')
      ? parseInt(localStorage.getItem('mage_slot_id')!)
      : null
  );
  const [roomInfo, setRoomInfo] = useState<RoomDto | null>(null);

  // Game state
  const [gameId, setGameId] = useState<string | null>(
    localStorage.getItem('mage_game_id')
  );
  const [gameInfo, setGameInfo] = useState<GameInfo | null>(null);

  // Auto-refresh room info when in waiting room
  useEffect(() => {
    if (screen === 'waiting_room' && roomCode) {
      loadRoomInfo();
      const interval = setInterval(loadRoomInfo, 2000);
      return () => clearInterval(interval);
    }
  }, [screen, roomCode]);

  // Auto-refresh game info when in game
  useEffect(() => {
    if (screen === 'game' && gameId) {
      loadGameInfo();
      const interval = setInterval(loadGameInfo, 2000);
      return () => clearInterval(interval);
    }
  }, [screen, gameId]);

  // Check if we should restore a previous session
  useEffect(() => {
    if (gameId) {
      setScreen('game');
    } else if (roomCode && connectionId && mySlotId !== null) {
      setScreen('waiting_room');
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
        localStorage.setItem('mage_game_id', info.game_id);
        setScreen('game');
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
      setError('請輸入你的名字');
      return;
    }

    try {
      setLoading(true);
      const response: CreateRoomResponse = await lobbyApi.createRoom(playerName.trim());

      setRoomCode(response.room_code);
      setConnectionId(response.connection_id);
      setMySlotId(response.slot_id);

      localStorage.setItem('mage_room_code', response.room_code);
      localStorage.setItem('mage_connection_id', response.connection_id);
      localStorage.setItem('mage_slot_id', response.slot_id.toString());

      setScreen('waiting_room');
      setError(null);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleJoinRoom = async () => {
    if (!playerName.trim()) {
      setError('請輸入你的名字');
      return;
    }
    if (!roomCodeInput.trim()) {
      setError('請輸入房間代碼');
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

      localStorage.setItem('mage_room_code', response.room_code);
      localStorage.setItem('mage_connection_id', response.connection_id);
      localStorage.setItem('mage_slot_id', response.slot_id.toString());

      setScreen('waiting_room');
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
      const updatedRoom = await lobbyApi.selectCharacter(roomCode, connectionId, characterId);
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
      const updatedRoom = await lobbyApi.setReady(roomCode, connectionId, newReadyState);
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
      const response: StartGameResponse = await lobbyApi.startGame(roomCode, connectionId);

      setGameId(response.game_id);
      localStorage.setItem('mage_game_id', response.game_id);

      setScreen('game');
      setError(null);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleLeaveRoom = async () => {
    if (!roomCode || !connectionId) return;

    if (!window.confirm('確定要離開房間嗎？')) return;

    try {
      await lobbyApi.leaveRoom(roomCode, connectionId);

      localStorage.removeItem('mage_room_code');
      localStorage.removeItem('mage_connection_id');
      localStorage.removeItem('mage_slot_id');

      setRoomCode(null);
      setConnectionId(null);
      setMySlotId(null);
      setRoomInfo(null);
      setScreen('lobby');
    } catch (err: any) {
      setError(err.message);
    }
  };

  const resetAll = () => {
    if (!window.confirm('確定要重置所有數據嗎？')) return;

    localStorage.removeItem('mage_room_code');
    localStorage.removeItem('mage_connection_id');
    localStorage.removeItem('mage_slot_id');
    localStorage.removeItem('mage_game_id');

    setRoomCode(null);
    setConnectionId(null);
    setMySlotId(null);
    setRoomInfo(null);
    setGameId(null);
    setGameInfo(null);
    setScreen('lobby');
  };

  // Game actions
  const allocateAttr = async (attr: AttributeType) => {
    if (!gameId) return;
    try {
      setLoading(true);
      const result = await gameApi.allocateAttribute(gameId, attr);
      setMessage(result.message);
      await loadGameInfo();
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  // Card selection state
  const [selectedCard, setSelectedCard] = useState<number | null>(null);
  const [cardAction, setCardAction] = useState<'top' | 'bottom' | 'bolt' | null>(null);

  const playBoltWithCard = async (cardId: number, attr: AttributeType) => {
    if (!gameId) return;
    try {
      setLoading(true);
      const result = await gameApi.playAttributeBolt(gameId, cardId, attr);
      setMessage(result.message);
      setSelectedCard(null);
      setCardAction(null);
      await loadGameInfo();
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const playSpell = async (cardId: number, side: 'Top' | 'Bottom', targets: number[]) => {
    if (!gameId) return;
    try {
      setLoading(true);
      const result = await gameApi.playSpellCard(gameId, cardId, side, targets);
      setMessage(result.message);
      setSelectedCard(null);
      setCardAction(null);
      await loadGameInfo();
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
      setMessage(result.message);
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
      setMessage(result.message);
      await loadGameInfo();
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  // ==================== LOBBY SCREEN ====================
  if (screen === 'lobby') {
    return (
      <div className="App">
        <div className="welcome">
          <h1>🧙‍♂️ MageBattle 法師對戰 ⚔️</h1>
          <p>4人2v2團隊對戰卡牌遊戲</p>

          <div className="lobby-container">
            <div className="lobby-card">
              <h2>創建房間</h2>
              <input
                type="text"
                placeholder="輸入你的名字"
                value={playerName}
                onChange={(e) => setPlayerName(e.target.value)}
                onKeyPress={(e) => e.key === 'Enter' && handleCreateRoom()}
              />
              <button
                onClick={handleCreateRoom}
                disabled={loading}
                className="btn-primary"
              >
                {loading ? '創建中...' : '創建新房間'}
              </button>
            </div>

            <div className="lobby-divider">或</div>

            <div className="lobby-card">
              <h2>加入房間</h2>
              <input
                type="text"
                placeholder="輸入你的名字"
                value={playerName}
                onChange={(e) => setPlayerName(e.target.value)}
              />
              <input
                type="text"
                placeholder="輸入6位房間代碼"
                value={roomCodeInput}
                onChange={(e) => setRoomCodeInput(e.target.value)}
                onKeyPress={(e) => e.key === 'Enter' && handleJoinRoom()}
                maxLength={6}
              />
              <button
                onClick={handleJoinRoom}
                disabled={loading}
                className="btn-primary"
              >
                {loading ? '加入中...' : '加入房間'}
              </button>
            </div>
          </div>

          {error && <div className="error">{error}</div>}

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
  if (screen === 'waiting_room' && roomInfo && mySlotId !== null) {
    const mySlot = roomInfo.player_slots[mySlotId];
    const isHost = mySlotId === roomInfo.host_slot;

    return (
      <div className="App">
        <div className="waiting-room">
          <h1>🧙‍♂️ 等待室</h1>

          <div className="room-code-display">
            <h2>房間代碼: <span className="code">{roomCode}</span></h2>
            <p>分享此代碼讓其他玩家加入</p>
          </div>

          {error && <div className="error">{error}</div>}

          <div className="player-slots-grid">
            {roomInfo.player_slots.map((slot, idx) => (
              <div
                key={idx}
                className={`player-slot ${slot.is_occupied ? 'occupied' : 'empty'} ${
                  idx === mySlotId ? 'my-slot' : ''
                } ${idx === roomInfo.host_slot ? 'host-slot' : ''}`}
              >
                <div className="slot-header">
                  <h3>玩家 {idx + 1}</h3>
                  {idx === roomInfo.host_slot && <span className="host-badge">房主</span>}
                  <span className="team-badge">隊伍 {idx % 2 === 0 ? 'A' : 'B'}</span>
                </div>

                {slot.is_occupied ? (
                  <>
                    <div className="player-name">{slot.player_name}</div>

                    {idx === mySlotId && !slot.character && (
                      <div className="character-selection">
                        <h4>選擇角色：</h4>
                        <div className="character-buttons">
                          {characterTypes.map((char) => (
                            <button
                              key={char.id}
                              onClick={() => handleSelectCharacter(char.id)}
                              disabled={loading}
                              className="btn-character"
                            >
                              {char.name}
                            </button>
                          ))}
                        </div>
                      </div>
                    )}

                    {slot.character && (
                      <div className="selected-character">
                        <div className="character-title">{slot.character_title}</div>
                        <div className={`ready-status ${slot.is_ready ? 'ready' : 'not-ready'}`}>
                          {slot.is_ready ? '✓ 已準備' : '等待中...'}
                        </div>
                      </div>
                    )}
                  </>
                ) : (
                  <div className="empty-slot-text">等待玩家加入...</div>
                )}
              </div>
            ))}
          </div>

          <div className="waiting-room-actions">
            {mySlot.character && (
              <button
                onClick={handleToggleReady}
                disabled={loading}
                className={mySlot.is_ready ? 'btn-warning' : 'btn-primary'}
              >
                {mySlot.is_ready ? '取消準備' : '準備'}
              </button>
            )}

            {isHost && roomInfo.can_start && (
              <button
                onClick={handleStartGame}
                disabled={loading}
                className="btn-success"
              >
                開始遊戲
              </button>
            )}

            <button onClick={handleLeaveRoom} className="btn-secondary">
              離開房間
            </button>
          </div>

          <div className="room-status">
            <p>玩家數量: {roomInfo.player_count}/4</p>
            {!roomInfo.can_start && roomInfo.player_count === 4 && (
              <p className="warning-text">請確保所有玩家都選擇了角色並標記為準備</p>
            )}
          </div>
        </div>
      </div>
    );
  }

  // ==================== GAME SCREEN ====================
  if (screen === 'game' && gameInfo && mySlotId !== null) {
    const currentPlayer = gameInfo.players[gameInfo.current_player_index];
    const myPlayer = gameInfo.players[mySlotId];
    const isMyTurn = gameInfo.current_player_index === mySlotId;

    return (
      <div className="App">
        <header className="game-header">
          <div className="header-left">
            <h1>🧙‍♂️ MageBattle</h1>
            <div className="player-indicator">
              你是: <strong>{myPlayer.name}</strong> ({myPlayer.character.title})
            </div>
          </div>
          <div className="game-stats">
            <span>回合 {gameInfo.turn_number}</span>
            <span>階段: {gameInfo.turn_phase}</span>
            <span className={isMyTurn ? 'current-turn' : ''}>
              當前玩家: {currentPlayer.name}
            </span>
          </div>
          <button onClick={resetAll} className="btn-small btn-warning">
            重置
          </button>
        </header>

        {error && <div className="error">{error}</div>}
        {message && <div className="message">{message}</div>}

        <div className="game-board">
          {/* My Hand - Always Visible */}
          <div className="my-hand-display">
            <h3>🎴 我的手牌 ({myPlayer.hand.length}/5)</h3>
            <div className="hand-cards-display">
              {myPlayer.hand.length > 0 ? (
                myPlayer.hand.map((cardId) => (
                  <div key={cardId} className="hand-card-item">
                    <div className="card-id">#{cardId}</div>
                    <div className="card-label">卡片</div>
                  </div>
                ))
              ) : (
                <div className="no-cards">手牌為空</div>
              )}
            </div>
            {myPlayer.discard_pile_count > 0 && (
              <div className="discard-info">
                🗑️ 棄牌堆: {myPlayer.discard_pile_count}張
              </div>
            )}
          </div>

          {/* Players Grid */}
          <div className="players-grid">
            {gameInfo.players.map((player) => (
              <div
                key={player.id}
                className={`player-card ${player.id === mySlotId ? 'my-player' : ''} ${
                  player.id === gameInfo.current_player_index ? 'current-turn-player' : ''
                } ${player.is_dead ? 'dead' : ''}`}
              >
                <div className="player-header">
                  <h3>{player.name}</h3>
                  <span
                    className={`team-badge ${
                      player.team === 'Team0' ? 'team-a' : 'team-b'
                    }`}
                  >
                    {player.team === 'Team0' ? '隊伍A' : '隊伍B'}
                  </span>
                </div>

                <div className="character-info">
                  <div className="character-title">{player.character.title}</div>
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
                      style={{ width: `${(player.hp / player.max_hp) * 100}%` }}
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

          {/* Action Panel */}
          {isMyTurn && myPlayer.can_act && (
            <div className="action-panel">
              <h2>你的回合</h2>

              {gameInfo.turn_phase === 'AllocateAttribute' && (
                <div className="action-section">
                  <h3>1️⃣ 分配屬性點</h3>
                  <div className="attribute-buttons">
                    {(
                      ['Fire', 'Wood', 'Thunder', 'Water', 'Wind', 'Poison'] as AttributeType[]
                    ).map((attr) => (
                      <button
                        key={attr}
                        onClick={() => allocateAttr(attr)}
                        disabled={loading}
                        className="btn-attr"
                      >
                        {attributeNames[attr]} (
                        {
                          myPlayer.attributes[
                            attr.toLowerCase() as keyof typeof myPlayer.attributes
                          ]
                        }
                        /5)
                      </button>
                    ))}
                  </div>
                </div>
              )}

              {gameInfo.turn_phase === 'PlayCard' && (
                <div className="action-section">
                  <h3>2️⃣ 出牌</h3>

                  {/* Step 1: Select Card */}
                  {!selectedCard && (
                    <div className="card-selection">
                      <h4>選擇一張卡片：</h4>
                      <div className="hand-cards">
                        {myPlayer.hand.map((cardId) => (
                          <button
                            key={cardId}
                            onClick={() => setSelectedCard(cardId)}
                            className="card-button"
                            disabled={loading}
                          >
                            卡片 #{cardId}
                          </button>
                        ))}
                      </div>
                    </div>
                  )}

                  {/* Step 2: Choose Action */}
                  {selectedCard && !cardAction && (
                    <div className="action-choice">
                      <h4>已選擇卡片 #{selectedCard}</h4>
                      <p>選擇動作：</p>
                      <div className="action-buttons">
                        <button
                          onClick={() => setCardAction('top')}
                          className="btn-action"
                          disabled={loading}
                        >
                          使用上方法術
                        </button>
                        <button
                          onClick={() => setCardAction('bottom')}
                          className="btn-action"
                          disabled={loading}
                        >
                          使用下方法術
                        </button>
                        <button
                          onClick={() => setCardAction('bolt')}
                          className="btn-action"
                          disabled={loading}
                        >
                          使用屬性彈
                        </button>
                      </div>
                      <button
                        onClick={() => setSelectedCard(null)}
                        className="btn-secondary btn-small"
                      >
                        取消
                      </button>
                    </div>
                  )}

                  {/* Step 3: Execute Action */}
                  {selectedCard && cardAction === 'bolt' && (
                    <div className="attribute-bolt-choice">
                      <h4>卡片 #{selectedCard} - 使用屬性彈</h4>
                      <p>攻擊目標：前一位玩家 ({gameInfo.players[(mySlotId + 3) % 4]?.name || '未知'})</p>
                      <p>選擇屬性：</p>
                      <div className="attribute-buttons">
                        {(
                          ['Fire', 'Wood', 'Thunder', 'Water', 'Wind', 'Poison'] as AttributeType[]
                        ).map((attr) => {
                          const attrValue =
                            myPlayer.attributes[
                              attr.toLowerCase() as keyof typeof myPlayer.attributes
                            ];
                          return (
                            attrValue > 0 && (
                              <button
                                key={attr}
                                onClick={() => playBoltWithCard(selectedCard, attr)}
                                disabled={loading}
                                className="btn-attr"
                              >
                                {attributeNames[attr]} (Lv{attrValue})
                              </button>
                            )
                          );
                        })}
                      </div>
                      <button
                        onClick={() => setCardAction(null)}
                        className="btn-secondary btn-small"
                      >
                        返回
                      </button>
                    </div>
                  )}

                  {selectedCard && (cardAction === 'top' || cardAction === 'bottom') && (
                    <div className="spell-choice">
                      <h4>卡片 #{selectedCard} - 使用{cardAction === 'top' ? '上方' : '下方'}法術</h4>
                      <p>TODO: 實現法術系統（目前先簡化執行）</p>
                      <button
                        onClick={() => playSpell(selectedCard, cardAction === 'top' ? 'Top' : 'Bottom', [])}
                        disabled={loading}
                        className="btn-primary"
                      >
                        確認使用法術
                      </button>
                      <button
                        onClick={() => setCardAction(null)}
                        className="btn-secondary btn-small"
                      >
                        返回
                      </button>
                    </div>
                  )}

                  {/* Liberation */}
                  {!selectedCard && myPlayer.can_liberate && (
                    <div className="sub-section liberation-section">
                      <h4>✨ 解放技能</h4>
                      <p>{myPlayer.character.liberation_name}</p>
                      <button
                        onClick={useLiberationSkill}
                        disabled={loading}
                        className="btn-primary btn-liberation"
                      >
                        使用解放技能
                      </button>
                    </div>
                  )}
                </div>
              )}

              {gameInfo.turn_phase === 'DrawCard' && (
                <div className="action-section">
                  <h3>3️⃣ 抽卡</h3>
                  <button
                    onClick={drawCardAction}
                    disabled={loading}
                    className="btn-primary"
                  >
                    抽一張卡
                  </button>
                </div>
              )}
            </div>
          )}

          {!isMyTurn && (
            <div className="waiting-panel">
              <h2>等待 {currentPlayer.name} 的回合...</h2>
              <div className="spinner"></div>
            </div>
          )}
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
