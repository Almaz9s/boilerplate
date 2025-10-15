import { createEffect } from 'effector'

import type { ExampleEntity } from '@/entities/example/model'

// Example async effect for API calls
export const fetchExamplesFx = createEffect<void, ExampleEntity[], Error>(async () => {
  // Replace with actual API call
  const response = await fetch('/api/examples')
  if (!response.ok) {
    throw new Error('Failed to fetch examples')
  }
  return response.json()
})

export const createExampleFx = createEffect<Omit<ExampleEntity, 'id'>, ExampleEntity, Error>(
  async (data) => {
    // Replace with actual API call
    const response = await fetch('/api/examples', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    })
    if (!response.ok) {
      throw new Error('Failed to create example')
    }
    return response.json()
  },
)
