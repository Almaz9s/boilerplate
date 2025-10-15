/**
 * Login Feature
 * Handles user login flow
 */
import { createEffect, createEvent, createStore, sample } from 'effector'

import { authApi } from '@/shared/api'
import { routes } from '@/shared/lib/router'
import { tokenStorage } from '@/shared/lib/storage'
import type { ApiError, LoginRequest } from '@/shared/types'

import { userUpdated } from '@/entities/user/model'

// Events
export const emailChanged = createEvent<string>()
export const passwordChanged = createEvent<string>()
export const formSubmitted = createEvent<void>()
export const formReset = createEvent<void>()

// Effects
export const loginFx = createEffect(async (credentials: LoginRequest) => {
  const response = await authApi.login(credentials)
  // Store token
  tokenStorage.set(response.token)
  return response
})

// Stores
export const $email = createStore('')
  .on(emailChanged, (_, email) => email)
  .reset(formReset)

export const $password = createStore('')
  .on(passwordChanged, (_, password) => password)
  .reset(formReset)

export const $isLoading = createStore(false)
  .on(loginFx, () => true)
  .on(loginFx.finally, () => false)

export const $error = createStore<string | null>(null)
  .on(loginFx.failData, (_, error: ApiError) => error.message)
  .reset([loginFx, formReset, emailChanged, passwordChanged])

// Derived stores
export const $isFormValid = sample({
  source: { email: $email, password: $password },
  fn: ({ email, password }) => {
    const emailValid = /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)
    const passwordValid = password.length >= 8
    return emailValid && passwordValid
  },
})

export const $canSubmit = sample({
  source: { isValid: $isFormValid, isLoading: $isLoading },
  fn: ({ isValid, isLoading }) => isValid && !isLoading,
})

// Form submission flow
sample({
  clock: formSubmitted,
  source: { email: $email, password: $password },
  filter: $canSubmit,
  fn: ({ email, password }): LoginRequest => ({ email, password }),
  target: loginFx,
})

// Update user entity on successful login
sample({
  clock: loginFx.doneData,
  fn: (response) => response.user,
  target: userUpdated,
})

// Reset form on success
sample({
  clock: loginFx.done,
  target: formReset,
})

// Redirect to home after successful login
sample({
  clock: loginFx.done,
  fn: () => {},
  target: routes.home.open,
})
