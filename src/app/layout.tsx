import "@/styles/globals.css"

import { fontMono, fontSans } from "@/lib/fonts"
import { cn } from "@/lib/utils"
import { Initing } from "@/components/Initing"
import { Menu } from "@/components/menu"
import { StyleSwitcher } from "@/components/style-switcher"
import { TailwindIndicator } from "@/components/tailwind-indicator"
import { ThemeProvider } from "@/components/theme-provider"
import { Toaster } from "@/components/ui/toaster"
interface ExamplesLayoutProps {
  children: React.ReactNode
}

export default async function MyApp({ children }: ExamplesLayoutProps) {
  return (
    <html lang="en" suppressHydrationWarning className="overflow-clip bg-black">
      <head />
      <body className="overflow-clip bg-transparent font-sans antialiased scrollbar-none">
        <Initing />
        <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
          <div className="h-screen overflow-clip">
            <Menu />
            <div
              className={cn(
                "h-screen overflow-auto border-t bg-background pb-8",
                // "scrollbar-none"
                "scrollbar scrollbar-track-transparent scrollbar-thumb-accent scrollbar-thumb-rounded-md"
              )}
            >
              {children}
            </div>
          </div>
          <TailwindIndicator />
          <Toaster />
        </ThemeProvider>
        <StyleSwitcher />
      </body>
    </html>
  )
}
