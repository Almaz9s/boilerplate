import { Home, LayoutDashboard, LogOut, Moon, Sun, User } from 'lucide-react'
import { Link } from 'atomic-router-react'
import { useUnit } from 'effector-react'

import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarInset,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SidebarRail,
  SidebarSeparator,
  SidebarTrigger,
} from '@/components/ui/sidebar'
import { routes } from '@/shared/lib/router'
import { $currentUser } from '@/entities/user/model/user'
import { logoutTriggered } from '@/features/auth/logout/model/logout'
import { ThemeToggle } from '@/widgets/theme-toggle'
import { useTheme } from '@/app/providers/theme-provider'

const navigationItems = [
  {
    title: 'Home',
    url: '/',
    icon: Home,
    route: routes.home,
  },
  {
    title: 'Demo',
    url: '/demo',
    icon: LayoutDashboard,
    route: routes.demo,
  },
]

export function AppLayout({ children }: { children: React.ReactNode }) {
  const user = useUnit($currentUser)
  const handleLogout = useUnit(logoutTriggered)
  const { theme, setTheme } = useTheme()

  return (
    <SidebarProvider>
      <Sidebar collapsible="icon">
        <SidebarHeader>
          <SidebarMenu>
            <SidebarMenuItem>
              <div className="flex items-center gap-2">
                <SidebarTrigger />
                <div className="group-data-[collapsible=icon]:hidden flex-1">
                  <Link to={routes.home} className="block">
                    <div className="grid text-left text-sm leading-tight">
                      <span className="truncate font-semibold">Boilerplate App</span>
                      <span className="truncate text-xs">Dashboard</span>
                    </div>
                  </Link>
                </div>
              </div>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarHeader>

        <SidebarContent>
          <SidebarGroup>
            <SidebarGroupLabel>Navigation</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                {navigationItems.map((item) => (
                  <SidebarMenuItem key={item.title}>
                    <SidebarMenuButton asChild tooltip={item.title}>
                      <Link to={item.route}>
                        <item.icon />
                        <span>{item.title}</span>
                      </Link>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                ))}
                <SidebarMenuItem>
                  <SidebarMenuButton
                    onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
                    tooltip="Toggle Theme"
                  >
                    <Sun className="h-4 w-4 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
                    <Moon className="absolute h-4 w-4 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
                    <span>Theme</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>

        <SidebarFooter>
          {user && (
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton size="lg" className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground">
                  <User className="size-4" />
                  <div className="grid flex-1 text-left text-sm leading-tight">
                    <span className="truncate font-semibold">{user.username}</span>
                    <span className="truncate text-xs">{user.email}</span>
                  </div>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton onClick={() => handleLogout()} tooltip="Logout">
                  <LogOut />
                  <span>Logout</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          )}
        </SidebarFooter>

        <SidebarRail />
      </Sidebar>

      <SidebarInset>
        <header className="flex h-16 shrink-0 items-center gap-2 border-b px-4">
          <div className="ml-auto">
            <ThemeToggle />
          </div>
        </header>
        <div className="flex flex-1 flex-col gap-4 p-4">
          {children}
        </div>
      </SidebarInset>
    </SidebarProvider>
  )
}
