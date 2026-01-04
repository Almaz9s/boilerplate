/**
 * Register Feature
 * Handles user registration flow
 */
import { createEffect, createEvent, createStore, sample } from 'effector'

import { authApi } from '@/shared/api'
import { isDevelopment } from '@/shared/config'
import { routes } from '@/shared/lib/router'
import { tokenStorage } from '@/shared/lib/storage'
import type { ApiError, RegisterRequest } from '@/shared/types'

import { userUpdated } from '@/entities/user'

// Events
export const emailChanged = createEvent<string>()
export const usernameChanged = createEvent<string>()
export const passwordChanged = createEvent<string>()
export const formSubmitted = createEvent<void>()
export const formReset = createEvent<void>()

// Effects
export const registerFx = createEffect(async (data: RegisterRequest) => {
  const response = await authApi.register(data)
  // Store token
  tokenStorage.set(response.token)
  return response
})

// Stores - Pre-filled with test credentials only in development
export const $email = createStore(isDevelopment ? 'newuser@example.com' : '')
  .on(emailChanged, (_, email) => email)
  .reset(formReset)

export const $username = createStore(isDevelopment ? 'newuser' : '')
  .on(usernameChanged, (_, username) => username)
  .reset(formReset)

export const $password = createStore(isDevelopment ? 'SecurePass123!' : '')
  .on(passwordChanged, (_, password) => password)
  .reset(formReset)

export const $isLoading = createStore(false)
  .on(registerFx, () => true)
  .on(registerFx.finally, () => false)

export const $error = createStore<string | null>(null)
  .on(registerFx.failData, (_, error: ApiError) => error.message)
  .reset([registerFx, formReset, emailChanged, usernameChanged, passwordChanged])

// Derived stores
export const $isFormValid = sample({
  source: { email: $email, username: $username, password: $password },
  fn: ({ email, username, password }) => {
    const emailValid = /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)
    const usernameValid = username.length >= 3 && username.length <= 100
    const passwordValid = password.length >= 8
    return emailValid && usernameValid && passwordValid
  },
})

export const $canSubmit = sample({
  source: { isValid: $isFormValid, isLoading: $isLoading },
  fn: ({ isValid, isLoading }) => isValid && !isLoading,
})

// Validation errors
export const $emailError = $email.map((email) => {
  if (!email) return null
  const valid = /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)
  return valid ? null : 'Invalid email address'
})

export const $usernameError = $username.map((username) => {
  if (!username) return null
  if (username.length < 3) return 'Username must be at least 3 characters'
  if (username.length > 100) return 'Username must be less than 100 characters'
  return null
})

export const $passwordError = $password.map((password) => {
  if (!password) return null
  if (password.length < 8) return 'Password must be at least 8 characters'
  return null
})

// Form submission flow
sample({
  clock: formSubmitted,
  source: { email: $email, username: $username, password: $password },
  filter: $canSubmit,
  fn: ({ email, username, password }): RegisterRequest => ({
    email,
    username,
    password,
  }),
  target: registerFx,
})

// Update user entity on successful registration
sample({
  clock: registerFx.doneData,
  fn: (response) => response.user,
  target: userUpdated,
})

// Redirect to home after successful registration
sample({
  clock: registerFx.done,
  target: routes.home.open,
})

// Reset form when navigating to register page (cleaner than resetting after success)
sample({
  clock: routes.register.opened,
  target: formReset,
})
