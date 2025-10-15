/**
 * Login Form Component
 */
import { useUnit } from 'effector-react'

import {
  Button,
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
  Input,
  Label,
} from '@/shared/ui'

import {
  $canSubmit,
  $email,
  $error,
  $isLoading,
  $password,
  emailChanged,
  formSubmitted,
  passwordChanged,
} from '../model'

export function LoginForm() {
  const {
    email,
    password,
    isLoading,
    error,
    canSubmit,
    onEmailChange,
    onPasswordChange,
    onSubmit,
  } = useUnit({
    email: $email,
    password: $password,
    isLoading: $isLoading,
    error: $error,
    canSubmit: $canSubmit,
    onEmailChange: emailChanged,
    onPasswordChange: passwordChanged,
    onSubmit: formSubmitted,
  })

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    onSubmit()
  }

  return (
    <Card className="w-full max-w-md">
      <CardHeader>
        <CardTitle>Login</CardTitle>
        <CardDescription>Enter your credentials to access your account</CardDescription>
      </CardHeader>
      <form onSubmit={handleSubmit}>
        <CardContent className="space-y-4">
          {error && (
            <div className="p-3 text-sm text-red-600 bg-red-50 border border-red-200 rounded-md">
              {error}
            </div>
          )}

          <div className="space-y-2">
            <Label htmlFor="email">Email</Label>
            <Input
              id="email"
              type="email"
              placeholder="you@example.com"
              value={email}
              onChange={(e) => onEmailChange(e.target.value)}
              disabled={isLoading}
              required
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="password">Password</Label>
            <Input
              id="password"
              type="password"
              placeholder="••••••••"
              value={password}
              onChange={(e) => onPasswordChange(e.target.value)}
              disabled={isLoading}
              required
            />
          </div>
        </CardContent>

        <CardFooter>
          <Button type="submit" className="w-full" disabled={!canSubmit || isLoading}>
            {isLoading ? 'Logging in...' : 'Login'}
          </Button>
        </CardFooter>
      </form>
    </Card>
  )
}
