/**
 * Router Guards
 * Auth-based navigation protection using atomic-router
 *
 * Guards redirect:
 * - Unauthenticated users trying to access /home or /demo -> /login
 * - Authenticated users trying to access /login or /register -> /home
 */
import { redirect } from 'atomic-router'
import { sample } from 'effector'
import { $isAuthenticated } from '@/entities/user'
import { routes } from './router'

// Protect authenticated routes - redirect to login if not authenticated
redirect({
  clock: sample({
    clock: routes.home.opened,
    source: $isAuthenticated,
    filter: (isAuth) => !isAuth,
  }),
  route: routes.login,
})

redirect({
  clock: sample({
    clock: routes.demo.opened,
    source: $isAuthenticated,
    filter: (isAuth) => !isAuth,
  }),
  route: routes.login,
})

// Prevent authenticated users from accessing auth pages
redirect({
  clock: sample({
    clock: routes.login.opened,
    source: $isAuthenticated,
    filter: (isAuth) => isAuth,
  }),
  route: routes.home,
})

redirect({
  clock: sample({
    clock: routes.register.opened,
    source: $isAuthenticated,
    filter: (isAuth) => isAuth,
  }),
  route: routes.home,
})
