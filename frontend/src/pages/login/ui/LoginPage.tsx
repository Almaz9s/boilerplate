/**
 * Login Page
 */
import { Link } from 'atomic-router-react'

import { routes } from '@/shared/lib/router'

import { LoginForm } from '@/features/auth/login/ui'

export function LoginPage() {
  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50 px-4">
      <div className="w-full max-w-md space-y-4">
        <LoginForm />
        <p className="text-center text-sm text-gray-600">
          Don't have an account?{' '}
          <Link to={routes.register} className="text-blue-600 hover:underline">
            Sign up
          </Link>
        </p>
      </div>
    </div>
  )
}
