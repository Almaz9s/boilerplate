# Authentication Implementation

This document describes the authentication system implemented in the frontend application following Feature-Sliced Design (FSD) and Effector best practices.

## Architecture Overview

The auth system is organized according to FSD layers:

```
src/
├── shared/
│   ├── api/
│   │   └── auth.ts              # Auth API methods
│   ├── types/
│   │   └── auth.ts              # Auth type definitions
│   └── lib/
│       ├── storage.ts           # Token storage utilities
│       └── guards/              # Route guard components
│           ├── ProtectedRoute.tsx
│           └── GuestRoute.tsx
├── entities/
│   └── user/
│       └── model/
│           └── user.ts          # User entity (core domain)
├── features/
│   └── auth/
│       ├── login/               # Login feature
│       │   ├── model/
│       │   │   └── login.ts
│       │   └── ui/
│       │       └── LoginForm.tsx
│       ├── register/            # Register feature
│       │   ├── model/
│       │   │   └── register.ts
│       │   └── ui/
│       │       └── RegisterForm.tsx
│       ├── logout/              # Logout feature
│       │   └── model/
│       │       └── logout.ts
│       └── session/             # Session management
│           └── model/
│               └── session.ts
└── pages/
    ├── login/
    │   └── ui/
    │       └── LoginPage.tsx
    └── register/
        └── ui/
            └── RegisterPage.tsx
```

## Backend API Endpoints

The authentication system integrates with these backend endpoints:

- `POST /api/v1/auth/register` - Register new user
- `POST /api/v1/auth/login` - Login with credentials
- `GET /api/v1/auth/me` - Get current user (requires auth token)

## Layers Explanation

### 1. Shared Layer

#### Types (`shared/types/auth.ts`)
```typescript
interface User {
  id: string;
  email: string;
  username: string;
  created_at: string;
}

interface LoginRequest { email: string; password: string; }
interface RegisterRequest { email: string; username: string; password: string; }
interface AuthResponse { user: User; token: string; }
```

#### API Client (`shared/api/client.ts`)
- Automatically includes `Authorization: Bearer <token>` header
- Reads token from localStorage on every request
- Handles errors and timeouts

#### Token Storage (`shared/lib/storage.ts`)
```typescript
tokenStorage.get()      // Get auth token
tokenStorage.set(token) // Store auth token
tokenStorage.remove()   // Clear auth token
tokenStorage.exists()   // Check if token exists
```

#### Route Guards (`shared/lib/guards/`)

**ProtectedRoute** - Redirects to `/login` if not authenticated:
```tsx
<ProtectedRoute>
  <DashboardPage />
</ProtectedRoute>
```

**GuestRoute** - Redirects authenticated users away:
```tsx
<GuestRoute redirectTo="/dashboard">
  <LoginPage />
</GuestRoute>
```

### 2. Entities Layer

#### User Entity (`entities/user/model/user.ts`)

The core domain model for user data. Contains:

**Effects:**
- `fetchCurrentUserFx` - Fetch current user from API

**Events:**
- `userUpdated` - Manually update user
- `userCleared` - Clear user (logout)

**Stores:**
- `$currentUser` - Current user object or null
- `$isUserLoading` - Loading state
- `$isAuthenticated` - Derived: true if user exists
- `$userEmail` - Derived: user's email
- `$username` - Derived: user's username

### 3. Features Layer

#### Login Feature (`features/auth/login/`)

**Model (`model/login.ts`):**
- Events: `emailChanged`, `passwordChanged`, `formSubmitted`, `formReset`
- Effect: `loginFx` - Calls login API and stores token
- Stores: `$email`, `$password`, `$isLoading`, `$error`
- Derived: `$isFormValid`, `$canSubmit`
- Flow: On success → updates user entity → resets form

**UI (`ui/LoginForm.tsx`):**
- Controlled form using Effector stores
- Real-time validation feedback
- Error display

#### Register Feature (`features/auth/register/`)

**Model (`model/register.ts`):**
- Events: `emailChanged`, `usernameChanged`, `passwordChanged`, `formSubmitted`
- Effect: `registerFx` - Calls register API and stores token
- Stores: `$email`, `$username`, `$password`, `$isLoading`, `$error`
- Validation: `$emailError`, `$usernameError`, `$passwordError`
- Flow: On success → updates user entity → resets form

**UI (`ui/RegisterForm.tsx`):**
- Form with inline validation
- Shows validation requirements
- Disabled during submission

#### Logout Feature (`features/auth/logout/`)

**Model (`model/logout.ts`):**
- Event: `logoutTriggered`
- Effect: `logoutFx` - Removes token from storage
- Flow: On logout → clears user entity

Usage:
```tsx
import { useUnit } from 'effector-react';
import { logoutTriggered } from '@/features/auth/logout';

function LogoutButton() {
  const logout = useUnit(logoutTriggered);
  return <Button onClick={logout}>Logout</Button>;
}
```

#### Session Feature (`features/auth/session/`)

**Model (`model/session.ts`):**
- Event: `appStarted` - Trigger on app initialization
- Effect: `initSessionFx` - Restores session from token
- Flow:
  - Checks if token exists in storage
  - If yes → fetches current user
  - If no/fails → clears user entity

