# Feature-Sliced Design Architecture

This document explains how Feature-Sliced Design (FSD) is implemented in this project.

## 📚 FSD Principles

Feature-Sliced Design is a frontend architecture methodology that organizes code by business domains and technical layers.

### Key Principles

1. **Layered Structure** - Code is organized in layers from abstract to specific
2. **Public API** - Each module exports a public API via `index.ts`
3. **Unidirectional Dependencies** - Higher layers can only import from lower layers
4. **Isolation** - Each slice is independent and encapsulated

## 🏗️ Layer Hierarchy

```
app/          ← Application layer (initialization, providers, global configs)
  ↓
pages/        ← Route pages (composed from lower layers)
  ↓
widgets/      ← Large composite UI blocks
  ↓
features/     ← User interactions and features
  ↓
entities/     ← Business entities
  ↓
shared/       ← Reusable infrastructure code
```

## 📁 Slice Structure

Each slice (page, feature, entity, widget) follows a standard structure:

```
slice-name/
├── ui/              # UI components
├── model/           # State management (Effector stores, events, effects)
├── api/             # API requests
├── lib/             # Utilities specific to this slice
├── config/          # Configuration
└── index.ts         # Public API
```

### Public API Pattern

Every slice must export its public API through `index.ts`:

```ts
// ✅ Good - Public API
// features/auth/index.ts
export { LoginForm } from './ui/login-form';
export { $isAuthenticated, login, logout } from './model/auth';

// ✅ Good - Usage
import { LoginForm, $isAuthenticated } from '@/features/auth';

// ❌ Bad - Direct import bypassing public API
import { LoginForm } from '@/features/auth/ui/login-form';
```

## 🎯 Layer Details

### App Layer (`src/app/`)

**Purpose**: Application initialization, providers, global configuration

**Contains**:
- Root component
- Providers (theme, router, error boundary)
- Global styles
- Application-level Effector stores

**Example**:
```tsx
// app/index.tsx
import { ThemeProvider } from './providers/theme-provider';
import { AppRouterProvider } from './providers/router-provider';
import { AppRouter } from './ui/app-router';

export const App = () => (
  <ThemeProvider>
    <AppRouterProvider>
      <AppRouter />
    </AppRouterProvider>
  </ThemeProvider>
);
```

### Pages Layer (`src/pages/`)

**Purpose**: Route pages composed from lower layers

**Contains**:
- Page components
- Page-specific models
- Route initialization logic

**Rules**:
- One page = one route
- Pages compose features, entities, widgets
- No business logic - delegate to features/entities

**Example**:
```tsx
// pages/profile/ui/profile-page.tsx
import { useUnit } from 'effector-react';
import { ProfileHeader } from '@/widgets/profile-header';
import { EditProfileForm } from '@/features/edit-profile';
import { $currentUser } from '@/entities/user';

export function ProfilePage() {
  const user = useUnit($currentUser);

  return (
    <div>
      <ProfileHeader user={user} />
      <EditProfileForm />
    </div>
  );
}
```

### Widgets Layer (`src/widgets/`)

**Purpose**: Large composite blocks combining multiple features/entities

**Contains**:
- Complex UI components
- Internal composition logic
- Widget-specific state

**Rules**:
- Can use features and entities
- Should be reusable across pages
- Contains composition, not business logic

**Example**:
```tsx
// widgets/profile-header/ui/profile-header.tsx
import { Avatar } from '@/shared/ui';
import { FollowButton } from '@/features/follow-user';
import type { User } from '@/entities/user';

export function ProfileHeader({ user }: { user: User }) {
  return (
    <header>
      <Avatar src={user.avatar} />
      <h1>{user.name}</h1>
      <FollowButton userId={user.id} />
    </header>
  );
}
```

### Features Layer (`src/features/`)

**Purpose**: User interactions, complete user scenarios

**Contains**:
- Interactive components (forms, buttons)
- Business logic
- Side effects
- Feature-specific state

**Rules**:
- Implements complete user actions
- Can use entities
- One feature = one user action/scenario

**Example**:
```tsx
// features/edit-profile/model/edit-profile.ts
import { createEvent, createStore, createEffect } from 'effector';
import { apiClient } from '@/shared/api';
import type { User } from '@/entities/user';

export const updateProfileFx = createEffect(
  async (data: Partial<User>) => {
    return apiClient.patch('/profile', data);
  }
);

export const formChanged = createEvent<Partial<User>>();

export const $formData = createStore<Partial<User>>({})
  .on(formChanged, (_, data) => data);
```

### Entities Layer (`src/entities/`)

**Purpose**: Business entities (domain models)

**Contains**:
- Entity types
- Entity stores
- Entity CRUD operations
- Entity UI components (cards, lists)

**Rules**:
- Pure business logic
- No feature-specific logic
- Reusable across features

