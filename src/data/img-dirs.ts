import { LazyStore, load } from '@tauri-apps/plugin-store';

export interface ImgDir {
    name: string
    path: string
    enableRename: boolean
}


const ImgDirStore = new LazyStore('ImgDirStore.json');

export async function getAll(): Promise<ImgDir[]> {
    return ImgDirStore.values();
}

export async function addImgDir(imgDir: ImgDir) {
    // 判断imgdirs里面的元素，是否是imgdir的父目录

    const ImgDirs = await getAll();
    ImgDirs.forEach(e => {
        if (imgDir.path.startsWith(e.path) || e.path.startsWith(imgDir.path)) {
            throw new Error("Imgdirs already contains a directory that is a parent or child of the specified directory");
        }
    });

    await ImgDirStore.set(imgDir.path, imgDir);
}

export async function removeImgDir(imgDirPath: string) {
    await ImgDirStore.delete(imgDirPath);
}