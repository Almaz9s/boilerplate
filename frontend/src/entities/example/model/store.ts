import { createEvent, createStore } from 'effector'

// Example entity store following Effector best practices
export interface ExampleEntity {
  id: string
  name: string
  createdAt: Date
}

// Events
export const exampleAdded = createEvent<ExampleEntity>()
export const exampleRemoved = createEvent<string>()
export const exampleUpdated = createEvent<{ id: string; name: string }>()

// Store
export const $examples = createStore<ExampleEntity[]>([])
  .on(exampleAdded, (state, example) => [...state, example])
  .on(exampleRemoved, (state, id) => state.filter((item) => item.id !== id))
  .on(exampleUpdated, (state, { id, name }) =>
    state.map((item) => (item.id === id ? { ...item, name } : item)),
  )
