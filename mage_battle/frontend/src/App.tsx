import React, { useState, useEffect, useRef } from "react";
import { gameApi, GameInfo, ActionResult } from "./api/gameApi";
import {
  lobbyApi,
  RoomDto,
  CreateRoomResponse,
  JoinRoomResponse,
  StartGameResponse,
} from "./api/lobbyApi";
import { testApi, TestScenarioInfo } from "./api/testApi";
import cardsDataRaw from "./cards.json";
import spellsDataRaw from "./spells.json";
import Guide from "./components/Guide";
import "./App.css";

type AttributeType = "Fire" | "Wood" | "Thunder" | "Water" | "Wind" | "Poison";

// Spell data types
interface CardEffect {
  type: string;
  params: (number | string)[];
}

interface CardSpell {
  spell_id: string;
  name: string;
  cost: string;
  description: string;
  target_pool?: string;
  effects?: CardEffect[];
  is_attribute_bolt?: boolean;
}

// Card reference type
interface CardReference {
  id: number;
  top_spell_id: string;
  bottom_spell_id?: string | null;
}

// Full card data with spell details
interface CardData {
  id: number;
  top_spell: CardSpell;
  bottom_spell: CardSpell | null;
}

// Parse the JSON data
const spellsMap = new Map<string, CardSpell>();
(spellsDataRaw as any).spells.forEach((spell: CardSpell) => {
  spellsMap.set(spell.spell_id, spell);
});

// Build full card data by resolving spell references
const cardData: CardData[] = (cardsDataRaw as any).cards.map((cardRef: CardReference) => {
  const topSpell = spellsMap.get(cardRef.top_spell_id);
  const bottomSpell = cardRef.bottom_spell_id ? spellsMap.get(cardRef.bottom_spell_id) : null;

  if (!topSpell) {
    console.error(`Spell ${cardRef.top_spell_id} not found for card ${cardRef.id}`);
  }

  return {
    id: cardRef.id,
    top_spell: topSpell || {
      spell_id: cardRef.top_spell_id,
      name: "未知法術",
      cost: "火1",
      description: "錯誤：法術數據未找到",
      target_pool: "Default",
      effects: []
    },
    bottom_spell: bottomSpell || null
  };
});

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

// Attribute proficiency info
const attributeProficiency: Record<
  string,
  { level: number; description: string }[]
> = {
  fire: [
    { level: 3, description: "所有屬性彈+1" },
    { level: 5, description: "回合開始時，對所有敵人造成1點傷害" },
  ],
  wood: [
    { level: 3, description: "減少1點生命並獲得1點護盾" },
    { level: 5, description: "自己與隊友受到的卡片傷害-1" },
  ],
  thunder: [
    { level: 3, description: "雷屬性卡片傷害+1" },
    { level: 5, description: "雷屬性卡片傷害+2 (合計+3)" },
  ],
  water: [
    { level: 3, description: "使用水屬卡片時，回復自身1的生命" },
    { level: 5, description: "使用水屬卡片時，回復自己與隊友1點生命" },
  ],
  wind: [
    { level: 2, description: "風屬性卡片攻擊的人這圈不能回復生命或獲得護盾" },
    { level: 5, description: "風屬性卡片可自由選擇對象（不局限於攻擊右邊）" },
  ],
  poison: [
    { level: 2, description: "被毒屬性卡片攻擊的人下回合先出卡片再配屬性點" },
    { level: 5, description: "被毒屬性卡片攻擊的人下回合只能出屬性彈" },
  ],
};

// Buff detailed descriptions
const buffDescriptions: Record<string, string> = {
  Immune: "免疫：免疫所有傷害和負面效果",
  Invincible: "無敵：無法被擊敗，生命值不會降至0以下",
  Paralysis: "麻痹：無法行動，跳過回合",
  Seal: "封印：無法使用卡片技能",
  Silent: "沉默：無法使用解放技能",
  MasterDisable: "大師禁用：無法使用5級或以上技能",
  DefenseInvalidation: "防禦無效化：護盾無效，直接受到傷害",
  Confuse: "混亂：回合開始時必須先出卡片再分配屬性點",
  HealthDrain: "生命汲取：每回合獲得生命值",
  HealthDrainTarget: "被生命汲取：每回合失去生命值",
  Regeneration: "再生：每回合恢復生命值",
  BurningOut: "燃盡：持續受到火焰傷害",
  GuardWoodCarving: "守護木雕：提供額外護盾保護",
};

// Helper function to get card information
const getCardInfo = (cardId: number) => {
  return cardData.find((card) => card.id === cardId);
};

