import { LazyStore } from '@tauri-apps/plugin-store';
import { getAllImageInfo, indexImage } from './image';

export interface ImgDir {
    name: string
    path: string
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

export async function addImgDir(imgDir: ImgDir, process?: (p: ImgDirProcessParams) => Promise<void>) {
    // 判断imgdirs里面的元素，是否是imgdir的父目录

    const ImgDirs = await getAll();
    ImgDirs.forEach(e => {
        if (imgDir.path.startsWith(e.path) || e.path.startsWith(imgDir.path)) {
            throw new Error("Imgdirs already contains a directory that is a parent or child of the specified directory");
        }
    });

    await ImgDirStore.set(imgDir.path, imgDir);

    getAllImageInfo(imgDir.path).then(async (images) => {

        const params = {
            total: images.length,
            current: 0
        } satisfies ImgDirProcessParams;

        for (const image of images) {
            await indexImage(image, imgDir.enableRename);

            params.current++;
            await process?.(params);

        }
    });
}

export async function removeImgDir(imgDirPath: string) {
    await ImgDirStore.delete(imgDirPath);
}