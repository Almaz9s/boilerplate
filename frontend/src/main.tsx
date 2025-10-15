/**
 * Application entry point
 */
import { StrictMode } from 'react'

import { createRoot } from 'react-dom/client'

import { App } from './app'
import './index.css'
import { history, router } from './shared/lib/router'

// Initialize router with browser history
router.setHistory(history)

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
