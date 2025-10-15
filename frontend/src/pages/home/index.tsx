import { useUnit } from 'effector-react'

import { Button } from '@/shared/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/shared/ui/card'

import { $currentUser } from '@/entities/user'

import { logoutFx } from '@/features/auth/logout'

import { ThemeToggle } from '@/widgets/theme-toggle'

export const HomePage = () => {
  const user = useUnit($currentUser)

  const handleLogout = () => {
    logoutFx()
  }

  // Auth checks handled by router guards - no need for manual redirects

  return (
    <div className="container mx-auto p-8">
      <div className="mb-4 flex items-center justify-between">
        <div>
          <h1 className="text-4xl font-bold">Welcome, {user?.username}!</h1>
          <p className="text-sm text-muted-foreground mt-1">{user?.email}</p>
        </div>
        <div className="flex items-center gap-2">
          <ThemeToggle />
          <Button variant="outline" onClick={handleLogout}>
            Logout
          </Button>
        </div>
      </div>

      <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
        <Card>
          <CardHeader>
            <CardTitle>React + TypeScript</CardTitle>
            <CardDescription>Modern React with full TypeScript support</CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">
              Built with Vite for fast development and optimized production builds.
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Effector</CardTitle>
            <CardDescription>Powerful state management</CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">
              Reactive state management with excellent TypeScript inference.
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Feature-Sliced Design</CardTitle>
            <CardDescription>Scalable architecture</CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">
              Organized by features for maintainability and scalability.
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Tailwind CSS</CardTitle>
            <CardDescription>Utility-first styling</CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">
              Fast styling with Tailwind's utility classes.
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>shadcn/ui</CardTitle>
            <CardDescription>Beautiful components</CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">
              All shadcn/ui components installed and ready to use.
            </p>
            <Button className="mt-4" variant="default">
              Example Button
            </Button>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>JWT Authentication</CardTitle>
            <CardDescription>Secure user authentication</CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">
              Protected routes with JWT token-based authentication from Rust backend.
            </p>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
