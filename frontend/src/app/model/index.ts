/**
 * App-level Effector stores and events
 * Root level state management
 */
import { createDomain } from 'effector'

// Create app domain for organized store management
export const appDomain = createDomain('app')

// App initialization events
export const appStarted = appDomain.createEvent()
export const appMounted = appDomain.createEvent()
