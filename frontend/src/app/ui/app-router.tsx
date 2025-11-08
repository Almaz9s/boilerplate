/**
 * App Router
 * Main routing component using atomic-router
 */
import { DemoPage, HomePage, LoginPage, NotFoundPage, RegisterPage } from '@/pages'
import { Route } from 'atomic-router-react'

import { routes } from '@/shared/lib/router'
import { AppLayout } from '@/widgets/app-layout'
// Import guards to ensure they're registered
import '@/shared/lib/router-guards'

// Pages that should be wrapped with the app layout (with sidebar)
function HomePageWithLayout() {
  return (
    <AppLayout>
      <HomePage />
    </AppLayout>
  )
}

function DemoPageWithLayout() {
  return (
    <AppLayout>
      <DemoPage />
    </AppLayout>
  )
}

function NotFoundPageWithLayout() {
  return (
    <AppLayout>
      <NotFoundPage />
    </AppLayout>
  )
}

export function AppRouter() {
  return (
    <>
      <Route route={routes.home} view={HomePageWithLayout} />
      <Route route={routes.demo} view={DemoPageWithLayout} />
      <Route route={routes.login} view={LoginPage} />
      <Route route={routes.register} view={RegisterPage} />
      <Route route={routes.notFound} view={NotFoundPageWithLayout} />
    </>
  )
}
