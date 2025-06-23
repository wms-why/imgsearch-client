'use client'
import { Metadata } from "next"
import Image from "next/image"
import { PlusCircledIcon } from "@radix-ui/react-icons"
import { useState, useCallback } from "react"

import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { ScrollArea, ScrollBar } from "@/components/ui/scroll-area"
import { Separator } from "@/components/ui/separator"
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@/components/ui/tabs"

import { AlbumArtwork } from "./components/album-artwork"

interface Album {
  name: string
  artist: string
  cover: string
}

export default function MusicPage() {
  const [keywords, setKeywords] = useState('')
  const [isSearching, setIsSearching] = useState(false)
  const [searchResults, setSearchResults] = useState<Album[]>([])

  const handleSearch = () => {
    if (keywords.trim() === '') {
      setSearchResults([])
      return
    }

    setIsSearching(true)
    // 模拟API调用
    setTimeout(() => {
      setSearchResults([
        {
          name: `搜索结果1 - ${keywords}`,
          artist: 'AI生成',
          cover: '/icons/128x128.png'
        },
        {
          name: `搜索结果2 - ${keywords}`,
          artist: 'AI生成',
          cover: '/icons/128x128.png'
        }
      ])
      setIsSearching(false)
    }, 1000)
  }

  return (
    <>
      <div>
        {/* <Menu /> */}
        <div className="border-t">
          <div className="bg-background">
            <div className="grid lg:grid-cols-5">
              <div className="col-span-3 lg:col-span-5 lg:border-l">
                <div className="h-full px-4 py-6 lg:px-8">

                  <div className="space-y-4  ">

                    <div className="flex h-10 w-full items-center gap-2 ">
                      <div className="text-xl w-18 font-bold ">Search</div>
                      <Input
                        placeholder="please input description..."
                        onChange={(e) => setKeywords(e.target.value)}
                        onKeyDown={(e) => {
                          if (e.key === 'Enter') {
                            handleSearch()
                          }
                        }}
                        className="flex-1 h-9"
                      />
                      <Button
                        onClick={() => handleSearch()}
                        className="h-9 px-4"
                      >
                        Search
                      </Button>
                    </div>

                    <Separator />
                    {isSearching ? (
                      <div className="py-8 text-center text-muted-foreground">
                        Searching...
                      </div>
                    ) : searchResults.length === 0 ? (
                      <div className="py-8 text-center text-muted-foreground">
                        No Pictures Found
                      </div>
                    ) : (
                      <div className="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5">
                        {searchResults.map((album) => (
                          <AlbumArtwork
                            key={album.name}
                            album={album}
                            className="w-full"
                            aspectRatio="square"
                            width={200}
                            height={200}
                          />
                        ))}
                      </div>
                    )}
                  </div>

                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </>
  )
}
