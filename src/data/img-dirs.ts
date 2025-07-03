import { LazyStore } from '@tauri-apps/plugin-store';
import { getAllImageInfo, indexImage, indexImages } from './img';
import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';

export interface ImgDir {
    name: string
    root: string
    enableRename: boolean
}

export interface ImgDirProcessParams {
    total: number
    current: number
}

const ImgDirStore = new LazyStore('ImgDirStore.json');

export async function getAll(): Promise<ImgDir[]> {
    return ImgDirStore.values();
}

export async function addImgDir(imgDir: ImgDir, process?: (p: ImgDirProcessParams) => void) {
    // 判断imgdirs里面的元素，是否是imgdir的父目录

    const ImgDirs = await getAll();
    ImgDirs.forEach(e => {
        if (imgDir.root.startsWith(e.root) || e.root.startsWith(imgDir.root)) {
            throw new Error("Imgdirs already contains a directory that is a parent or child of the specified directory");
        }
    });

    await ImgDirStore.set(imgDir.root, imgDir);
    const images = await getAllImageInfo(imgDir.root);

    const total = images.length;
    const imgProcessSize = 5;

    let i = 0;
    while (i < total) {
        const params = {
            total,
            current: i
        } satisfies ImgDirProcessParams;
        process?.(params);
        const ps = images.slice(i, imgProcessSize).map(e => e.path);
        try {
            await indexImages(imgDir.root, ps, imgDir.enableRename);
        } catch (e) {
            error(`index image error: ${e}, ${ps.join('|')}`);
            throw e;
        }
        i += imgProcessSize;
    }
}

export async function removeImgDir(imgDirPath: string) {
    await ImgDirStore.delete(imgDirPath);
}