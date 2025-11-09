import { Home, LayoutDashboard, LogOut, Moon, Sun, User, Sparkles } from 'lucide-react'
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
import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { routes } from '@/shared/lib/router'
import { $currentUser } from '@/entities/user/model/user'
import { logoutTriggered } from '@/features/auth/logout/model/logout'
import { ThemeToggle } from '@/widgets/theme-toggle'
import { useTheme } from '@/app/providers/theme-provider'
import { SlideMenu } from '@/widgets/slide-menu'

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
      <Sidebar collapsible="icon" className="border-r">
        <SidebarHeader className="border-b border-sidebar-border">
          <SidebarMenu>
            <SidebarMenuItem>
              <div className="flex items-center gap-3 px-2 py-2 group-data-[collapsible=icon]:justify-center group-data-[collapsible=icon]:px-0">
                <SidebarTrigger className="hover:bg-sidebar-accent rounded-md transition-colors" />
                <div className="group-data-[collapsible=icon]:hidden flex-1">
                  <Link to={routes.home} className="block group">
                    <div className="flex items-center gap-2">
                      <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-gradient-to-br from-primary to-primary/80 text-primary-foreground shadow-sm transition-transform group-hover:scale-105">
                        <Sparkles className="h-4 w-4" />
                      </div>
                      <div className="grid text-left text-sm leading-tight">
                        <span className="truncate font-bold tracking-tight">Boilerplate</span>
                        <span className="truncate text-xs text-sidebar-foreground/60">Dashboard</span>
                      </div>
                    </div>
                  </Link>
                </div>
              </div>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarHeader>

        <SidebarContent className="px-2 py-4 group-data-[collapsible=icon]:px-0 group-data-[collapsible=icon]:py-2">
          <SidebarGroup>
            <SidebarGroupLabel className="px-2 text-xs font-semibold text-sidebar-foreground/60">
              Navigation
            </SidebarGroupLabel>
            <SidebarGroupContent className="mt-2">
              <SidebarMenu>
                {navigationItems.map((item) => (
                  <SidebarMenuItem key={item.title}>
                    <SidebarMenuButton
                      asChild
                      tooltip={item.title}
                      className="group/item transition-all hover:bg-sidebar-accent/80 hover:pl-3 data-[active=true]:bg-sidebar-accent data-[active=true]:text-sidebar-accent-foreground data-[active=true]:shadow-sm group-data-[collapsible=icon]:!p-2 group-data-[collapsible=icon]:justify-center"
                    >
                      <Link to={item.route}>
                        <item.icon className="transition-transform group-hover/item:scale-110" />
                        <span className="font-medium group-data-[collapsible=icon]:hidden">{item.title}</span>
                      </Link>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                ))}
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>

          <SidebarSeparator className="my-4" />

          <SidebarGroup>
            <SidebarGroupLabel className="px-2 text-xs font-semibold text-sidebar-foreground/60">
              Settings
            </SidebarGroupLabel>
            <SidebarGroupContent className="mt-2">
              <SidebarMenu>
                <SidebarMenuItem>
                  <SidebarMenuButton
                    onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
                    tooltip="Toggle Theme"
                    className="group/theme transition-all hover:bg-sidebar-accent/80 hover:pl-3 group-data-[collapsible=icon]:!p-2 group-data-[collapsible=icon]:justify-center"
                  >
                    <div className="relative flex h-4 w-4 items-center justify-center">
                      <Sun className="absolute h-4 w-4 rotate-0 scale-100 transition-all group-hover/theme:rotate-12 dark:-rotate-90 dark:scale-0" />
                      <Moon className="absolute h-4 w-4 rotate-90 scale-0 transition-all group-hover/theme:-rotate-12 dark:rotate-0 dark:scale-100" />
                    </div>
                    <span className="font-medium group-data-[collapsible=icon]:hidden">Theme</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>

        <SidebarFooter className="mt-auto border-t border-sidebar-border">
          {user && (
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  size="lg"
                  tooltip={{
                    children: (
                      <div className="flex flex-col gap-0.5">
                        <span className="font-semibold">{user.username}</span>
                        <span className="text-xs text-muted-foreground">{user.email}</span>
                      </div>
                    ),
                  }}
                  className="group/user data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground hover:bg-sidebar-accent/80 transition-all group-data-[collapsible=icon]:!p-2 group-data-[collapsible=icon]:justify-center"
                >
                  <Avatar className="h-8 w-8 rounded-lg border-2 border-sidebar-border transition-transform group-hover/user:scale-105">
                    <AvatarFallback className="rounded-lg bg-gradient-to-br from-primary/20 to-primary/10 text-xs font-semibold">
                      {user.username.slice(0, 2).toUpperCase()}
                    </AvatarFallback>
                  </Avatar>
                  <div className="grid flex-1 text-left text-sm leading-tight group-data-[collapsible=icon]:hidden">
                    <span className="truncate font-semibold">{user.username}</span>
                    <span className="truncate text-xs text-sidebar-foreground/60">{user.email}</span>
                  </div>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton
                  onClick={() => handleLogout()}
                  tooltip="Logout"
                  className="group/logout hover:bg-destructive/10 hover:text-destructive transition-all hover:pl-3 group-data-[collapsible=icon]:!p-2 group-data-[collapsible=icon]:justify-center"
                >
                  <LogOut className="transition-transform group-hover/logout:translate-x-0.5" />
                  <span className="font-medium group-data-[collapsible=icon]:hidden">Logout</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          )}
        </SidebarFooter>

        <SidebarRail />
      </Sidebar>

      <SidebarInset>
        <header className="flex h-16 shrink-0 items-center gap-2 border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 px-4 sticky top-0 z-10">
          <div className="ml-auto">
            <ThemeToggle />
          </div>
        </header>
        <div className="flex flex-1 flex-col gap-4 p-4">
          {children}
        </div>
      </SidebarInset>

      <SlideMenu />
    </SidebarProvider>
  )
}
