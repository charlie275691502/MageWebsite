import axios from 'axios';

const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:3000';

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Types
export interface Player {
  id: number;
  name: string;
  character: {
    character_type: string;
    name: string;
    title: string;
    is_liberated: boolean;
    liberation_name: string;
  };
  team: string;
  hp: number;
  max_hp: number;
  shield: number;
  attributes: {
    fire: number;
    wood: number;
    thunder: number;
    water: number;
    wind: number;
    poison: number;
  };
  buffs: Array<{
    buff_type: string;
    name: string;
    duration: string;
    data: number | null;
  }>;
  hand: number[];
  hand_count: number;
  discard_pile_count: number;
  is_dead: boolean;
  death_turns: number;
  can_act: boolean;
  can_liberate: boolean;
}

export interface GameInfo {
  game_id: string;
  state: string;
  current_player_index: number;
  turn_number: number;
  turn_phase: string;
  players: Player[];
  deck_remaining: number;
}

export interface ActionResult {
  success: boolean;
  message: string;
  events: Array<{
    event_type: string;
    message: string;
    player_id?: number;
    value?: number;
  }>;
}

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: {
    code: string;
    message: string;
    details?: string;
  };
}

// API Functions
export const gameApi = {
  // Create a new game
  async createGame(playerNames: string[], characters: string[]): Promise<GameInfo> {
    const response = await api.post<ApiResponse<GameInfo>>('/api/game/new', {
      player_names: playerNames,
      characters: characters,
    });
    if (!response.data.success || !response.data.data) {
      throw new Error(response.data.error?.message || 'Failed to create game');
    }
    return response.data.data;
  },

  // Get game info
  async getGameInfo(gameId: string): Promise<GameInfo> {
    const response = await api.get<ApiResponse<GameInfo>>(`/api/game/${gameId}`);
    if (!response.data.success || !response.data.data) {
      throw new Error(response.data.error?.message || 'Failed to get game info');
    }
    return response.data.data;
  },

  // Get all players
  async getPlayers(gameId: string): Promise<Player[]> {
    const response = await api.get<ApiResponse<Player[]>>(`/api/game/${gameId}/players`);
    if (!response.data.success || !response.data.data) {
      throw new Error(response.data.error?.message || 'Failed to get players');
    }
    return response.data.data;
  },

  // Allocate attribute
  async allocateAttribute(gameId: string, attribute: string): Promise<ActionResult> {
    const response = await api.post<ApiResponse<ActionResult>>(
      `/api/game/${gameId}/allocate`,
      { attribute }
    );
    if (!response.data.success || !response.data.data) {
      throw new Error(response.data.error?.message || 'Failed to allocate attribute');
    }
    return response.data.data;
  },

  // Play attribute bolt (requires a card)
  async playAttributeBolt(gameId: string, cardId: number, attribute: string, targets: number[]): Promise<ActionResult> {
    const response = await api.post<ApiResponse<ActionResult>>(
      `/api/game/${gameId}/play_bolt`,
      { card_id: cardId, attribute, targets }
    );
    if (!response.data.success || !response.data.data) {
      throw new Error(response.data.error?.message || 'Failed to play attribute bolt');
    }
    return response.data.data;
  },

  // Play spell card (choose top or bottom spell)
  async playSpellCard(gameId: string, cardId: number, side: 'Top' | 'Bottom', targets: number[]): Promise<ActionResult> {
    const response = await api.post<ApiResponse<ActionResult>>(
      `/api/game/${gameId}/play_card`,
      { card_id: cardId, side, targets }
    );
    if (!response.data.success || !response.data.data) {
      throw new Error(response.data.error?.message || 'Failed to play spell card');
    }
    return response.data.data;
  },

  // Use liberation
  async useLiberationSkill(gameId: string, targets: number[]): Promise<ActionResult> {
    const response = await api.post<ApiResponse<ActionResult>>(
      `/api/game/${gameId}/liberate`,
      { targets }
    );
    if (!response.data.success || !response.data.data) {
      throw new Error(response.data.error?.message || 'Failed to use liberation');
    }
    return response.data.data;
  },

  // Draw card
  async drawCard(gameId: string): Promise<ActionResult> {
    const response = await api.post<ApiResponse<ActionResult>>(`/api/game/${gameId}/draw`);
    if (!response.data.success || !response.data.data) {
      throw new Error(response.data.error?.message || 'Failed to draw card');
    }
    return response.data.data;
  },
};

export default gameApi;
