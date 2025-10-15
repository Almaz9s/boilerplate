/**
 * App Router
 * Main routing component using atomic-router
 */
import { DemoPage, HomePage, LoginPage, NotFoundPage, RegisterPage } from '@/pages'
import { Route } from 'atomic-router-react'

import { routes } from '@/shared/lib/router'
// Import guards to ensure they're registered
import '@/shared/lib/router-guards'

export function AppRouter() {
  return (
    <>
      <Route route={routes.home} view={HomePage} />
      <Route route={routes.demo} view={DemoPage} />
      <Route route={routes.login} view={LoginPage} />
      <Route route={routes.register} view={RegisterPage} />
      <Route route={routes.notFound} view={NotFoundPage} />
    </>
  )
}
