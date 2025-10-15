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

// Clear user entity on logout
sample({
  clock: logoutFx.done,
  target: userCleared,
})

// Redirect to login after logout
sample({
  clock: logoutFx.done,
  target: routes.login.open,
})
