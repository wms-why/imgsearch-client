"use client"

import { useCallback, useEffect, useState } from "react"
import Image from "next/image"
import { usePathname } from "next/navigation"
import logo from "@/assets/icons/32x32.png"
import { Globe, Mic, Sailboat } from "lucide-react"
import { WindowControls, WindowTitlebar } from "tauri-controls"

import {
  Menubar,
  MenubarCheckboxItem,
  MenubarContent,
  MenubarItem,
  MenubarLabel,
  MenubarMenu,
  MenubarRadioGroup,
  MenubarRadioItem,
  MenubarSeparator,
  MenubarShortcut,
  MenubarSub,
  MenubarSubContent,
  MenubarSubTrigger,
  MenubarTrigger,
} from "@/components/ui/menubar"

import { AboutDialog } from "./about-dialog"
import { ExamplesNav } from "./examples-nav"
import { MenuModeToggle } from "./menu-mode-toggle"
import { Dialog, DialogTrigger } from "./ui/dialog"
import Link from "next/link"
export function Menu() {
  const closeWindow = useCallback(async () => {
    const { getCurrentWindow } = await import("@tauri-apps/api/window")
    const appWindow = getCurrentWindow()
    appWindow.close()
  }, [])

  return (
    <WindowTitlebar
    // controlsOrder="left"
    // className="pl-0"
    // windowControlsProps={{ platform: "windows", justify: false }}
    >
      <Menubar className="rounded-none border-b border-none pl-2 lg:pl-3">
        <MenubarMenu>
          {/* App Logo */}
          <div className="inline-flex h-fit w-fit items-center text-cyan-500">
            <Image src={logo} alt="logo" width={20} height={20} />
          </div>
        </MenubarMenu>

        <MenubarMenu>
          <MenubarTrigger className="font-bold">App</MenubarTrigger>
          <Dialog modal={false}>
            <MenubarContent>

              {/* <MenubarItem>
                <Link href="/imgdir">Image Directory</Link>
              </MenubarItem> */}
              <MenubarItem>
                <Link href="/guide">Guide</Link>
              </MenubarItem>

              <MenubarItem>
                <Link href="/terms">Terms of Service</Link>
              </MenubarItem>

              <MenubarItem>
                <Link href="/privacy">Privacy</Link>
              </MenubarItem>

              <MenubarItem onClick={closeWindow}>
                Close
              </MenubarItem>
              <DialogTrigger asChild>
                <MenubarItem>About App</MenubarItem>
              </DialogTrigger>
            </MenubarContent>

            <AboutDialog />
          </Dialog>
        </MenubarMenu>

        <MenubarMenu>
          <MenubarTrigger >
            <Link href="/search">Search</Link>
          </MenubarTrigger>
        </MenubarMenu>

        <MenubarMenu>
          <MenubarTrigger >
            <Link href="/imgdir">Directory</Link>
          </MenubarTrigger>
        </MenubarMenu>

        <MenubarMenu>
          <MenubarTrigger >
            <Link href="/images">Library</Link>
          </MenubarTrigger>
        </MenubarMenu>



        <MenuModeToggle />
        {/* 
        <ExamplesNav /> */}
      </Menubar>
    </WindowTitlebar>
  )
}
