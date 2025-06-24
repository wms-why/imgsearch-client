import { create, BaseDirectory, readDir, FileInfo } from '@tauri-apps/plugin-fs';
import { join } from '@tauri-apps/api/path';

export const ImageValidSubfix = [".jpg", ".jpeg", ".png", ".gif", ".bmp", ".webp"];
async function processDirRecursively(parent: string, result: string[]) {
    const entries = await readDir(parent);
    for (const entry of entries) {
        const abpath = await join(parent, entry.name);
        if (entry.isDirectory) {
            processDirRecursively(abpath, result);
        } else {
            ImageValidSubfix.find(subfix => entry.name.endsWith(subfix)) && result.push(abpath);
        }
    }
}
export async function getAllImageInfo(directory: string) {
    const result: string[] = [];
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
export async function indexImage(imagePath: string, rename: boolean): Promise<ImageIndexInfo> {

}