// Helper function to check if spell requirements are met
const checkSpellRequirement = (
  cost: string,
  playerAttributes: any
): boolean => {
  // Parse cost like "火1", "火2木1", "風5", etc.
  // Match all patterns of attribute character followed by digit(s)
  const pattern = /([火木雷水風毒])(\d+)/g;
  const matches = Array.from(cost.matchAll(pattern));

  if (matches.length === 0) return true; // If can't parse, assume it can be cast

  // Check ALL requirements must be met
  return matches.every((match) => {
    const attrChar = match[1];
    const requiredLevel = parseInt(match[2]);
    const attrType = attributeNamesReverse[attrChar];

    if (!attrType) return true; // Unknown attribute, assume can be cast

    const playerAttrLevel =
      playerAttributes[attrType.toLowerCase() as keyof typeof playerAttributes];
    return playerAttrLevel >= requiredLevel;
  });
};

// Helper function to get activated attribute proficiencies for a spell
const getActivatedProficiencies = (
  spellCost: string,
  playerAttributes: any
): string[] => {
  const activated: string[] = [];

  // Parse spell cost to find which attributes are involved
  const pattern = /([火木雷水風毒])(\d+)/g;
  const matches = Array.from(spellCost.matchAll(pattern));

  const involvedAttributes = new Set<string>();
  matches.forEach((match) => {
    const attrChar = match[1];
    const attrType = attributeNamesReverse[attrChar];
    if (attrType) {
      involvedAttributes.add(attrType.toLowerCase());
    }
  });

  // Check each attribute's proficiency levels
  Object.entries(attributeProficiency).forEach(([attr, proficiencies]) => {
    // Only show proficiencies for attributes involved in this spell
    if (!involvedAttributes.has(attr)) return;

    const playerLevel = playerAttributes[attr as keyof typeof playerAttributes];

    proficiencies.forEach((prof) => {
      // Skip passive skills: Wood Lv5 and Fire Lv5
      if (
        (attr === "wood" && prof.level === 5) ||
        (attr === "fire" && prof.level === 5)
      ) {
        return;
      }

      if (playerLevel >= prof.level) {
        activated.push(
          `${
            attributeNames[
              (attr.charAt(0).toUpperCase() + attr.slice(1)) as AttributeType
            ]
          } Lv${prof.level}: ${prof.description}`
        );
      }
    });
  });

  return activated;
};

// Helper function to get activated proficiencies for attribute bolt
const getActivatedProficienciesForBolt = (
  boltAttr: AttributeType,
  playerAttributes: any
): string[] => {
  const activated: string[] = [];
  const attrLower = boltAttr.toLowerCase();

  // Fire Lv3 affects ALL attribute bolts (but don't add it twice for fire bolt)
  if (playerAttributes.fire >= 3 && attrLower !== "fire") {
    activated.push(
      `火 Lv3: ${
        attributeProficiency.fire.find((p) => p.level === 3)?.description
      }`
    );
  }

  // Check the specific bolt attribute's proficiencies
  const proficiencies = attributeProficiency[attrLower];
  if (proficiencies) {
    const playerLevel =
      playerAttributes[attrLower as keyof typeof playerAttributes];

    proficiencies.forEach((prof) => {
      // Skip passive skills: Wood Lv5 and Fire Lv5
      if (
        (attrLower === "wood" && prof.level === 5) ||
        (attrLower === "fire" && prof.level === 5)
      ) {
        return;
      }

      if (playerLevel >= prof.level) {
        activated.push(
          `${attributeNames[boltAttr]} Lv${prof.level}: ${prof.description}`
        );
      }
    });
  }

  return activated;
};

