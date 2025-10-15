/**
 * App Component
 * Root application component with providers
 */
import { useEffect } from 'react'

import { useUnit } from 'effector-react'

import { $sessionInitialized } from '@/features/auth/session'

import { appStarted } from './model'
import ErrorBoundary from './providers/error-boundary'
import { AppRouterProvider } from './providers/router-provider'
import { ThemeProvider } from './providers/theme-provider'
import { AppRouter } from './ui/app-router'

export const App = () => {
  const sessionInitialized = useUnit($sessionInitialized)

  // Initialize session on app start
  useEffect(() => {
    appStarted()
  }, [])

  // Wait for session check to complete before rendering router
  if (!sessionInitialized) {
    return (
      <ErrorBoundary>
        <ThemeProvider defaultTheme="system" storageKey="app-theme">
          <div className="flex items-center justify-center min-h-screen">
            <div className="text-center">
              <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-gray-900 dark:border-gray-100 mx-auto"></div>
              <p className="mt-4 text-sm text-gray-600 dark:text-gray-400">Loading...</p>
            </div>
          </div>
        </ThemeProvider>
      </ErrorBoundary>
    )
  }

  return (
    <ErrorBoundary>
      <ThemeProvider defaultTheme="system" storageKey="app-theme">
        <AppRouterProvider>
          <AppRouter />
        </AppRouterProvider>
      </ThemeProvider>
    </ErrorBoundary>
  )
}
