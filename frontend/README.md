# Frontend Boilerplate

A senior-level React + TypeScript boilerplate with Feature-Sliced Design architecture, Effector state management, Tailwind CSS, and shadcn/ui.

## 🚀 Tech Stack

- **React 19** - Latest React with concurrent features
- **TypeScript** - Type-safe development
- **Vite** - Lightning-fast build tool with HMR
- **Effector** - Reactive state management
- **Atomic Router** - Type-safe routing built on Effector
- **Feature-Sliced Design** - Scalable architectural methodology
- **Tailwind CSS** - Utility-first CSS framework
- **shadcn/ui** - High-quality, accessible React components
- **ESLint + Prettier** - Code quality and formatting
- **pnpm** - Fast, disk-efficient package manager

## 📁 Project Structure

This project follows **Feature-Sliced Design (FSD)** methodology for better scalability and maintainability:

```
src/
├── app/              # Application initialization layer
│   ├── model/        # Root Effector stores and events
│   ├── providers/    # App-level providers (theme, router, etc.)
│   └── ui/           # App-level UI components
│
├── pages/            # Page components (routes)
│   ├── home/
│   ├── demo/
│   └── not-found/
│
├── widgets/          # Large composite blocks
│   └── theme-toggle/
│
├── features/         # User interactions and features
│   └── example-feature/
│
├── entities/         # Business entities
│   └── example/
│
├── shared/           # Reusable resources
│   ├── api/          # API client and utilities
│   ├── config/       # Configuration and constants
│   ├── lib/          # Utility functions
│   ├── types/        # Shared TypeScript types
│   └── ui/           # UI components (shadcn/ui)
│
└── components/       # shadcn/ui components
    └── ui/
```

### FSD Layers (from top to bottom)

1. **app** - Application setup, providers, global styles
2. **pages** - Route pages, composed from lower layers
3. **widgets** - Large independent UI blocks
4. **features** - User scenarios, interactions
5. **entities** - Business entities
6. **shared** - Reusable infrastructure code

**Rule**: Higher layers can only import from lower layers, ensuring unidirectional data flow.

## 🏗️ Architecture Patterns

### State Management with Effector

Effector provides predictable state management with:

- **Stores** - Reactive state containers
- **Events** - State update triggers
- **Effects** - Async operations
- **Domains** - Logical grouping of stores/events

Example:

```typescript
// model/counter.ts
import { createEvent, createStore } from 'effector';

export const increment = createEvent();
export const decrement = createEvent();
export const reset = createEvent();

export const $counter = createStore(0)
  .on(increment, (state) => state + 1)
  .on(decrement, (state) => state - 1)
  .reset(reset);
```

### Routing with Atomic Router

Type-safe routing built on Effector:

```typescript
// shared/lib/router.ts
import { createRoute, createHistoryRouter } from 'atomic-router';

export const routes = {
  home: createRoute(),
  demo: createRoute(),
} as const;

export const router = createHistoryRouter({
  routes: [
    { path: '/', route: routes.home },
    { path: '/demo', route: routes.demo },
  ],
});
```

### Shared Layer Organization

The `shared` layer is organized into segments:

- **api** - HTTP client, request utilities
- **config** - Environment variables, app configuration
- **lib** - Pure utility functions (format, validation, storage)
- **types** - Common TypeScript types and interfaces
- **ui** - Reusable UI components (shadcn/ui)

## 🎨 Styling

### Tailwind CSS

Utility-first CSS with custom configuration for design system consistency.

### shadcn/ui

Pre-built, accessible components that can be customized:

```tsx
import { Button, Card, Input } from '@/shared/ui';

<Card>
  <Input placeholder="Enter text..." />
  <Button>Submit</Button>
</Card>
```

All components are included and can be customized in `src/components/ui/`.

## 🛠️ Development

### Getting Started

