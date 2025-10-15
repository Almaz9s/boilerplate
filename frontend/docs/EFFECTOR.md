# Effector State Management Guide

This guide covers Effector patterns and best practices used in this project.

## 📚 Core Concepts

### Stores ($)
Reactive state containers. Named with `$` prefix.

```typescript
import { createStore } from 'effector';

export const $counter = createStore(0);
export const $user = createStore<User | null>(null);
```

### Events
Trigger state changes. Named in present tense.

```typescript
import { createEvent } from 'effector';

export const increment = createEvent();
export const decrement = createEvent();
export const userChanged = createEvent<User>();
```

### Effects (Fx)
Async operations. Named with `Fx` suffix.

```typescript
import { createEffect } from 'effector';
import { apiClient } from '@/shared/api';

export const fetchUserFx = createEffect(async (id: string) => {
  return apiClient.get<User>(`/users/${id}`);
});
```

### Domains
Logical grouping of stores/events/effects.

```typescript
import { createDomain } from 'effector';

export const authDomain = createDomain('auth');

export const loginFx = authDomain.createEffect(/*...*/);
export const $isAuthenticated = authDomain.createStore(false);
```

## 🎯 Common Patterns

### Basic Store Updates

```typescript
import { createEvent, createStore } from 'effector';

// Simple increment
export const increment = createEvent();
export const $counter = createStore(0)
  .on(increment, (state) => state + 1);

// With payload
export const add = createEvent<number>();
export const $total = createStore(0)
  .on(add, (state, payload) => state + payload);

// Replace state
export const setUser = createEvent<User>();
export const $user = createStore<User | null>(null)
  .on(setUser, (_, user) => user);
```

### Reset Store

```typescript
import { createEvent, createStore } from 'effector';

export const reset = createEvent();
export const logout = createEvent();

export const $data = createStore<Data | null>(null)
  .reset(reset)
  .reset(logout); // Can have multiple reset triggers
```

### Effect Handling

```typescript
import { createEffect, createStore } from 'effector';

export const fetchUserFx = createEffect(async (id: string) => {
  const response = await fetch(`/api/users/${id}`);
  return response.json();
});

// Success case
export const $user = createStore<User | null>(null)
  .on(fetchUserFx.doneData, (_, user) => user);

// Loading state
export const $isLoading = createStore(false)
  .on(fetchUserFx, () => true)
  .on(fetchUserFx.finally, () => false);

// Error handling
export const $error = createStore<string | null>(null)
  .on(fetchUserFx.failData, (_, error) => error.message)
  .reset(fetchUserFx);
```

### Derived Stores

```typescript
import { createStore, combine } from 'effector';

export const $firstName = createStore('John');
export const $lastName = createStore('Doe');

// Combine stores
export const $fullName = combine(
  $firstName,
  $lastName,
  (first, last) => `${first} ${last}`
);

// Map store
export const $users = createStore<User[]>([]);
export const $userCount = $users.map((users) => users.length);
```

### Sample (Connect Events)

```typescript
import { createEvent, createEffect, sample } from 'effector';

export const formSubmitted = createEvent<FormData>();
export const saveDataFx = createEffect(async (data: FormData) => {
  return apiClient.post('/data', data);
});

// Trigger effect when event fires
sample({
  clock: formSubmitted,
  target: saveDataFx,
});

// With transformation
sample({
  clock: formSubmitted,
  fn: (formData) => ({ ...formData, timestamp: Date.now() }),
  target: saveDataFx,
});

// With source (combine data)
export const $userId = createStore('123');

sample({
  clock: formSubmitted,
  source: $userId,
  fn: (userId, formData) => ({ ...formData, userId }),
  target: saveDataFx,
});
```

### Guards (Conditional Logic)

```typescript
import { createEvent, createStore, sample } from 'effector';

export const buttonClicked = createEvent();
export const $isEnabled = createStore(true);

sample({
  clock: buttonClicked,
  source: $isEnabled,
  filter: (isEnabled) => isEnabled, // Only proceed if true
  target: doSomethingFx,
});

// Or use guard directly
import { guard } from 'effector';

guard({
  clock: buttonClicked,
  source: $isEnabled,
  filter: (isEnabled) => isEnabled,
  target: doSomethingFx,
});
```

## 🏗️ FSD Layer Patterns

### Entity Model

Business entity with CRUD operations:

