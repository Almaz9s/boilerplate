/**
 * Validation utilities
 */

export const isEmail = (value: string): boolean => {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  return emailRegex.test(value)
}

export const isUrl = (value: string): boolean => {
  try {
    new URL(value)
    return true
  } catch {
    return false
  }
}

export const isEmpty = (value: unknown): boolean => {
  if (value === null || value === undefined) return true
  if (typeof value === 'string') return value.trim().length === 0
  if (Array.isArray(value)) return value.length === 0
  if (typeof value === 'object') return Object.keys(value).length === 0
  return false
}

export const isNotEmpty = (value: unknown): boolean => !isEmpty(value)

export const minLength = (value: string, length: number): boolean => {
  return value.length >= length
}

export const maxLength = (value: string, length: number): boolean => {
  return value.length <= length
}
