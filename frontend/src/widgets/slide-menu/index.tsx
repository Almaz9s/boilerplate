import * as React from 'react'
import { Command, FileText, Settings, Terminal } from 'lucide-react'

import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
} from '@/components/ui/sheet'

const SLIDE_MENU_WIDTH = '16rem' // Same as expanded sidebar
const SLIDE_MENU_KEYBOARD_SHORTCUT = 'Backquote' // Physical key code, not character

interface SlideMenuProps {
  children?: React.ReactNode
}

export function SlideMenu({ children }: SlideMenuProps) {
  const [open, setOpen] = React.useState(false)

  // Handle keyboard shortcut Ctrl+` (supports all keyboard layouts via physical key position)
  React.useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      // Check for Ctrl/Cmd modifier and physical key position
      // event.code represents the physical key, not the character it produces
      // This works universally: US layout (`), Russian layout (ё), etc.
      if ((event.metaKey || event.ctrlKey) && event.code === SLIDE_MENU_KEYBOARD_SHORTCUT) {
        event.preventDefault()
        setOpen((prev) => !prev)
      }
    }

    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [])

  return (
    <Sheet open={open} onOpenChange={setOpen}>
      <SheetContent
        side="left"
        className="w-[var(--slide-menu-width)] p-0"
        style={
          {
            '--slide-menu-width': SLIDE_MENU_WIDTH,
          } as React.CSSProperties
        }
      >
        <SheetHeader className="p-6 pb-4">
          <SheetTitle>Quick Menu</SheetTitle>
          <SheetDescription>
            Press{' '}
            <kbd className="pointer-events-none inline-flex h-5 select-none items-center gap-1 rounded border bg-muted px-1.5 font-mono text-[10px] font-medium text-muted-foreground opacity-100">
              <span className="text-xs">Ctrl</span>+`
            </kbd>{' '}
            to toggle
          </SheetDescription>
        </SheetHeader>

        <div className="flex flex-col gap-2 px-6">
          {children || (
            <>
              <div className="space-y-4">
                <div className="space-y-2">
                  <h3 className="text-sm font-semibold">Quick Actions</h3>
                  <div className="space-y-1">
                    <button className="flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm hover:bg-accent hover:text-accent-foreground">
                      <Terminal className="size-4" />
                      <span>Open Terminal</span>
                    </button>
                    <button className="flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm hover:bg-accent hover:text-accent-foreground">
                      <FileText className="size-4" />
                      <span>New Document</span>
                    </button>
                    <button className="flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm hover:bg-accent hover:text-accent-foreground">
                      <Settings className="size-4" />
                      <span>Settings</span>
                    </button>
                    <button className="flex w-full items-center gap-3 rounded-md px-3 py-2 text-sm hover:bg-accent hover:text-accent-foreground">
                      <Command className="size-4" />
                      <span>Command Palette</span>
                    </button>
                  </div>
                </div>

                <div className="space-y-2">
                  <h3 className="text-sm font-semibold">Recent Items</h3>
                  <div className="space-y-1">
                    <div className="rounded-md px-3 py-2 text-sm text-muted-foreground">
                      No recent items
                    </div>
                  </div>
                </div>
              </div>
            </>
          )}
        </div>
      </SheetContent>
    </Sheet>
  )
}