```typescript
// entities/user/model/user.ts
import { createStore, createEffect, createEvent } from 'effector';
import { apiClient } from '@/shared/api';

export interface User {
  id: string;
  name: string;
  email: string;
}

// Effects
export const fetchUserFx = createEffect(async (id: string) => {
  return apiClient.get<User>(`/users/${id}`);
});

export const fetchUsersFx = createEffect(async () => {
  return apiClient.get<User[]>('/users');
});

// Events
export const userUpdated = createEvent<User>();

// Stores
export const $users = createStore<Record<string, User>>({})
  .on(fetchUserFx.doneData, (state, user) => ({
    ...state,
    [user.id]: user,
  }))
  .on(fetchUsersFx.doneData, (_, users) =>
    users.reduce((acc, user) => ({ ...acc, [user.id]: user }), {})
  )
  .on(userUpdated, (state, user) => ({
    ...state,
    [user.id]: user,
  }));

export const $currentUser = createStore<User | null>(null)
  .on(fetchUserFx.doneData, (_, user) => user);

// Derived stores
export const $usersList = $users.map((users) => Object.values(users));
export const $usersCount = $usersList.map((users) => users.length);
```

### Feature Model

User interaction with business logic:

```typescript
// features/edit-profile/model/edit-profile.ts
import { createStore, createEvent, createEffect, sample } from 'effector';
import { apiClient } from '@/shared/api';
import { $currentUser, userUpdated } from '@/entities/user';
import type { User } from '@/entities/user';

// Events
export const formChanged = createEvent<Partial<User>>();
export const formSubmitted = createEvent();
export const formReset = createEvent();

// Effects
export const updateProfileFx = createEffect(async (data: Partial<User>) => {
  return apiClient.patch<User>('/profile', data);
});

// Stores
export const $formData = createStore<Partial<User>>({})
  .on(formChanged, (state, data) => ({ ...state, ...data }))
  .reset(formReset)
  .reset(updateProfileFx.done);

export const $isSubmitting = createStore(false)
  .on(updateProfileFx, () => true)
  .on(updateProfileFx.finally, () => false);

export const $error = createStore<string | null>(null)
  .on(updateProfileFx.failData, (_, error) => error.message)
  .reset(updateProfileFx);

// Initialize form with current user
sample({
  clock: $currentUser,
  filter: Boolean,
  fn: (user) => ({ name: user.name, email: user.email }),
  target: $formData,
});

// Submit form
sample({
  clock: formSubmitted,
  source: $formData,
  target: updateProfileFx,
});

// Update entity store on success
sample({
  clock: updateProfileFx.doneData,
  target: userUpdated,
});
```

### Page Model

Page-level orchestration:

```typescript
// pages/profile/model/profile.ts
import { createEvent, sample } from 'effector';
import { routes } from '@/shared/lib/router';
import { fetchUserFx, $currentUser } from '@/entities/user';

// Page events
export const pageOpened = createEvent();

// Load user when page opens
sample({
  clock: routes.profile.opened,
  fn: () => 'current', // Get user ID from route params in real app
  target: fetchUserFx,
});
```

## 🔗 React Integration

### useUnit Hook

The primary hook for using Effector stores in React:

```tsx
import { useUnit } from 'effector-react';
import { $user, $isLoading, fetchUserFx, logout } from './model/user';

export function UserProfile() {
  // Single store
  const user = useUnit($user);

  // Multiple stores
  const { isLoading, user } = useUnit({
    isLoading: $isLoading,
    user: $user,
  });

  // Stores and events
  const { user, logout, fetchUser } = useUnit({
    user: $user,
    logout,
    fetchUser: fetchUserFx,
  });

  return (
    <div>
      {isLoading ? (
        <div>Loading...</div>
      ) : (
        <>
          <h1>{user?.name}</h1>
          <button onClick={logout}>Logout</button>
          <button onClick={() => fetchUser(user.id)}>Refresh</button>
        </>
      )}
    </div>
  );
}
```

### Form Example

```tsx
import { useUnit } from 'effector-react';
import { $formData, formChanged, formSubmitted } from './model/form';
import { Button, Input } from '@/shared/ui';

export function ProfileForm() {
  const { data, submit } = useUnit({
    data: $formData,
    submit: formSubmitted,
  });

  return (
    <form onSubmit={(e) => { e.preventDefault(); submit(); }}>
      <Input
        value={data.name || ''}
        onChange={(e) => formChanged({ name: e.target.value })}
      />
      <Input
        value={data.email || ''}
        onChange={(e) => formChanged({ email: e.target.value })}
      />
      <Button type="submit">Save</Button>
    </form>
  );
}
```

## 🎨 Advanced Patterns

### Async Store Pattern

