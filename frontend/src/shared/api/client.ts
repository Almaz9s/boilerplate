/**
 * API Client
 * Centralized HTTP client with interceptors and error handling
 */
import { config } from '../config'
import { tokenStorage } from '../lib/storage'
import type { ApiError } from '../types'

export interface RequestConfig extends RequestInit {
  params?: Record<string, string | number | boolean>
  timeout?: number
}

class ApiClient {
  private baseUrl: string
  private defaultTimeout: number

  constructor(baseUrl: string, timeout = 30000) {
    this.baseUrl = baseUrl
    this.defaultTimeout = timeout
  }

  private buildUrl(path: string, params?: Record<string, string | number | boolean>): string {
    const url = new URL(path, this.baseUrl)

    if (params) {
      Object.entries(params).forEach(([key, value]) => {
        url.searchParams.append(key, String(value))
      })
    }

    return url.toString()
  }

  private async handleResponse<T>(response: Response): Promise<T> {
    if (!response.ok) {
      const error: ApiError = {
        message: response.statusText,
        status: response.status,
      }

      try {
        const errorData = await response.json()
        error.message = errorData.message || error.message
        error.details = errorData
      } catch {
        // If response is not JSON, use statusText
      }

      throw error
    }

    // Handle empty responses
    const contentType = response.headers.get('content-type')
    if (!contentType || !contentType.includes('application/json')) {
      return undefined as T
    }

    return response.json()
  }

  async request<T>(path: string, config: RequestConfig = {}): Promise<T> {
    const { params, timeout = this.defaultTimeout, ...init } = config
    const url = this.buildUrl(path, params)

    const controller = new AbortController()
    const timeoutId = setTimeout(() => controller.abort(), timeout)

    // Get auth token from storage
    const token = this.getAuthToken()

    try {
      const headers: HeadersInit = {
        'Content-Type': 'application/json',
        ...(token ? { Authorization: `Bearer ${token}` } : {}),
        ...(init.headers as HeadersInit),
      }

      const response = await fetch(url, {
        ...init,
        signal: controller.signal,
        headers,
      })

      return await this.handleResponse<T>(response)
    } catch (error) {
      if (error instanceof Error && error.name === 'AbortError') {
        throw {
          message: 'Request timeout',
          code: 'TIMEOUT',
        } as ApiError
      }
      throw error
    } finally {
      clearTimeout(timeoutId)
    }
  }

  private getAuthToken(): string | null {
    return tokenStorage.get()
  }

  get<T>(path: string, config?: RequestConfig): Promise<T> {
    return this.request<T>(path, { ...config, method: 'GET' })
  }

  post<T>(path: string, data?: unknown, config?: RequestConfig): Promise<T> {
    return this.request<T>(path, {
      ...config,
      method: 'POST',
      body: JSON.stringify(data),
    })
  }

  put<T>(path: string, data?: unknown, config?: RequestConfig): Promise<T> {
    return this.request<T>(path, {
      ...config,
      method: 'PUT',
      body: JSON.stringify(data),
    })
  }

  patch<T>(path: string, data?: unknown, config?: RequestConfig): Promise<T> {
    return this.request<T>(path, {
      ...config,
      method: 'PATCH',
      body: JSON.stringify(data),
    })
  }

  delete<T>(path: string, config?: RequestConfig): Promise<T> {
    return this.request<T>(path, { ...config, method: 'DELETE' })
  }
}

export const apiClient = new ApiClient(config.api.baseUrl, config.api.timeout)
