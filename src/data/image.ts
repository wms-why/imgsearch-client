import { create, BaseDirectory, readDir, FileInfo } from '@tauri-apps/plugin-fs';
import { join } from '@tauri-apps/api/path';
import { invoke } from '@tauri-apps/api/core';

export const ImageValidSubfix = [".jpg", ".jpeg", ".png", ".webp"];
async function processDirRecursively(parent: string, result: ImagePath[]) {
  const entries = await readDir(parent);
  for (const entry of entries) {
    const abpath = await join(parent, entry.name);
    if (entry.isDirectory) {
      await processDirRecursively(abpath, result);
    } else {
      ImageValidSubfix.find(subfix => entry.name.endsWith(subfix)) && result.push({ path: abpath, name: entry.name });
    }
  }
}

export interface ImagePath {
  path: string;
  name: string;
}

export async function getAllImageInfo(directory: string) {
  const result: ImagePath[] = [];
  await processDirRecursively(directory, result);

  return result;
}

export async function indexImage(imagePath: ImagePath, rename: boolean) {

  await invoke("index_image", { model: { ...imagePath, rename } });

}

export interface ImagePaths {
  rootDir: string;
  paths: string[];
}
export async function indexImages(rootDir: string, paths: string[], rename: boolean) {

  return await invoke<void>("index_images", {
    model: {
      rootDir,
      paths,
      rename
    }
  });

}

export interface SearchResult {
  name: string,
  path: string,
  root: string,
  thumbnail: string,
  desc: string | null,
  score: number,
}
export async function search(keyword: string, top: number) {
  return await invoke<SearchResult[]>("search", { model: { keyword, top } });
}