// Helper function to extract enchantments from cost string
const extractEnchantments = (cost: string): string[] => {
  const pattern = /([火木雷水風毒])\d+/g;
  const matches = Array.from(cost.matchAll(pattern));
  return matches
    .map((match) => {
      const attrChar = match[1];
      const attrType = attributeNamesReverse[attrChar];
      return attrType?.toLowerCase() || "";
    })
    .filter(Boolean);
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
  const previousGameInfoRef = useRef<GameInfo | null>(null);

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

  // Load test scenarios when on lobby screen (only in local environment)
  useEffect(() => {
    if (screen === "lobby" && isLocalEnvironment()) {
      loadTestScenarios();
    }
  }, [screen]);

  const loadRoomInfo = async () => {
    if (!roomCode) return;
    try {
      const info = await lobbyApi.getRoomInfo(roomCode);
      setRoomInfo(info);

      // If game has started, transition to game screen
      if (info.game_id) {
        setGameId(info.game_id);
        localStorage.setItem("mage_game_id", info.game_id);
        setGameLog([]); // Clear logs when game starts
        previousGameInfoRef.current = null; // Clear previous game state
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

      // Detect changes and generate log messages
      // Only process if there's been a meaningful state change
      const previousGameInfo = previousGameInfoRef.current;

      if (previousGameInfo) {
        // Check if any significant state has changed
        const hasStateChanged =
          previousGameInfo.turn_number !== info.turn_number ||
          previousGameInfo.turn_phase !== info.turn_phase ||
          previousGameInfo.current_player_index !== info.current_player_index ||
          // Also check if any player stats have changed
          previousGameInfo.players.some((prevPlayer, idx) => {
            const player = info.players[idx];
            return (
              prevPlayer.hp !== player.hp ||
              prevPlayer.shield !== player.shield ||
              Object.keys(prevPlayer.attributes).some(
                (attr) =>
                  prevPlayer.attributes[
                    attr as keyof typeof prevPlayer.attributes
                  ] !==
                  player.attributes[attr as keyof typeof player.attributes]
              )
            );
          });

        if (hasStateChanged) {
          const newLogs: string[] = [];

          // Process new actions from server action_log
          const prevActionLog = previousGameInfo.action_log || [];
          const currentActionLog = info.action_log || [];

          if (currentActionLog.length > prevActionLog.length) {
            const newActions = currentActionLog.slice(prevActionLog.length);
            newLogs.push(...newActions);
          }

          // Determine who acted (the player whose turn just finished)
          let actingPlayer: (typeof info.players)[0] | null = null;

          // If turn or phase changed, the previous current player acted
          if (
            previousGameInfo.current_player_index !==
              info.current_player_index ||
            previousGameInfo.turn_phase !== info.turn_phase
          ) {
            actingPlayer =
              previousGameInfo.players[previousGameInfo.current_player_index];
          }

          // Check for attribute changes and log allocations
          info.players.forEach((player, idx) => {
            const prevPlayer = previousGameInfo.players[idx];

            // Check for attribute changes
            const attrTypes: Array<keyof typeof player.attributes> = [
              "fire",
              "wood",
              "thunder",
              "water",
              "wind",
              "poison",
            ];
            attrTypes.forEach((attr) => {
              if (prevPlayer.attributes[attr] !== player.attributes[attr]) {
                const change =
                  player.attributes[attr] - prevPlayer.attributes[attr];
                if (change > 0) {
                  const attrName =
                    attributeNames[
                      (attr.charAt(0).toUpperCase() +
                        attr.slice(1)) as AttributeType
                    ];
                  newLogs.push(
                    `${player.name} 分配了 ${change} 點到 ${attrName}`
                  );
                }
              }
            });
          });

          // Check for HP/shield changes (damage/healing)
          info.players.forEach((player, idx) => {
            const prevPlayer = previousGameInfo.players[idx];

            // HP changes
            if (prevPlayer.hp !== player.hp) {
              const hpChange = player.hp - prevPlayer.hp;
              if (
                hpChange < 0 &&
                actingPlayer &&
                actingPlayer.id !== player.id
              ) {
                // Damage dealt by another player
                newLogs.push(
                  `${actingPlayer.name} 對 ${
                    player.name
                  } 造成 ${Math.abs(hpChange)} 點傷害 (${prevPlayer.hp} → ${player.hp} HP)`
                );
              } else if (hpChange < 0) {
                // Damage from unknown source
                newLogs.push(
                  `${player.name} 受到 ${Math.abs(hpChange)} 點傷害 (${
                    prevPlayer.hp
                  } → ${player.hp} HP)`
                );
              } else if (hpChange > 0) {
                // Healing
                newLogs.push(
                  `${player.name} 恢復了 ${hpChange} 點生命 (${prevPlayer.hp} → ${player.hp} HP)`
                );
              }
            }

            // Shield changes
            if (prevPlayer.shield !== player.shield) {
              const shieldChange = player.shield - prevPlayer.shield;
              if (shieldChange > 0) {
                newLogs.push(`${player.name} 獲得了 ${shieldChange} 點護盾`);
              } else if (shieldChange < 0) {
                newLogs.push(
                  `${player.name} 失去了 ${Math.abs(shieldChange)} 點護盾`
                );
              }
            }
          });

          // Check for turn/phase changes
          if (
            previousGameInfo.current_player_index !==
              info.current_player_index ||
            previousGameInfo.turn_phase !== info.turn_phase
          ) {
            const currentPlayer = info.players[info.current_player_index];
            if (info.turn_phase === "AllocateAttribute") {
              newLogs.push(`=== ${currentPlayer.name} 的回合 ===`);
            }
          }

          if (newLogs.length > 0) {
            setGameLog((prev) => [...prev, ...newLogs]);
          }

          // Update ref immediately (synchronous)
          previousGameInfoRef.current = info;
        }
      } else {
        // First time loading, add existing action_log if any
        const actionLog = info.action_log || [];
        if (actionLog.length > 0) {
          setGameLog((prev) => [...prev, ...actionLog]);
        }
        // Set the previous state
        previousGameInfoRef.current = info;
      }

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

      setGameLog([]); // Clear logs when starting a new game
      previousGameInfoRef.current = null; // Clear previous game state
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

  const loadTestScenarios = async () => {
    try {
      setLoadingTestScenarios(true);
      const scenarios = await testApi.listTestScenarios();
      setTestScenarios(scenarios);
      setError(null);
    } catch (err: any) {
      console.error("Failed to load test scenarios:", err);
      // Don't set error state for test scenarios, just log it
    } finally {
      setLoadingTestScenarios(false);
    }
  };

  const handleCreateTestGame = async (scenarioId: string) => {
    try {
      setLoading(true);
      const response = await testApi.createTestGame(scenarioId);

      setGameId(response.game_id);
      localStorage.setItem("mage_game_id", response.game_id);

      // Set mySlotId to 0 for test games (the test player is always slot 0)
      setMySlotId(0);
      localStorage.setItem("mage_slot_id", "0");

      setGameLog([]); // Clear logs when starting a test game
      previousGameInfoRef.current = null; // Clear previous game state
      setScreen("game");
      setError(null);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  // Game actions
  const addLog = (result: ActionResult) => {
    // Filter out specific message types
    const shouldSkipMessage = (msg: string) => {
      return (
        msg.includes("Drew card") ||
        msg.includes("抽卡") ||
        msg.includes("allocated") ||
        msg.includes("分配")
      );
    };

    // If there's a message, add it directly (unless filtered)
    if (result.message && !shouldSkipMessage(result.message)) {
      setGameLog((prev) => [...prev, result.message]);
    }

    // Also process events if available
    if (result.events && result.events.length > 0) {
      const newLogs: string[] = [];

      result.events.forEach((event) => {
        // Skip filtered event types
        if (
          event.event_type === "CardDrawn" ||
          !event.message ||
          shouldSkipMessage(event.message)
        )
          return;

        newLogs.push(event.message);
      });

      if (newLogs.length > 0) {
        setGameLog((prev) => [...prev, ...newLogs]);
      }
    }
  };

  const allocateAttr = async (attr: AttributeType) => {
    if (!gameId) return;
    try {
      setLoading(true);
      const result = await gameApi.allocateAttribute(gameId, attr);
      addLog(result);
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
  const [selectedBoltAttr, setSelectedBoltAttr] =
    useState<AttributeType | null>(null);

  // Card hover state for popup
  const [hoveredCard, setHoveredCard] = useState<number | null>(null);
  const [hoverTimeout, setHoverTimeout] = useState<NodeJS.Timeout | null>(null);

  // Attribute hover state for proficiency popup
  const [hoveredAttribute, setHoveredAttribute] = useState<{
    playerId: number;
    attr: string;
  } | null>(null);

  // Buff hover state for buff details popup
  const [hoveredBuff, setHoveredBuff] = useState<{
    playerId: number;
    buffIdx: number;
  } | null>(null);

  // Shield hover state
  const [hoveredShield, setHoveredShield] = useState<{
    playerId: number;
    location: string;
  } | null>(null);

  // Game log state
  const [gameLog, setGameLog] = useState<string[]>([]);

  // Guide state
  const [isGuideOpen, setIsGuideOpen] = useState(false);

  // Test mode state
  const [testScenarios, setTestScenarios] = useState<TestScenarioInfo[]>([]);
  const [loadingTestScenarios, setLoadingTestScenarios] = useState(false);

  // Check if running in local environment
  const isLocalEnvironment = () => {
    const apiUrl = process.env.REACT_APP_API_URL || 'http://localhost:3000';
    return apiUrl.includes('localhost') || apiUrl.includes('127.0.0.1');
  };

  const playBoltWithCard = async (
    cardId: number,
    attr: AttributeType,
    targets: number[]
  ) => {
    if (!gameId || !gameInfo || mySlotId === null) return;
    if (targets.length === 0) {
      setError('必須選擇至少一個目標');
      return;
    }
    try {
      setLoading(true);
      const result = await gameApi.playAttributeBolt(gameId, cardId, attr, targets);
      addLog(result);
      setSelectedCard(null);
      setCardAction(null);
      setSelectedBoltAttr(null);
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
    if (!gameId || !gameInfo || mySlotId === null) return;
    try {
      setLoading(true);
      const result = await gameApi.playSpellCard(gameId, cardId, side, targets);
      addLog(result);
      setSelectedCard(null);
      setCardAction(null);
      setSelectedBoltAttr(null);
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
      addLog(result);
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
      addLog(result);
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
            <div className="instructions-header">
              <h3>📖 遊戲說明</h3>
              <button
                onClick={() => setIsGuideOpen(true)}
                className="btn-guide-inline"
              >
                查看完整指南
              </button>
            </div>
            <ul>
              <li>創建房間或使用代碼加入房間</li>
              <li>每個瀏覽器控制一個角色</li>
              <li>4位玩家分成2隊（P1&P3 vs P2&P4）</li>
              <li>每回合：分配屬性點 → 出牌/使用解放 → 抽卡</li>
              <li>擊敗對方隊伍的兩位玩家獲勝</li>
            </ul>
          </div>

          {/* Guide Modal */}
          <Guide isOpen={isGuideOpen} onClose={() => setIsGuideOpen(false)} />

          {/* Test Scenarios Section - Only show in local environment */}
          {isLocalEnvironment() && (
            <div className="test-scenarios-section">
              <h3>🧪 測試場景</h3>
              <p className="test-description">
                快速開始預設好的測試場景，用於測試法術和增益效果
              </p>
              {loadingTestScenarios ? (
                <div className="test-loading">載入測試場景中...</div>
              ) : testScenarios.length > 0 ? (
                <div className="test-scenarios-grid">
                  {testScenarios.map((scenario) => (
                    <div key={scenario.scenario_id} className="test-scenario-card">
                      <div className="test-scenario-header">
                        <strong>{scenario.scenario_id}</strong>
                      </div>
                      <div className="test-scenario-description">
                        {scenario.description}
                      </div>
                      <button
                        onClick={() => handleCreateTestGame(scenario.scenario_id)}
                        disabled={loading}
                        className="btn-test"
                      >
                        {loading ? "創建中..." : "開始測試"}
                      </button>
                    </div>
                  ))}
                </div>
              ) : (
                <div className="test-empty">沒有可用的測試場景</div>
              )}
            </div>
          )}
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
            <span>階段: {gameInfo.turn_phase === "AllocateAttribute" ? "分配屬性" : gameInfo.turn_phase === "PlayCard" ? "出牌" : "抽牌"}</span>
            <span className={isMyTurn ? "current-turn" : ""}>
              當前玩家: {currentPlayer.name}
            </span>
            <span>🗑️ 棄牌: {myPlayer.discard_pile_count}張</span>
            <span>📚 牌庫: {gameInfo.deck_remaining}張</span>
          </div>
          <div className="header-actions">
            <button onClick={() => setIsGuideOpen(true)} className="btn-small btn-guide">
              📖 指南
            </button>
            <button onClick={resetAll} className="btn-small btn-warning">
              重置
            </button>
          </div>
        </header>

        {/* Guide Modal */}
        <Guide isOpen={isGuideOpen} onClose={() => setIsGuideOpen(false)} />

        {error && <div className="error">{error}</div>}

        {/* Turn Phase Hint - Top of Game */}
        {isMyTurn && (
          <div
            className={`turn-phase-hint-top turn-phase-${gameInfo.turn_phase
              .toLowerCase()
              .replace(/[^a-z]/g, "")}`}
          >
            你的回合:{" "}
            {gameInfo.turn_phase === "AllocateAttribute"
              ? "請分配屬性點"
              : gameInfo.turn_phase === "PlayCard"
              ? "請出牌"
              : "請抽牌"}
          </div>
        )}

        <div className="game-layout">
          {/* LEFT: Players List & Log */}
          <div className="players-list-panel">
            <h3>玩家列表</h3>
            <div className="players-list">
              {(() => {
                // Reorder players: self first, then next players in turn order
                const orderedPlayers = [];
                for (let i = 0; i < gameInfo.players.length; i++) {
                  const playerIndex = (mySlotId + i) % gameInfo.players.length;
                  orderedPlayers.push(gameInfo.players[playerIndex]);
                }
                return orderedPlayers;
              })().map((player) => {
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
                      isCurrentTurn ? "current-turn-player" : ""
                    } player-team-${player.team.toLowerCase()}`}
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
                          <span className="hp-text">❤️ {player.hp}</span>
                          {player.shield > 0 && (
                            <span
                              className="shield-text shield-tooltip"
                              onMouseEnter={() =>
                                setHoveredShield({
                                  playerId: player.id,
                                  location: "player-list",
                                })
                              }
                              onMouseLeave={() => setHoveredShield(null)}
                            >
                              | 🛡️ {player.shield}
                              {hoveredShield?.playerId === player.id &&
                                hoveredShield?.location === "player-list" && (
                                  <div className="shield-tooltip-popup">
                                    <strong>護盾</strong>
                                    <p>可格擋傷害。若單次傷害數值大於等於護盾值，則護盾被破壞，免除剩餘的傷害。</p>
                                  </div>
                                )}
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
                        const attrLevel =
                          player.attributes[
                            attr as keyof typeof player.attributes
                          ];
                        const proficiencies = attributeProficiency[attr];
                        const isHovered =
                          hoveredAttribute?.playerId === player.id &&
                          hoveredAttribute?.attr === attr;

                        return (
                          <div
                            key={attr}
                            className={`attr-inline ${
                              canAllocate ? "clickable" : ""
                            }`}
                            onClick={() => canAllocate && allocateAttr(attrCap)}
                            onMouseEnter={() =>
                              setHoveredAttribute({ playerId: player.id, attr })
                            }
                            onMouseLeave={() => setHoveredAttribute(null)}
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
                            <span>{attrLevel}</span>

                            {/* Proficiency Popup */}
                            {isHovered &&
                              proficiencies &&
                              player.id === mySlotId && (
                                <div className="attr-proficiency-popup">
                                  <div className="proficiency-title">
                                    {attributeNames[attrCap]}屬性專精
                                  </div>
                                  {proficiencies.map((prof, idx) => (
                                    <div
                                      key={idx}
                                      className={`proficiency-item ${
                                        attrLevel >= prof.level
                                          ? "active"
                                          : "inactive"
                                      }`}
                                    >
                                      <div className="proficiency-level">
                                        Lv{prof.level}:
                                      </div>
                                      <div className="proficiency-desc">
                                        {prof.description}
                                      </div>
                                    </div>
                                  ))}
                                </div>
                              )}
                          </div>
                        );
                      })}
                    </div>

                    {/* Buffs Display */}
                    {player.buffs && player.buffs.length > 0 && (
                      <div className="player-buffs-row">
                        {player.buffs.map((buff, idx) => {
                          const isDebuff = [
                            "Paralysis",
                            "Seal",
                            "Silent",
                            "MasterDisable",
                            "DefenseInvalidation",
                            "Confuse",
                            "HealthDrainTarget",
                          ].includes(buff.buff_type);

                          const buffEmojis: Record<string, string> = {
                            Immune: "🛡️",
                            Invincible: "✨",
                            Paralysis: "❄️",
                            Seal: "🔒",
                            Silent: "🤐",
                            MasterDisable: "🚫",
                            DefenseInvalidation: "💔",
                            Confuse: "😵",
                            HealthDrain: "🩸",
                            HealthDrainTarget: "🎯",
                            Regeneration: "💚",
                            BurningOut: "🔥",
                            GuardWoodCarving: "🗿",
                          };

                          const emoji = buffEmojis[buff.buff_type] || "⭐";
                          const isHovered = hoveredBuff?.playerId === player.id && hoveredBuff?.buffIdx === idx;
                          const buffDescription = buffDescriptions[buff.buff_type] || buff.name;

                          return (
                            <div
                              key={`${buff.buff_type}-${idx}`}
                              className={`buff-badge ${
                                isDebuff ? "debuff" : "buff"
                              }`}
                              onMouseEnter={() => setHoveredBuff({ playerId: player.id, buffIdx: idx })}
                              onMouseLeave={() => setHoveredBuff(null)}
                            >
                              <span className="buff-emoji">{emoji}</span>
                              <span className="buff-name">{buff.name}</span>
                              {buff.duration !== "永久" && (
                                <span className="buff-duration">
                                  {buff.duration}
                                </span>
                              )}

                              {/* Buff Details Popup */}
                              {isHovered && (
                                <div className="buff-detail-popup">
                                  <div className="buff-detail-title">{buff.name}</div>
                                  <div className="buff-detail-description">{buffDescription}</div>
                                  <div className="buff-detail-duration">持續時間: {buff.duration}</div>
                                  {buff.data !== null && buff.data !== undefined && (
                                    <div className="buff-detail-data">數值: {buff.data}</div>
                                  )}
                                </div>
                              )}
                            </div>
                          );
                        })}
                      </div>
                    )}

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
              <h3>🎴 我的手牌</h3>
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
                        } ${selectedCard === cardId ? "selected" : ""}`}
                        onClick={() => {
                          if (canClick) {
                            // Reset action states when choosing a different card
                            setCardAction(null);
                            setSelectedBoltAttr(null);
                            setSelectedCard(cardId);
                          }
                        }}
                        onMouseEnter={() => handleCardMouseEnter(cardId)}
                        onMouseLeave={handleCardMouseLeave}
                      >
                        {cardInfo ? (
                          <>
                            <div className="card-top-section">
                              <div className="card-spell-name">
                                {cardInfo.top_spell.name}
                              </div>
                              <div className={`card-cost ${
                                !checkSpellRequirement(cardInfo.top_spell.cost, myPlayer.attributes)
                                  ? "requirement-not-met"
                                  : ""
                              }`}>
                                {cardInfo.top_spell.cost}
                              </div>
                            </div>
                            {cardInfo.bottom_spell && (
                              <div className="card-bottom-section">
                                <div className="card-spell-name">
                                  {cardInfo.bottom_spell.name}
                                </div>
                                <div className={`card-cost ${
                                  !checkSpellRequirement(cardInfo.bottom_spell.cost, myPlayer.attributes)
                                    ? "requirement-not-met"
                                    : ""
                                }`}>
                                  {cardInfo.bottom_spell.cost}
                                </div>
                              </div>
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
                              <div>效果: {cardInfo.top_spell.description}</div>
                            </div>
                            {cardInfo.bottom_spell && (
                              <div className="popup-section">
                                <strong>
                                  下: {cardInfo.bottom_spell.name}
                                </strong>
                                <div>消耗: {cardInfo.bottom_spell.cost}</div>
                                <div>
                                  效果: {cardInfo.bottom_spell.description}
                                </div>
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
                      <div
                        className="shield-text shield-tooltip"
                        onMouseEnter={() =>
                          setHoveredShield({
                            playerId: player.id,
                            location: "game-board",
                          })
                        }
                        onMouseLeave={() => setHoveredShield(null)}
                      >
                        🛡️ {player.shield}
                        {hoveredShield?.playerId === player.id &&
                          hoveredShield?.location === "game-board" && (
                            <div className="shield-tooltip-popup">
                              <strong>護盾</strong>
                              <p>可格擋傷害。若單次傷害數值大於等於護盾值，則護盾被破壞，免除剩餘的傷害。</p>
                            </div>
                          )}
                      </div>
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
                                    {cardInfo.top_spell.description}
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
                                      {cardInfo.bottom_spell.description}
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
                                          onClick={() => {
                                            setSelectedBoltAttr(attr);
                                            setCardAction("bolt");
                                          }}
                                          disabled={loading || attrValue === 0}
                                          className="btn-bolt"
                                          title={
                                            attrValue === 0
                                              ? "此屬性等級為0"
                                              : `使用${attributeNames[attr]}屬性彈 (Lv${attrValue})`
                                          }
                                        >
                                          {attributeNames[attr]}彈
                                          <span className="bolt-level">
                                            Lv{attrValue}
                                          </span>
                                        </button>
                                      );
                                    })}
                                  </div>
                                </div>
                              </div>
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

                          // Find furthest enemy in turn order
                          const myTeam = myPlayer.team;
                          const enemies = gameInfo.players.filter(
                            (p) => p.team !== myTeam && !p.is_dead
                          );

                          // Calculate distance to each enemy going forward in turn order
                          let furthestEnemy = -1;
                          let maxDistance = -1;

                          enemies.forEach((enemy) => {
                            const distance = (enemy.id - mySlotId + 4) % 4;
                            if (distance > maxDistance) {
                              maxDistance = distance;
                              furthestEnemy = enemy.id;
                            }
                          });

                          const activatedProfs = getActivatedProficiencies(
                            spell.cost,
                            myPlayer.attributes
                          );

                          return (
                            <div className="target-selection-popup">
                              <h4>選擇目標 - {spell.name}</h4>
                              <div className="spell-info-compact">
                                <div>需求: {spell.cost}</div>
                                <div>效果: {spell.description}</div>
                              </div>

                              {activatedProfs.length > 0 && (
                                <div className="activated-proficiencies">
                                  <div className="prof-header">
                                    ✨ 啟動專精效果
                                  </div>
                                  {activatedProfs.map((prof, idx) => (
                                    <div key={idx} className="prof-item">
                                      {prof}
                                    </div>
                                  ))}
                                </div>
                              )}

                              <div className="target-buttons-horizontal">
                                {gameInfo.players.map((player) => {
                                  // For single target attack, only allow furthest enemy
                                  const isValidTarget =
                                    !player.is_dead &&
                                    player.id === furthestEnemy;

                                  // Check if Wind Lv2 proficiency will apply
                                  const spellEnchantments = extractEnchantments(
                                    spell.cost
                                  );
                                  const windLv2Applied =
                                    spellEnchantments.includes("Wind") &&
                                    myPlayer.attributes.wind >= 2;

                                  // Check if Poison Lv2 proficiency will apply
                                  const poisonLv2Applied =
                                    spellEnchantments.includes("Poison") &&
                                    myPlayer.attributes.poison >= 2;

                                  return (
                                    <button
                                      key={player.id}
                                      className={`target-button team-${player.team.toLowerCase()} ${
                                        isValidTarget ? "valid" : "disabled"
                                      }`}
                                      onClick={() => {
                                        if (isValidTarget && !loading) {
                                          playSpell(
                                            selectedCard,
                                            cardAction === "top"
                                              ? "Top"
                                              : "Bottom",
                                            [player.id]
                                          );
                                        }
                                      }}
                                      disabled={!isValidTarget || loading}
                                    >
                                      <div className="target-name">
                                        {player.name}
                                      </div>
                                      <div className="target-stats">
                                        ❤️ {player.hp}
                                        {player.shield > 0 && (
                                          <span
                                            className="shield-tooltip"
                                            onMouseEnter={() =>
                                              setHoveredShield({
                                                playerId: player.id,
                                                location: "target-spell",
                                              })
                                            }
                                            onMouseLeave={() => setHoveredShield(null)}
                                          >
                                            {` | 🛡️ ${player.shield}`}
                                            {hoveredShield?.playerId === player.id &&
                                              hoveredShield?.location === "target-spell" && (
                                                <div className="shield-tooltip-popup">
                                                  <strong>護盾</strong>
                                                  <p>可格擋傷害。若單次傷害數值大於等於護盾值，則護盾被破壞，免除剩餘的傷害。</p>
                                                </div>
                                              )}
                                          </span>
                                        )}
                                      </div>
                                      {windLv2Applied && (
                                        <div className="debuff-warning">
                                          🚫 無法回復/護盾
                                        </div>
                                      )}
                                      {poisonLv2Applied && (
                                        <div className="debuff-warning">
                                          ⏱️ 先出卡片
                                        </div>
                                      )}
                                    </button>
                                  );
                                })}
                              </div>
                            </div>
                          );
                        }

                        // If bolt action chosen, show target selection
                        if (cardAction === "bolt" && selectedBoltAttr) {
                          // Find furthest enemy in turn order
                          const myTeam = myPlayer.team;
                          const enemies = gameInfo.players.filter(
                            (p) => p.team !== myTeam && !p.is_dead
                          );

                          // Calculate distance to each enemy going forward in turn order
                          let furthestEnemy = -1;
                          let maxDistance = -1;

                          enemies.forEach((enemy) => {
                            const distance = (enemy.id - mySlotId + 4) % 4;
                            if (distance > maxDistance) {
                              maxDistance = distance;
                              furthestEnemy = enemy.id;
                            }
                          });

                          const activatedProfs =
                            getActivatedProficienciesForBolt(
                              selectedBoltAttr,
                              myPlayer.attributes
                            );

                          return (
                            <div className="target-selection-popup">
                              <h4>
                                選擇目標 - {attributeNames[selectedBoltAttr]}屬性彈
                              </h4>
                              <div className="spell-info-compact">
                                <div>
                                  屬性: {attributeNames[selectedBoltAttr]} Lv
                                  {
                                    myPlayer.attributes[
                                      selectedBoltAttr.toLowerCase() as keyof typeof myPlayer.attributes
                                    ]
                                  }
                                </div>
                                <div>基礎傷害: 等於屬性等級</div>
                              </div>

                              {activatedProfs.length > 0 && (
                                <div className="activated-proficiencies">
                                  <div className="prof-header">
                                    ✨ 啟動專精效果
                                  </div>
                                  {activatedProfs.map((prof, idx) => (
                                    <div key={idx} className="prof-item">
                                      {prof}
                                    </div>
                                  ))}
                                </div>
                              )}

                              <div className="target-buttons-horizontal">
                                {gameInfo.players.map((player) => {
                                  // For single target attack, only allow furthest enemy
                                  const isValidTarget =
                                    !player.is_dead &&
                                    player.id === furthestEnemy;

                                  // Check if Wind Lv2 proficiency will apply
                                  const windLv2Applied =
                                    selectedBoltAttr === "Wind" &&
                                    myPlayer.attributes.wind >= 2;

                                  return (
                                    <button
                                      key={player.id}
                                      className={`target-button team-${player.team.toLowerCase()} ${
                                        isValidTarget ? "valid" : "disabled"
                                      }`}
                                      onClick={() => {
                                        if (isValidTarget && !loading) {
                                          playBoltWithCard(
                                            selectedCard,
                                            selectedBoltAttr,
                                            [player.id]
                                          );
                                        }
                                      }}
                                      disabled={!isValidTarget || loading}
                                    >
                                      <div className="target-name">
                                        {player.name}
                                      </div>
                                      <div className="target-stats">
                                        ❤️ {player.hp}
                                        {player.shield > 0 && (
                                          <span
                                            className="shield-tooltip"
                                            onMouseEnter={() =>
                                              setHoveredShield({
                                                playerId: player.id,
                                                location: "target-bolt",
                                              })
                                            }
                                            onMouseLeave={() => setHoveredShield(null)}
                                          >
                                            {` | 🛡️ ${player.shield}`}
                                            {hoveredShield?.playerId === player.id &&
                                              hoveredShield?.location === "target-bolt" && (
                                                <div className="shield-tooltip-popup">
                                                  <strong>護盾</strong>
                                                  <p>可格擋傷害。若單次傷害數值大於等於護盾值，則護盾被破壞，免除剩餘的傷害。</p>
                                                </div>
                                              )}
                                          </span>
                                        )}
                                      </div>
                                      {windLv2Applied && (
                                        <div className="debuff-warning">
                                          🚫 無法回復/護盾
                                        </div>
                                      )}
                                    </button>
                                  );
                                })}
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
