/**
 * Register Form Component
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
  $emailError,
  $error,
  $isLoading,
  $password,
  $passwordError,
  $username,
  $usernameError,
  emailChanged,
  formSubmitted,
  passwordChanged,
  usernameChanged,
} from '../model'

export function RegisterForm() {
  const {
    email,
    username,
    password,
    isLoading,
    error,
    canSubmit,
    emailError,
    usernameError,
    passwordError,
    onEmailChange,
    onUsernameChange,
    onPasswordChange,
    onSubmit,
  } = useUnit({
    email: $email,
    username: $username,
    password: $password,
    isLoading: $isLoading,
    error: $error,
    canSubmit: $canSubmit,
    emailError: $emailError,
    usernameError: $usernameError,
    passwordError: $passwordError,
    onEmailChange: emailChanged,
    onUsernameChange: usernameChanged,
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
        <CardTitle>Create Account</CardTitle>
        <CardDescription>Sign up to get started with your account</CardDescription>
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
            {emailError && <p className="text-sm text-red-600">{emailError}</p>}
          </div>

          <div className="space-y-2">
            <Label htmlFor="username">Username</Label>
            <Input
              id="username"
              type="text"
              placeholder="johndoe"
              value={username}
              onChange={(e) => onUsernameChange(e.target.value)}
              disabled={isLoading}
              required
              minLength={3}
              maxLength={100}
            />
            {usernameError && <p className="text-sm text-red-600">{usernameError}</p>}
            <p className="text-xs text-gray-500">Must be between 3 and 100 characters</p>
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
              minLength={8}
            />
            {passwordError && <p className="text-sm text-red-600">{passwordError}</p>}
            <p className="text-xs text-gray-500">Must be at least 8 characters</p>
          </div>
        </CardContent>

        <CardFooter>
          <Button type="submit" className="w-full" disabled={!canSubmit || isLoading}>
            {isLoading ? 'Creating account...' : 'Sign Up'}
          </Button>
        </CardFooter>
      </form>
    </Card>
  )
}