```bash
# Install dependencies
pnpm install

# Start development server
pnpm dev

# Build for production
pnpm build

# Preview production build
pnpm preview

# Lint code
pnpm lint
```

### Environment Variables

Create `.env` file in the root:

```env
VITE_API_BASE_URL=http://localhost:3000
VITE_APP_NAME=Frontend Boilerplate
VITE_APP_VERSION=1.0.0
```

### ESLint Configuration

The project uses a comprehensive ESLint setup with:

- TypeScript support with type imports
- React and React Hooks rules
- Import ordering based on FSD layers
- Prettier integration
- Custom rules for FSD architecture

Import order (enforced):

1. React and core libraries
2. External packages
3. Internal layers (app → pages → widgets → features → entities → shared)
4. Relative imports
5. Type imports

## 📝 Code Examples

### Creating a New Page

```bash
# Create directory structure
mkdir -p src/pages/profile/{ui,model,api}
```

```tsx
// src/pages/profile/ui/profile-page.tsx
import { useUnit } from 'effector-react';
import { $user } from '../model/user';
import { Card, CardContent, CardHeader, CardTitle } from '@/shared/ui';

export function ProfilePage() {
  const user = useUnit($user);

  return (
    <div className="container mx-auto py-8">
      <Card>
        <CardHeader>
          <CardTitle>Profile</CardTitle>
        </CardHeader>
        <CardContent>
          <p>{user?.name}</p>
        </CardContent>
      </Card>
    </div>
  );
}
```

```ts
// src/pages/profile/index.ts
export { ProfilePage } from './ui/profile-page';
```

### Adding a Route

```ts
// shared/lib/router.ts
export const routes = {
  home: createRoute(),
  profile: createRoute(), // Add new route
  notFound: createRoute(),
} as const;

export const router = createHistoryRouter({
  routes: [
    { path: '/', route: routes.home },
    { path: '/profile', route: routes.profile }, // Add path
    { path: '*', route: routes.notFound },
  ],
});
```

```tsx
// app/ui/app-router.tsx
import { ProfilePage } from '@/pages';

<Route route={routes.profile} view={ProfilePage} />
```

### Creating a Feature

```bash
mkdir -p src/features/auth/{ui,model,api}
```

```ts
// src/features/auth/model/auth.ts
import { createEvent, createStore, createEffect } from 'effector';
import { apiClient } from '@/shared/api';

export const loginFx = createEffect(async (credentials: { email: string; password: string }) => {
  return apiClient.post('/auth/login', credentials);
});

export const logout = createEvent();

export const $isAuthenticated = createStore(false)
  .on(loginFx.doneData, () => true)
  .reset(logout);
```

### Making API Calls

```ts
// shared/api/client.ts is pre-configured
import { apiClient } from '@/shared/api';

// GET request
const users = await apiClient.get('/users');

// POST request
const user = await apiClient.post('/users', { name: 'John' });

// With query params
const results = await apiClient.get('/search', {
  params: { q: 'query', page: 1 }
});
```

## 📚 Learn More

### Feature-Sliced Design

- [Official Documentation](https://feature-sliced.design/)
- [FSD Examples](https://github.com/feature-sliced/examples)

### Effector

- [Official Documentation](https://effector.dev/)
- [Effector React](https://effector.dev/en/api/effector-react/)

### Atomic Router

- [GitHub Repository](https://github.com/atomic-router/atomic-router)

### shadcn/ui

- [Component Documentation](https://ui.shadcn.com/)

## 🤝 Best Practices

1. **Follow FSD structure** - Keep each layer's responsibility clear
2. **Use TypeScript strictly** - Avoid `any`, prefer type inference
3. **Effector patterns** - Keep stores small and focused
4. **Component composition** - Build complex UIs from simple components
5. **Import order** - Let ESLint maintain consistent imports
6. **Public APIs** - Export from `index.ts` files only
7. **Shared utilities** - Keep `shared/lib` pure and framework-agnostic

## 📄 License

MIT
