const API_BASE = process.env.REACT_APP_API_URL + 'api';

export interface RoomDto {
  room_code: string;
  host_slot: number;
  state: string;
  player_slots: PlayerSlotDto[];
  game_id?: string;
  player_count: number;
  can_start: boolean;
}

export interface PlayerSlotDto {
  slot_id: number;
  player_name?: string;
  character?: string;
  character_title?: string;
  team?: string;  // 'A', 'B', 'C', or 'D'
  is_ready: boolean;
  is_occupied: boolean;
}

export interface CreateRoomResponse {
  room_code: string;
  slot_id: number;
  connection_id: string;
}

export interface JoinRoomResponse {
  room_code: string;
  slot_id: number;
  connection_id: string;
  room: RoomDto;
}

export interface StartGameResponse {
  game_id: string;
  your_slot_id: number;
}

class LobbyApi {
  private async request<T>(endpoint: string, options?: RequestInit): Promise<T> {
    const response = await fetch(`${API_BASE}${endpoint}`, {
      ...options,
      headers: {
        'Content-Type': 'application/json',
        ...options?.headers,
      },
    });

    const data = await response.json();

    if (!response.ok || !data.success) {
      throw new Error(data.error?.message || 'Request failed');
    }

    return data.data;
  }

  async createRoom(playerName: string): Promise<CreateRoomResponse> {
    return this.request<CreateRoomResponse>('/lobby/create', {
      method: 'POST',
      body: JSON.stringify({ player_name: playerName }),
    });
  }

  async joinRoom(roomCode: string, playerName: string): Promise<JoinRoomResponse> {
    return this.request<JoinRoomResponse>('/lobby/join', {
      method: 'POST',
      body: JSON.stringify({ room_code: roomCode, player_name: playerName }),
    });
  }

  async getRoomInfo(roomCode: string): Promise<RoomDto> {
    return this.request<RoomDto>(`/lobby/${roomCode}`, {
      method: 'GET',
    });
  }

  async selectCharacter(
    roomCode: string,
    connectionId: string,
    character: string
  ): Promise<RoomDto> {
    return this.request<RoomDto>(`/lobby/${roomCode}/character`, {
      method: 'POST',
      body: JSON.stringify({ connection_id: connectionId, character }),
    });
  }

  async selectTeam(
    roomCode: string,
    connectionId: string,
    team: string  // 'A', 'B', 'C', or 'D'
  ): Promise<RoomDto> {
    return this.request<RoomDto>(`/lobby/${roomCode}/team`, {
      method: 'POST',
      body: JSON.stringify({ connection_id: connectionId, team }),
    });
  }

  async setReady(
    roomCode: string,
    connectionId: string,
    ready: boolean
  ): Promise<RoomDto> {
    return this.request<RoomDto>(`/lobby/${roomCode}/ready`, {
      method: 'POST',
      body: JSON.stringify({ connection_id: connectionId, ready }),
    });
  }

  async startGame(roomCode: string, connectionId: string): Promise<StartGameResponse> {
    return this.request<StartGameResponse>(`/lobby/${roomCode}/start`, {
      method: 'POST',
      body: JSON.stringify({ connection_id: connectionId }),
    });
  }

  async leaveRoom(roomCode: string, connectionId: string): Promise<string> {
    return this.request<string>(`/lobby/${roomCode}/leave`, {
      method: 'POST',
      body: JSON.stringify({ connection_id: connectionId }),
    });
  }
}

export const lobbyApi = new LobbyApi();
