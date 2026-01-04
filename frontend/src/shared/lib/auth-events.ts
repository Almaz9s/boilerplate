/**
 * Auth Events
 * Shared events for authentication state changes
 * Used to decouple API client from auth feature (avoid circular deps)
 */
import { createEvent } from 'effector'

// Fired when API returns 401 Unauthorized
// Signals that the current token is invalid/expired
export const unauthorizedReceived = createEvent()
