import { BaseDirectory, exists, readFile, writeTextFile } from '@tauri-apps/plugin-fs';
import { loadLocal as localImgDirs } from './img-dirs';
export async function loadLocal() {
    Promise.all([localImgDirs])
}

export async function saveToFile(content: string, relfilepath: string) {
    relfilepath = `.imgsearch/${relfilepath}`;
    return writeTextFile(relfilepath, content, {
        baseDir: BaseDirectory.Home,
    });
}

export async function loadFromFile(relfilepath: string) {
    relfilepath = `.imgsearch/${relfilepath}`;

    if (await exists('token', {
        baseDir: BaseDirectory.AppLocalData,
    })) {
        const contents = await readFile(relfilepath, {
            baseDir: BaseDirectory.Home,
        });
        return contents.toString();
    } else {
        return null;
    }

}