**Example**:
```tsx
// entities/user/model/user.ts
import { createStore, createEffect } from 'effector';
import { apiClient } from '@/shared/api';

export interface User {
  id: string;
  name: string;
  email: string;
  avatar?: string;
}

export const fetchUserFx = createEffect(
  async (id: string) => {
    return apiClient.get<User>(`/users/${id}`);
  }
);

export const $users = createStore<Record<string, User>>({})
  .on(fetchUserFx.doneData, (state, user) => ({
    ...state,
    [user.id]: user,
  }));
```

### Shared Layer (`src/shared/`)

**Purpose**: Reusable infrastructure code

**Organized by segments**:
- `api/` - HTTP client, request utilities
- `config/` - Environment variables, constants
- `lib/` - Pure utility functions
- `types/` - Common TypeScript types
- `ui/` - UI component library (shadcn/ui)

**Rules**:
- No business logic
- Framework-agnostic utilities
- No imports from other layers

**Example**:
```ts
// shared/lib/format.ts
export const formatDate = (date: Date): string => {
  return new Intl.DateTimeFormat('en-US').format(date);
};

// shared/types/index.ts
export type Nullable<T> = T | null;
export type AsyncStatus = 'idle' | 'pending' | 'success' | 'error';
```

## 🔒 Import Rules

### Allowed Imports

```typescript
// ✅ Higher layer → Lower layer
// pages → widgets, features, entities, shared
import { ProfileHeader } from '@/widgets/profile-header';
import { EditProfile } from '@/features/edit-profile';
import { $user } from '@/entities/user';
import { Button } from '@/shared/ui';

// ✅ Same layer → Shared
import { apiClient } from '@/shared/api';
import { formatDate } from '@/shared/lib';

// ✅ Within same slice
import { ProfileForm } from './profile-form';
import { $profile } from '../model/profile';
```

### Forbidden Imports

```typescript
// ❌ Lower layer → Higher layer
// entities → features (NO!)
// shared → entities (NO!)

// ❌ Same layer cross-slice
// features/auth → features/profile (NO!)

// ❌ Bypassing public API
import { LoginForm } from '@/features/auth/ui/login-form'; // NO!
import { LoginForm } from '@/features/auth'; // YES!
```

## 🧩 Effector Integration

Effector stores fit naturally into FSD layers:

### Feature Model
```ts
// features/authentication/model/auth.ts
import { createEvent, createStore, createEffect } from 'effector';
import { apiClient } from '@/shared/api';

export const loginFx = createEffect(
  async (credentials: { email: string; password: string }) => {
    return apiClient.post('/auth/login', credentials);
  }
);

export const logout = createEvent();

export const $isAuthenticated = createStore(false)
  .on(loginFx.doneData, () => true)
  .reset(logout);

// features/authentication/index.ts
export { $isAuthenticated, loginFx, logout } from './model/auth';
export { LoginForm } from './ui/login-form';
```

### Entity Model
```ts
// entities/user/model/user.ts
import { createStore, createEffect } from 'effector';
import { apiClient } from '@/shared/api';

export const fetchUserFx = createEffect(async (id: string) => {
  return apiClient.get(`/users/${id}`);
});

export const $currentUser = createStore(null)
  .on(fetchUserFx.doneData, (_, user) => user);

// entities/user/index.ts
export { $currentUser, fetchUserFx } from './model/user';
export type { User } from './model/user';
```

## 🎨 Component Composition

### Page Composition
```tsx
// pages/dashboard/ui/dashboard-page.tsx
import { Sidebar } from '@/widgets/sidebar';
import { StatsCards } from '@/widgets/stats-cards';
import { ActivityFeed } from '@/features/activity-feed';
import { UserList } from '@/entities/user';

export function DashboardPage() {
  return (
    <div className="flex">
      <Sidebar />
      <main>
        <StatsCards />
        <ActivityFeed />
        <UserList />
      </main>
    </div>
  );
}
```

## 📋 Checklist for New Features

When adding a new feature:

1. [ ] Identify the correct layer (feature/entity/widget)
2. [ ] Create slice directory with segments (ui/, model/, api/)
3. [ ] Define public API in `index.ts`
4. [ ] Implement Effector stores/events in `model/`
5. [ ] Create UI components in `ui/`
6. [ ] Add API calls in `api/` (if needed)
7. [ ] Ensure imports follow FSD rules
8. [ ] Export only necessary items in public API

## 🔗 Resources

- [Feature-Sliced Design Official Docs](https://feature-sliced.design/)
- [FSD Examples](https://github.com/feature-sliced/examples)
- [Effector Documentation](https://effector.dev/)
- [Atomic Router](https://github.com/atomic-router/atomic-router)

## 🤝 Best Practices

1. **Keep slices small** - One feature = one user action
2. **Use public APIs** - Never bypass `index.ts`
3. **Follow import rules** - Use ESLint to enforce
4. **Separate concerns** - Logic in model/, UI in ui/
5. **Type everything** - Leverage TypeScript fully
6. **Document exports** - Comment public APIs
7. **Test boundaries** - Ensure layers respect dependencies
