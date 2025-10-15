/**
 * Guest Route Component
 * Redirects authenticated users away from auth pages (login/register)
 */
import { useUnit } from 'effector-react'

import { Navigate } from 'react-router-dom'

import { $isAuthenticated } from '@/entities/user'

interface GuestRouteProps {
  children: React.ReactNode
  redirectTo?: string
}

export function GuestRoute({ children, redirectTo = '/' }: GuestRouteProps) {
  const isAuthenticated = useUnit($isAuthenticated)

  if (isAuthenticated) {
    return <Navigate to={redirectTo} replace />
  }

  return <>{children}</>
}
