/**
 * Auth Session Management
 * Handles session initialization and restoration
 */
import { createEffect, createEvent, createStore, sample } from 'effector'

import { appStarted } from '@/app/model'
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

// Clear user if session init fails (e.g., token invalid, network error)
sample({
  clock: initSessionFx.fail,
  target: userCleared,
})

// Navigation is handled by router guards - no manual redirects needed
// Router guards will redirect unauthenticated users to login automatically
