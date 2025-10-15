/**
 * Shared types and interfaces
 * Common types used across the application
 */

export type Nullable<T> = T | null
export type Optional<T> = T | undefined
export type Maybe<T> = T | null | undefined

export * from './auth'

export interface BaseEntity {
  id: string | number
  createdAt?: string
  updatedAt?: string
}

export interface PaginationParams {
  page: number
  limit: number
}

export interface PaginatedResponse<T> {
  data: T[]
  total: number
  page: number
  limit: number
  totalPages: number
}

export interface ApiError {
  message: string
  code?: string
  status?: number
  details?: Record<string, unknown>
}

export type AsyncStatus = 'idle' | 'pending' | 'success' | 'error'

export interface AsyncState<T, E = ApiError> {
  data: Nullable<T>
  error: Nullable<E>
  status: AsyncStatus
}
