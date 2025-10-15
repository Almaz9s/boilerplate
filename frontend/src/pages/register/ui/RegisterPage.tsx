/**
 * Register Page
 */
import { Link } from 'atomic-router-react'

import { routes } from '@/shared/lib/router'

import { RegisterForm } from '@/features/auth/register/ui'

export function RegisterPage() {
  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50 px-4">
      <div className="w-full max-w-md space-y-4">
        <RegisterForm />
        <p className="text-center text-sm text-gray-600">
          Already have an account?{' '}
          <Link to={routes.login} className="text-blue-600 hover:underline">
            Login
          </Link>
        </p>
      </div>
    </div>
  )
}
