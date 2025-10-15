/**
 * Shared configuration module
 * Contains app-wide configuration and environment variables
 */

export const config = {
  api: {
    baseUrl: import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080',
    timeout: 30000,
  },
  app: {
    name: import.meta.env.VITE_APP_NAME || 'Frontend Boilerplate',
    version: import.meta.env.VITE_APP_VERSION || '0.0.0',
    environment: import.meta.env.MODE || 'development',
  },
} as const

export const isDevelopment = config.app.environment === 'development'
export const isProduction = config.app.environment === 'production'

// Legacy export for backward compatibility
export const APP_CONFIG = {
  apiUrl: config.api.baseUrl,
  environment: config.app.environment,
  isDevelopment,
  isProduction,
}
