/**
 * Application initialization logic
 * Side effects and startup procedures
 */
import { sample } from 'effector'

import { appMounted, appStarted } from './index'

// Sample for app initialization side effects
sample({
  clock: appStarted,
  fn: () => {
    console.log('Application started')
    // Add any initialization logic here
    // e.g., load user session, initialize analytics, etc.
  },
})

sample({
  clock: appMounted,
  fn: () => {
    console.log('Application mounted')
    // Add any mount logic here
  },
})
