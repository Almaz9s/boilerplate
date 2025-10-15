/**
 * Demo Page
 * Showcases Effector state management with shadcn/ui components
 */
import { useUnit } from 'effector-react'

import { Link } from 'atomic-router-react'

import { routes } from '@/shared/lib/router'
import {
  Badge,
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  Input,
  Label,
  Progress,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '@/shared/ui'

import { $counter, decrement, increment, reset } from '../model/counter'
import { $text, setText } from '../model/text'

export function DemoPage() {
  const counter = useUnit($counter)
  const text = useUnit($text)

  return (
    <div className="container mx-auto py-12 px-4">
      <div className="max-w-4xl mx-auto space-y-8">
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-4xl font-bold tracking-tight">Demo Page</h1>
            <p className="text-muted-foreground mt-2">
              Effector state management with shadcn/ui components
            </p>
          </div>
          <Link to={routes.home}>
            <Button variant="outline">Back to Home</Button>
          </Link>
        </div>

        <Tabs defaultValue="counter" className="w-full">
          <TabsList className="grid w-full grid-cols-2">
            <TabsTrigger value="counter">Counter Demo</TabsTrigger>
            <TabsTrigger value="form">Form Demo</TabsTrigger>
          </TabsList>

          <TabsContent value="counter" className="space-y-4">
            <Card>
              <CardHeader>
                <CardTitle>Counter Example</CardTitle>
                <CardDescription>
                  State managed by Effector with increment, decrement, and reset actions
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-6">
                <div className="flex items-center justify-center">
                  <Badge variant="secondary" className="text-6xl font-bold px-8 py-4">
                    {counter}
                  </Badge>
                </div>

                <Progress value={(counter % 100) + 50} className="w-full" />

                <div className="flex gap-4 justify-center">
                  <Button onClick={() => decrement()} variant="outline" size="lg">
                    Decrement
                  </Button>
                  <Button onClick={() => reset()} variant="secondary" size="lg">
                    Reset
                  </Button>
                  <Button onClick={() => increment()} size="lg">
                    Increment
                  </Button>
                </div>

                <div className="text-sm text-muted-foreground text-center">
                  The counter state is managed by Effector stores and events
                </div>
              </CardContent>
            </Card>
          </TabsContent>

          <TabsContent value="form" className="space-y-4">
            <Card>
              <CardHeader>
                <CardTitle>Form Example</CardTitle>
                <CardDescription>Text input synchronized with Effector state</CardDescription>
              </CardHeader>
              <CardContent className="space-y-6">
                <div className="space-y-2">
                  <Label htmlFor="text-input">Enter some text</Label>
                  <Input
                    id="text-input"
                    value={text}
                    onChange={(e) => setText(e.target.value)}
                    placeholder="Type something..."
                  />
                </div>

                {text && (
                  <Card className="bg-muted">
                    <CardContent className="pt-6">
                      <p className="text-sm font-medium">Current value:</p>
                      <p className="text-lg mt-2">{text}</p>
                    </CardContent>
                  </Card>
                )}

                <div className="text-sm text-muted-foreground">
                  The text is stored in an Effector store and updated via an event
                </div>
              </CardContent>
            </Card>
          </TabsContent>
        </Tabs>
      </div>
    </div>
  )
}
