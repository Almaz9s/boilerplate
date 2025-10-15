/**
 * Text input model
 * Example Effector store for form state
 */
import { createEvent, createStore } from 'effector'

// Events
export const setText = createEvent<string>()
export const clearText = createEvent()

// Store
export const $text = createStore('')
  .on(setText, (_, text) => text)
  .reset(clearText)
