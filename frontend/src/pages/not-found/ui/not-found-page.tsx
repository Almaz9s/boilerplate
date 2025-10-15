/**
 * 404 Not Found Page
 */
import { Link } from 'atomic-router-react'

import { routes } from '@/shared/lib/router'
import { Button, Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/shared/ui'

export function NotFoundPage() {
  return (
    <div className="container mx-auto py-12 px-4">
      <div className="max-w-2xl mx-auto">
        <Card>
          <CardHeader className="text-center">
            <CardTitle className="text-6xl font-bold">404</CardTitle>
            <CardDescription className="text-xl">Page Not Found</CardDescription>
          </CardHeader>
          <CardContent className="text-center space-y-4">
            <p className="text-muted-foreground">
              The page you're looking for doesn't exist or has been moved.
            </p>
            <Link to={routes.home}>
              <Button size="lg">Go Home</Button>
            </Link>
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
