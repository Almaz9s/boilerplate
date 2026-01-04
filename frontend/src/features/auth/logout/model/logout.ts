/**
 * Logout Feature
 * Handles user logout flow
 */
import { createEffect, createEvent, sample } from 'effector'

import { routes } from '@/shared/lib/router'
import { tokenStorage } from '@/shared/lib/storage'

import { userCleared } from '@/entities/user'

// Events
export const logoutTriggered = createEvent<void>()

// Effects
export const logoutFx = createEffect(async () => {
  // Clear token from storage
  tokenStorage.remove()
  // Could call logout API endpoint here if needed
  // await authApi.logout();
})

// Logout flow
sample({
  clock: logoutTriggered,
  target: logoutFx,
})

// Clear user entity on logout (use .finally to ensure cleanup even on errors)
sample({
  clock: logoutFx.finally,
  target: userCleared,
})

// Redirect to login after logout (use .finally to ensure redirect even on errors)
sample({
  clock: logoutFx.finally,
  target: routes.login.open,
})
