/**
 * Router Provider
 * Wraps the app with atomic-router context
 */
import type { ReactNode } from 'react'

import { RouterProvider } from 'atomic-router-react'

import { router } from '@/shared/lib/router'

interface Props {
  children: ReactNode
}

export function AppRouterProvider({ children }: Props) {
  return <RouterProvider router={router}>{children}</RouterProvider>
}
