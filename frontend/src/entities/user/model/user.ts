/**
 * User Entity
 * Core user business logic and state
 */
import { createEffect, createEvent, createStore } from 'effector'

import { authApi } from '@/shared/api'
import type { User } from '@/shared/types'

// Effects
export const fetchCurrentUserFx = createEffect(async () => {
  return authApi.me()
})

// Events
export const userUpdated = createEvent<User>()
export const userCleared = createEvent()

// Stores
export const $currentUser = createStore<User | null>(null)
  .on(fetchCurrentUserFx.doneData, (_, user) => user)
  .on(userUpdated, (_, user) => user)
  .reset(userCleared)

export const $isUserLoading = createStore(false)
  .on(fetchCurrentUserFx, () => true)
  .on(fetchCurrentUserFx.finally, () => false)

// Derived stores
export const $isAuthenticated = $currentUser.map((user) => user !== null)
export const $userEmail = $currentUser.map((user) => user?.email ?? null)
export const $username = $currentUser.map((user) => user?.username ?? null)