### 4. Pages Layer

#### Login Page (`pages/login/ui/LoginPage.tsx`)
- Renders `LoginForm`
- Link to register page
- Centered layout

#### Register Page (`pages/register/ui/RegisterPage.tsx`)
- Renders `RegisterForm`
- Link to login page
- Centered layout

## Usage Guide

### 1. Initialize Session on App Start

```tsx
// src/main.tsx or src/App.tsx
import { useEffect } from 'react';
import { appStarted } from '@/features/auth/session';

function App() {
  useEffect(() => {
    appStarted();
  }, []);

  return <Router>...</Router>;
}
```

### 2. Setup Routes with Guards

```tsx
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { ProtectedRoute, GuestRoute } from '@/shared/lib/guards';
import { LoginPage } from '@/pages/login';
import { RegisterPage } from '@/pages/register';
import { DashboardPage } from '@/pages/dashboard';

function AppRoutes() {
  return (
    <Routes>
      <Route
        path="/login"
        element={
          <GuestRoute>
            <LoginPage />
          </GuestRoute>
        }
      />
      <Route
        path="/register"
        element={
          <GuestRoute>
            <RegisterPage />
          </GuestRoute>
        }
      />
      <Route
        path="/dashboard"
        element={
          <ProtectedRoute>
            <DashboardPage />
          </ProtectedRoute>
        }
      />
    </Routes>
  );
}
```

### 3. Display User Information

```tsx
import { useUnit } from 'effector-react';
import { $currentUser, $isAuthenticated } from '@/entities/user';

function UserProfile() {
  const { user, isAuthenticated } = useUnit({
    user: $currentUser,
    isAuthenticated: $isAuthenticated,
  });

  if (!isAuthenticated) return null;

  return (
    <div>
      <p>Email: {user.email}</p>
      <p>Username: {user.username}</p>
    </div>
  );
}
```

### 4. Implement Logout

```tsx
import { useUnit } from 'effector-react';
import { logoutTriggered } from '@/features/auth/logout';
import { Button } from '@/shared/ui';

function LogoutButton() {
  const logout = useUnit(logoutTriggered);

  return (
    <Button onClick={logout}>
      Logout
    </Button>
  );
}
```

### 5. Conditional Rendering Based on Auth

```tsx
import { useUnit } from 'effector-react';
import { $isAuthenticated } from '@/entities/user';

function Header() {
  const isAuthenticated = useUnit($isAuthenticated);

  return (
    <header>
      {isAuthenticated ? (
        <>
          <DashboardLink />
          <LogoutButton />
        </>
      ) : (
        <>
          <Link to="/login">Login</Link>
          <Link to="/register">Sign Up</Link>
        </>
      )}
    </header>
  );
}
```

## Security Features

1. **JWT Token Storage**: Tokens stored in localStorage (consider httpOnly cookies for production)
2. **Automatic Token Injection**: API client automatically adds Authorization header
3. **Session Restoration**: App checks for existing token on startup
4. **Route Protection**: Protected routes redirect unauthenticated users
5. **Password Validation**: Minimum 8 characters enforced
6. **Email Validation**: RFC-compliant email validation

## Best Practices Followed

### Effector Patterns
- ✅ Stores prefixed with `$`
- ✅ Effects suffixed with `Fx`
- ✅ Events in present tense
- ✅ Use `sample` for connecting logic
- ✅ Reset stores on logout
- ✅ Derived stores for computed values
- ✅ Loading and error states

### Feature-Sliced Design
- ✅ Clear layer separation (shared → entities → features → pages)
- ✅ Public API via index files
- ✅ No cross-imports between features
- ✅ Entities before features
- ✅ Features are independent and composable

### TypeScript
- ✅ All interfaces properly typed
- ✅ API request/response types
- ✅ No `any` types
- ✅ Strict null checks

## Testing Recommendations

```typescript
// Test login flow
import { allSettled, fork } from 'effector';
import { loginFx, $isLoading, $error } from './login';

test('successful login', async () => {
  const scope = fork({
    handlers: [[loginFx, async () => ({ user: mockUser, token: 'token' })]],
  });

  await allSettled(loginFx, {
    scope,
    params: { email: 'test@example.com', password: 'password123' },
  });

  expect(scope.getState($isLoading)).toBe(false);
  expect(scope.getState($error)).toBeNull();
});
```

## Future Enhancements

- [ ] Add refresh token flow
- [ ] Implement "Remember me" functionality
- [ ] Add social login (OAuth)
- [ ] Email verification flow
- [ ] Password reset feature
- [ ] Two-factor authentication
- [ ] Session timeout warning
- [ ] Multiple device management

## Troubleshooting

**Issue**: User is not persisted after page refresh
- **Solution**: Ensure `appStarted()` is called on app initialization

**Issue**: API returns 401 Unauthorized
- **Solution**: Check if token is being stored and included in requests

**Issue**: Infinite redirect loop
- **Solution**: Verify route guards are not wrapping incompatible routes

**Issue**: Form validation not working
- **Solution**: Check that events are properly connected to stores
