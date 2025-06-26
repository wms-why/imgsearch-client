import { create, BaseDirectory, readDir, FileInfo } from '@tauri-apps/plugin-fs';
import { join } from '@tauri-apps/api/path';
import { invoke } from '@tauri-apps/api/core';

export const ImageValidSubfix = [".jpg", ".jpeg", ".png", ".webp"];
async function processDirRecursively(root: string, parent: string, result: ImagePath[]) {
    const entries = await readDir(parent);
    for (const entry of entries) {
        const abpath = await join(parent, entry.name);
        if (entry.isDirectory) {
            processDirRecursively(root, abpath, result);
        } else {
            ImageValidSubfix.find(subfix => entry.name.endsWith(subfix)) && result.push({ path: abpath, name: entry.name, rootDir: root });
        }
    }
}

export interface ImagePath {
    rootDir: string;
    path: string;
    name: string;
}

export async function getAllImageInfo(directory: string) {
    const result: ImagePath[] = [];
    processDirRecursively(directory, directory, result);

    return result;
}

export async function indexImage(imagePath: ImagePath, rename: boolean) {

    await invoke("index_image", { model: { ...imagePath, rename } });

}
