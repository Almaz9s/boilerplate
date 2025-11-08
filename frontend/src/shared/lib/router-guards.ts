/**
 * Router Guards
 * Auth-based navigation protection using atomic-router
 *
 * Guards redirect:
 * - Unauthenticated users trying to access /home or /demo -> /login
 * - Authenticated users trying to access /login or /register -> /home
 *
 * Important: Guards wait for session initialization to prevent race conditions
 * on page refresh where router might check auth before session is restored.
 */
import { redirect } from 'atomic-router'
import { sample } from 'effector'
import { $isAuthenticated } from '@/entities/user'
import { $sessionInitialized } from '@/features/auth/session'
import { routes } from './router'

// Protect authenticated routes - redirect to login if not authenticated
// Only redirect after session initialization to avoid race conditions
redirect({
  clock: sample({
    clock: routes.home.opened,
    source: { isAuth: $isAuthenticated, sessionInit: $sessionInitialized },
    filter: ({ isAuth, sessionInit }) => sessionInit && !isAuth,
  }),
  route: routes.login,
})

redirect({
  clock: sample({
    clock: routes.demo.opened,
    source: { isAuth: $isAuthenticated, sessionInit: $sessionInitialized },
    filter: ({ isAuth, sessionInit }) => sessionInit && !isAuth,
  }),
  route: routes.login,
})

// Prevent authenticated users from accessing auth pages
// Only redirect after session initialization to avoid race conditions
redirect({
  clock: sample({
    clock: routes.login.opened,
    source: { isAuth: $isAuthenticated, sessionInit: $sessionInitialized },
    filter: ({ isAuth, sessionInit }) => sessionInit && isAuth,
  }),
  route: routes.home,
})

redirect({
  clock: sample({
    clock: routes.register.opened,
    source: { isAuth: $isAuthenticated, sessionInit: $sessionInitialized },
    filter: ({ isAuth, sessionInit }) => sessionInit && isAuth,
  }),
  route: routes.home,
})
