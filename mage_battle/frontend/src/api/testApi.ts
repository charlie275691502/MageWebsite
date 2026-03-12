import axios from 'axios';

const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:3000';

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Types
export interface TestScenarioInfo {
  scenario_id: string;
  description: string;
}

export interface CreateTestGameResponse {
  game_id: string;
  scenario_id: string;
  scenario_description: string;
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
export const testApi = {
  // List all test scenarios
  async listTestScenarios(): Promise<TestScenarioInfo[]> {
    const response = await api.get<ApiResponse<TestScenarioInfo[]>>('/api/test/scenarios');
    if (!response.data.success || !response.data.data) {
      throw new Error(response.data.error?.message || 'Failed to list test scenarios');
    }
    return response.data.data;
  },

  // Create test game from scenario
  async createTestGame(scenarioId: string): Promise<CreateTestGameResponse> {
    const response = await api.post<ApiResponse<CreateTestGameResponse>>(
      `/api/test/create/${scenarioId}`
    );
    if (!response.data.success || !response.data.data) {
      throw new Error(response.data.error?.message || 'Failed to create test game');
    }
    return response.data.data;
  },
};

export default testApi;
