/**
 * Router configuration using atomic-router
 */
import { createHistoryRouter, createRoute } from 'atomic-router'
import { createBrowserHistory } from 'history'

// Define routes
export const routes = {
  home: createRoute(),
  demo: createRoute(),
  login: createRoute(),
  register: createRoute(),
  notFound: createRoute(),
} as const

// Create browser history
export const history = createBrowserHistory()

// Create router instance
export const router = createHistoryRouter({
  routes: [
    { path: '/', route: routes.home },
    { path: '/demo', route: routes.demo },
    { path: '/login', route: routes.login },
    { path: '/register', route: routes.register },
  ],
  notFoundRoute: routes.notFound,
})
