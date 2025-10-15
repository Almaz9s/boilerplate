/**
 * Home Page
 */
import { Link } from 'atomic-router-react'

import { routes } from '@/shared/lib/router'
import { Button, Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/shared/ui'

export function HomePage() {
  return (
    <div className="container mx-auto py-12 px-4">
      <div className="max-w-4xl mx-auto space-y-8">
        <div className="text-center space-y-4">
          <h1 className="text-4xl font-bold tracking-tight">Frontend Boilerplate</h1>
          <p className="text-xl text-muted-foreground">
            A senior-level React, TypeScript, Effector, and Feature-Sliced Design boilerplate
          </p>
        </div>

        <div className="grid gap-6 md:grid-cols-2">
          <Card>
            <CardHeader>
              <CardTitle>Architecture</CardTitle>
              <CardDescription>Feature-Sliced Design</CardDescription>
            </CardHeader>
            <CardContent>
              <ul className="space-y-2 text-sm text-muted-foreground">
                <li>• Shared - Reusable utilities and UI components</li>
                <li>• Entities - Business entities</li>
                <li>• Features - User interactions and features</li>
                <li>• Widgets - Composite blocks</li>
                <li>• Pages - Application pages</li>
                <li>• App - Application initialization</li>
              </ul>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Tech Stack</CardTitle>
              <CardDescription>Modern tools and libraries</CardDescription>
            </CardHeader>
            <CardContent>
              <ul className="space-y-2 text-sm text-muted-foreground">
                <li>• React 19 + TypeScript</li>
                <li>• Effector for state management</li>
                <li>• Atomic Router for routing</li>
                <li>• Tailwind CSS + shadcn/ui</li>
                <li>• Vite for fast builds</li>
                <li>• ESLint + Prettier</li>
              </ul>
            </CardContent>
          </Card>
        </div>

        <div className="flex justify-center gap-4">
          <Link to={routes.demo}>
            <Button size="lg">View Demo</Button>
          </Link>
          <Button size="lg" variant="outline" asChild>
            <a href="https://feature-sliced.design" target="_blank" rel="noopener noreferrer">
              Learn FSD
            </a>
          </Button>
        </div>
      </div>
    </div>
  )
}
