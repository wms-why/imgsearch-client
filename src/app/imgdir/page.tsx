"use client";

import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { Dialog, DialogTrigger, DialogContent, DialogHeader, DialogTitle, DialogDescription } from "@/components/ui/dialog";
import { Checkbox } from "@/components/ui/checkbox";
import { getAll, addImgDir, removeImgDir, ImgDir } from "@/data/img-dirs";
import { Trash2, Plus, FolderOpen } from "lucide-react";
import { useToast } from "@/components/ui/use-toast";
import { open } from '@tauri-apps/plugin-dialog';

export default function ImgdirPage() {
  const [imgDirs, setImgDirs] = useState<ImgDir[]>([]);
  const [newDir, setNewDir] = useState<ImgDir>({ name: "", path: "", enableRename: false });
  const [loading, setLoading] = useState(false);
  const [dialogOpen, setDialogOpen] = useState(false);

  const { toast } = useToast();
  useEffect(() => {
    loadImgDirs();
  }, []);

  const loadImgDirs = async () => {
    setLoading(true);
    try {
      const dirs = await getAll();
      setImgDirs(dirs);
    } catch (error) {
      toast({
        title: "Failed to load",
        description: error instanceof Error ? error.message : "Unknown error",
        variant: "destructive",
      });
    } finally {
      setLoading(false);
    }
  };

  const handDialogChange = (open: boolean) => {
    setDialogOpen(open);
    if (open) {
      setNewDir({ name: "", path: "", enableRename: false });
    }
  }

  const handleSelectDirectory = async () => {
    try {
      const selected = await open({
        title: "Select directory",
        multiple: false,
        directory: true,
      });

      if (selected) {
        const dirName = selected.split(/[\\/]/).pop() || '';
        setNewDir({ ...newDir, name: dirName, path: selected });
      }
    } catch (error) {
      toast({
        title: "Selection failed",
        description: error instanceof Error ? error.message : "Unknown error",
        variant: "destructive",
      });
    }
  };

  const handleAddDir = async () => {
    if (!newDir.name || !newDir.path) {
      toast({
        title: "Incomplete input",
        description: "Please fill in directory name and path",
        variant: "destructive",
      });
      return;
    }

    try {
      await addImgDir(newDir);
      handDialogChange(false);
      await loadImgDirs();
      toast({
        title: "Success",
        description: "Image directory added successfully",
      });
    } catch (error) {
      toast({
        title: "Failed to add",
        description: error instanceof Error ? error.message : "Unknown error",
        variant: "destructive",
      });
    }
  };

  const handleRemoveDir = async (path: string) => {
    try {
      await removeImgDir(path);
      await loadImgDirs();
      toast({
        title: "Success",
        description: "Image directory removed successfully",
      });
    } catch (error) {
      toast({
        title: "Failed to remove",
        description: error instanceof Error ? error.message : "Unknown error",
        variant: "destructive",
      });
    }
  };

  return (
    <div className="container mx-auto py-8">
      <Dialog open={dialogOpen} onOpenChange={handDialogChange}>
        <DialogTrigger asChild>
          <Button className="mb-6" variant="ghost" >
            <Plus className="mr-2 h-4 w-4" /> Add Directory
          </Button>
        </DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Add New Directory</DialogTitle>
            <DialogDescription>
              Select a directory to add to your image library
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-4">


            <div className="flex gap-2">
              <Input
                placeholder="Directory path"
                value={newDir.path}
                readOnly
              />

              <Button variant="outline" onClick={handleSelectDirectory}>
                <FolderOpen className="mr-2 h-4 w-4" /> Select
              </Button>

            </div>


            <Input
              placeholder="Directory name"
              value={newDir.name}
              onChange={(e) => setNewDir({ ...newDir, name: e.target.value })}
            />

            <div className="flex items-center space-x-2">
              <Checkbox
                id="rename"
                checked={newDir.enableRename}
                onCheckedChange={(checked) => setNewDir({ ...newDir, enableRename: checked as boolean })}
              />
              <label htmlFor="rename" className="text-sm font-medium leading-none">
                Allow renaming files in this directory
              </label>
            </div>



            <Button onClick={handleAddDir} className="w-full">
              Add Directory
            </Button>
          </div>
        </DialogContent>
      </Dialog>

      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Name</TableHead>
            <TableHead>Path</TableHead>
            <TableHead>Allow Rename</TableHead>
            <TableHead>Actions</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {imgDirs.map((dir) => (
            <TableRow key={dir.path}>
              <TableCell>{dir.name}</TableCell>
              <TableCell className="font-mono text-sm">{dir.path}</TableCell>
              <TableCell>
                <Checkbox
                  checked={dir.enableRename}
                  disabled
                  className="ml-2"
                />
              </TableCell>
              <TableCell>
                <Button
                  variant="destructive"
                  size="sm"
                  onClick={() => handleRemoveDir(dir.path)}
                >
                  <Trash2 className="h-4 w-4" />
                </Button>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
}