Complete async operation state:

```typescript
import { createStore, createEffect } from 'effector';
import type { AsyncState } from '@/shared/types';

export const fetchDataFx = createEffect(async () => {
  return apiClient.get('/data');
});

export const $dataState = createStore<AsyncState<Data>>({
  data: null,
  error: null,
  status: 'idle',
})
  .on(fetchDataFx, (state) => ({
    ...state,
    status: 'pending',
  }))
  .on(fetchDataFx.doneData, (_, data) => ({
    data,
    error: null,
    status: 'success',
  }))
  .on(fetchDataFx.failData, (state, error) => ({
    ...state,
    error,
    status: 'error',
  }));
```

### Pagination Pattern

```typescript
import { createStore, createEvent, createEffect, sample } from 'effector';

export const pageChanged = createEvent<number>();
export const nextPage = createEvent();
export const prevPage = createEvent();

export const fetchPageFx = createEffect(
  async ({ page, limit }: { page: number; limit: number }) => {
    return apiClient.get('/items', { params: { page, limit } });
  }
);

export const $page = createStore(1)
  .on(pageChanged, (_, page) => page)
  .on(nextPage, (page) => page + 1)
  .on(prevPage, (page) => Math.max(1, page - 1));

export const $limit = createStore(10);

export const $items = createStore<Item[]>([])
  .on(fetchPageFx.doneData, (_, { items }) => items);

export const $total = createStore(0)
  .on(fetchPageFx.doneData, (_, { total }) => total);

// Auto-fetch on page change
sample({
  clock: $page,
  source: { page: $page, limit: $limit },
  target: fetchPageFx,
});
```

### Optimistic Updates

```typescript
import { createEffect, createStore, sample, restore } from 'effector';

export const addItemFx = createEffect(async (item: Item) => {
  return apiClient.post('/items', item);
});

export const $items = createStore<Item[]>([]);

// Optimistic add
$items.on(addItemFx, (items, newItem) => [
  ...items,
  { ...newItem, id: `temp-${Date.now()}`, pending: true },
]);

// Replace temp with real on success
$items.on(addItemFx.doneData, (items, realItem) =>
  items.map((item) =>
    item.pending ? realItem : item
  )
);

// Remove temp on failure
$items.on(addItemFx.fail, (items) =>
  items.filter((item) => !item.pending)
);
```

## 🧪 Testing

### Testing Stores

```typescript
import { allSettled, fork } from 'effector';
import { $counter, increment, decrement } from './counter';

test('counter increments', async () => {
  const scope = fork();

  await allSettled(increment, { scope });
  expect(scope.getState($counter)).toBe(1);

  await allSettled(increment, { scope });
  expect(scope.getState($counter)).toBe(2);
});
```

### Testing Effects

```typescript
import { allSettled, fork } from 'effector';
import { fetchUserFx, $user } from './user';

test('fetches user', async () => {
  const scope = fork({
    handlers: [
      [fetchUserFx, async () => ({ id: '1', name: 'John' })],
    ],
  });

  await allSettled(fetchUserFx, { scope, params: '1' });

  expect(scope.getState($user)).toEqual({ id: '1', name: 'John' });
});
```

## 📋 Best Practices

1. **Naming Conventions**
   - Stores: `$storeName`
   - Events: `verbName` (increment, changed, submitted)
   - Effects: `verbNameFx` (fetchUserFx, saveDataFx)
   - Domains: `domainName` (authDomain, userDomain)

2. **Store Organization**
   - Keep stores small and focused
   - Use `combine` for derived state
   - Reset stores when needed

3. **Effects**
   - One effect = one async operation
   - Handle all states: pending, success, error
   - Use `.fail` and `.done` for side effects

4. **Events**
   - Named in present tense
   - Carry meaningful payloads
   - Don't create unnecessary events

5. **Sample Usage**
   - Prefer `sample` over direct event subscription
   - Use for connecting stores and effects
   - Keep logic declarative

6. **TypeScript**
   - Always type stores: `createStore<Type>(initial)`
   - Type event payloads: `createEvent<PayloadType>()`
   - Infer effect types from function

7. **Performance**
   - Batch updates with `sample`
   - Avoid unnecessary derived stores
   - Use `map` for simple transformations

## 🔗 Resources

- [Effector Documentation](https://effector.dev/)
- [Effector React Hooks](https://effector.dev/en/api/effector-react/)
- [Best Practices](https://effector.dev/en/introduction/core-concepts/)
- [Atomic Router](https://github.com/atomic-router/atomic-router)
