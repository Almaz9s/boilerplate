/**
 * Auth-related types
 * Using generated types from OpenAPI spec
 */
import type { components } from './api.generated'

export type User = components['schemas']['UserResponseDto']
export type RegisterRequest = components['schemas']['RegisterRequestDto']
export type LoginRequest = components['schemas']['LoginRequestDto']
export type AuthResponse = components['schemas']['AuthResponseDto']
