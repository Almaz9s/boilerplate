/**
 * Storage utilities for localStorage and sessionStorage
 */

type StorageType = 'local' | 'session'

class StorageService {
  private getStorage(type: StorageType): Storage {
    return type === 'local' ? localStorage : sessionStorage
  }

  set<T>(key: string, value: T, type: StorageType = 'local'): void {
    try {
      const storage = this.getStorage(type)
      storage.setItem(key, JSON.stringify(value))
    } catch (error) {
      console.error(`Error saving to ${type}Storage:`, error)
    }
  }

  get<T>(key: string, type: StorageType = 'local'): T | null {
    try {
      const storage = this.getStorage(type)
      const item = storage.getItem(key)
      return item ? JSON.parse(item) : null
    } catch (error) {
      console.error(`Error reading from ${type}Storage:`, error)
      return null
    }
  }

  remove(key: string, type: StorageType = 'local'): void {
    try {
      const storage = this.getStorage(type)
      storage.removeItem(key)
    } catch (error) {
      console.error(`Error removing from ${type}Storage:`, error)
    }
  }

  clear(type: StorageType = 'local'): void {
    try {
      const storage = this.getStorage(type)
      storage.clear()
    } catch (error) {
      console.error(`Error clearing ${type}Storage:`, error)
    }
  }

  has(key: string, type: StorageType = 'local'): boolean {
    try {
      const storage = this.getStorage(type)
      return storage.getItem(key) !== null
    } catch {
      return false
    }
  }
}

export const storage = new StorageService()

// Auth token storage
const TOKEN_KEY = 'auth_token'

export const tokenStorage = {
  get(): string | null {
    return storage.get<string>(TOKEN_KEY)
  },

  set(token: string): void {
    storage.set(TOKEN_KEY, token)
  },

  remove(): void {
    storage.remove(TOKEN_KEY)
  },

  exists(): boolean {
    return storage.has(TOKEN_KEY)
  },
}
