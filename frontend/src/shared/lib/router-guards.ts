/**
 * Router Guards
 * Auth-based navigation protection using atomic-router
 *
 * Guards redirect:
 * - Unauthenticated users trying to access /home or /demo -> /login
 * - Authenticated users trying to access /login or /register -> /home
 */
import { chainRoute, redirect } from 'atomic-router'
import { $isAuthenticated } from '@/entities/user'
import { routes } from './router'

// Protect authenticated routes - redirect to login if not authenticated
chainRoute({
  route: routes.home,
  beforeOpen: {
    source: $isAuthenticated,
    filter: (isAuth) => !isAuth,
    target: redirect({ route: routes.login }),
  },
})

chainRoute({
  route: routes.demo,
  beforeOpen: {
    source: $isAuthenticated,
    filter: (isAuth) => !isAuth,
    target: redirect({ route: routes.login }),
  },
})

// Prevent authenticated users from accessing auth pages
chainRoute({
  route: routes.login,
  beforeOpen: {
    source: $isAuthenticated,
    filter: (isAuth) => isAuth,
    target: redirect({ route: routes.home }),
  },
})

chainRoute({
  route: routes.register,
  beforeOpen: {
    source: $isAuthenticated,
    filter: (isAuth) => isAuth,
    target: redirect({ route: routes.home }),
  },
})
