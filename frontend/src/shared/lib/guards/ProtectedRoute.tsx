/**
 * Protected Route Component
 * Redirects to login if user is not authenticated
 */
import { useUnit } from 'effector-react'

import { Navigate } from 'react-router-dom'

import { $isAuthenticated } from '@/entities/user'

interface ProtectedRouteProps {
  children: React.ReactNode
}

export function ProtectedRoute({ children }: ProtectedRouteProps) {
  const isAuthenticated = useUnit($isAuthenticated)

  if (!isAuthenticated) {
    return <Navigate to="/login" replace />
  }

  return <>{children}</>
}
