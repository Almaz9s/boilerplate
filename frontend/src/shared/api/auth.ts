/**
 * Auth API
 * API methods for authentication
 */
import type { AuthResponse, LoginRequest, RegisterRequest, User } from '../types'
import { apiClient } from './client'

const AUTH_BASE = '/api/v1/auth'

export const authApi = {
  register(data: RegisterRequest): Promise<AuthResponse> {
    return apiClient.post<AuthResponse>(`${AUTH_BASE}/register`, data)
  },

  login(data: LoginRequest): Promise<AuthResponse> {
    return apiClient.post<AuthResponse>(`${AUTH_BASE}/login`, data)
  },

  me(): Promise<User> {
    return apiClient.get<User>(`${AUTH_BASE}/me`)
  },
}
