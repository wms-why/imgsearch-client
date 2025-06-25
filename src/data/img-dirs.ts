import { LazyStore } from '@tauri-apps/plugin-store';
import { getAllImageInfo, indexImage } from './image';

const indexImageSize = 5;
export interface ImgDir {
    name: string
    path: string
    enableRename: boolean
}

export interface ImgDirProcessParams {
    total: number
    current: number
    currentName: string
}

const ImgDirStore = new LazyStore('ImgDirStore.json');

export async function getAll(): Promise<ImgDir[]> {
    return ImgDirStore.values();
}

export async function addImgDir(imgDir: ImgDir, process?: (p: ImgDirProcessParams) => void) {
    // 判断imgdirs里面的元素，是否是imgdir的父目录

    const ImgDirs = await getAll();
    ImgDirs.forEach(e => {
        if (imgDir.path.startsWith(e.path) || e.path.startsWith(imgDir.path)) {
            throw new Error("Imgdirs already contains a directory that is a parent or child of the specified directory");
        }
    });

    await ImgDirStore.set(imgDir.path, imgDir);

    getAllImageInfo(imgDir.path).then(async (images) => {

        const total = images.length;
        let i = 0;

        let runCount = 0;

        while (runCount < indexImageSize && i < total) {
            processItem()
            runCount++;
        }
        function processItem() {

            const params = {
                total,
                current: i + 1,
                currentName: images[i].name,
            } satisfies ImgDirProcessParams;
            process?.(params);

            indexImage(images[i], imgDir.enableRename).then(() => {
                if (i < total) {
                    setTimeout(processItem, 100);
                }
            });

            i++;
        }
    });
}

export async function removeImgDir(imgDirPath: string) {
    await ImgDirStore.delete(imgDirPath);
}