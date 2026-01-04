/**
 * Auth Session Management
 * Handles session initialization and restoration
 */
import { createEffect, createEvent, createStore, sample } from 'effector'

import { appStarted } from '@/app/model'
import { unauthorizedReceived } from '@/shared/lib/auth-events'
import { routes } from '@/shared/lib/router'
import { tokenStorage } from '@/shared/lib/storage'

import { fetchCurrentUserFx, userCleared } from '@/entities/user'

// Events
export const sessionCheckRequested = createEvent()

// Effects
export const initSessionFx = createEffect(async () => {
  const token = tokenStorage.get()
  if (!token) {
    // No token, user is not authenticated - this is a valid state
    return null
  }
  // Token exists, fetch current user
  return await fetchCurrentUserFx()
})

// Effect to clear invalid token from storage
export const clearInvalidTokenFx = createEffect(() => {
  tokenStorage.remove()
})

// Stores
export const $sessionInitialized = createStore(false).on(initSessionFx.finally, () => true)

// Init session on app start
sample({
  clock: appStarted,
  target: initSessionFx,
})

// Also allow manual session check
sample({
  clock: sessionCheckRequested,
  target: initSessionFx,
})

// Clear user and token if session init fails (e.g., token expired/invalid)
sample({
  clock: initSessionFx.fail,
  target: [userCleared, clearInvalidTokenFx],
})

// Navigation is handled by router guards - no manual redirects needed
// Router guards will redirect unauthenticated users to login automatically

// Handle global 401 unauthorized responses (e.g., token expired during session)
// Clear token and user, then redirect to login
sample({
  clock: unauthorizedReceived,
  target: [clearInvalidTokenFx, userCleared],
})

sample({
  clock: unauthorizedReceived,
  target: routes.login.open,
})
