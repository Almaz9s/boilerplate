/**
 * Counter model
 * Example Effector store with events
 */
import { createEvent, createStore } from 'effector'

// Events
export const increment = createEvent()
export const decrement = createEvent()
export const reset = createEvent()

// Store
export const $counter = createStore(0)
  .on(increment, (state) => state + 1)
  .on(decrement, (state) => state - 1)
  .reset(reset)
