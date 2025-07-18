'use client'
import { getAll, SearchResult } from "@/data/image"
import { useEffect, useState } from "react"
import { useToast } from "@/components/ui/use-toast";
import { convertFileSrc } from "@tauri-apps/api/core";

export default function ImagesPage() {

  const [images, setImages] = useState<SearchResult[]>([]);

  const { toast } = useToast();
  useEffect(() => {
    getAll().then(data => {
      setImages(data);
    }).catch(err => {
      toast({
        title: "error",
        description: err,
        variant: "destructive",
      });
    })

  }, [])

  return (
    <div className="container mx-auto py-4">
      <h1 className="text-2xl font-bold mb-4">Image Library</h1>
      <div className="space-y-2">
        {images.length === 0 ? (
          <div className="flex items-center justify-center h-64 text-gray-500">
            No Content
          </div>
        ) : (
          <>
            {images.map((image) => (
              <div key={image.path} className="flex items-center gap-4 p-2 hover:bg-gray-100 rounded" title={image.path}>
                <img
                  src={convertFileSrc(image.thumbnail)}
                  alt={image.name}
                  className="w-12 h-12 object-cover rounded"
                />
                <div className="flex-1">
                  <p className="font-medium">{image.name}</p>
                  <p className="text-sm text-gray-500">{image.desc || "No description"}</p>
                </div>
                <div className="flex items-center gap-2">
                  <span className={`px-2 py-1 text-xs rounded ${image.idxed ? 'bg-green-100 text-green-800' : 'bg-yellow-100 text-yellow-800'
                    }`}>
                    {image.idxed ? 'indexed' : 'None Indexed'}
                  </span>
                  {/* <button
                    className="px-3 py-1 text-sm bg-blue-500 text-white rounded hover:bg-blue-600 disabled:opacity-50"
                    disabled={image.idxed}
                  >
                    Manual Index
                  </button> */}
                </div>
              </div>
            ))}
          </>

        )
        }
      </div>
    </div>
  )
}