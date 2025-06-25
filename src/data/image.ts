import { create, BaseDirectory, readDir, FileInfo } from '@tauri-apps/plugin-fs';
import { join } from '@tauri-apps/api/path';
import { invoke } from '@tauri-apps/api/core';

export const ImageValidSubfix = [".jpg", ".jpeg", ".png", ".gif", ".bmp", ".webp"];
async function processDirRecursively(parent: string, result: ImagePath[]) {
    const entries = await readDir(parent);
    for (const entry of entries) {
        const abpath = await join(parent, entry.name);
        if (entry.isDirectory) {
            processDirRecursively(abpath, result);
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
    processDirRecursively(directory, result);
    return result;
}

export interface ImageIndexInfo {
    name: string
    path: string
    thumbnail: string
    size: number
    width: number
    height: number
}

export async function indexImage(imagePath: ImagePath, rename: boolean): Promise<ImageIndexInfo | null> {

    return new Promise(resolve => setTimeout(resolve, 1000 + Math.random() * 1000));

}