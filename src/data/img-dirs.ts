import { LazyStore } from '@tauri-apps/plugin-store';
import { getAllImageInfo, indexImage, indexImages } from './image';
import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';
import { watch, WatchEventKind, WatchEventKindCreate } from '@tauri-apps/plugin-fs';
import { invoke } from '@tauri-apps/api/core';

export interface ImgDir {
    name: string
    root: string
    enableRename: boolean
}

export interface ImgDirProcessParams {
    error: string | null
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

    setTimeout(async () => {
        const images = await getAllImageInfo(imgDir.root);

        const total = images.length;
        const imgProcessSize = 5;

        let i = 0;
        while (i < total) {
            const params = {
                error: null,
                total,
                current: i
            } satisfies ImgDirProcessParams;

            process?.(params);
            const ps = images.slice(i, imgProcessSize).map(e => e.path);
            try {
                await indexImages(imgDir.root, ps, imgDir.enableRename);
            } catch (e) {
                let msg = `index image error: ${e}}`;
                error(msg);
                const params = {
                    error: msg,
                    total,
                    current: i
                } satisfies ImgDirProcessParams;

                process?.(params);
            }

            i += imgProcessSize;
        }
    }, 1000)

}

export async function removeImgDir(imgDirPath: string) {
    await ImgDirStore.delete(imgDirPath);
    // removeRoot(imgDirPath);
}


export async function onStartup() {
    const ps = await getAll();

    function isKind<K extends string>(obj: any, kind: K): obj is Record<K, any> {
        return typeof obj === 'object' && obj !== null && (kind in obj || kind == obj.kind);
    }

    ps.forEach(async dir => {
        let modifyTimeout: NodeJS.Timeout | null = null;

        await watch(
            dir.root,
            (event) => {

                console.log(event);

                const { type } = event;

                /**
                 * win11 创建文件与修改文件事件重叠，忽略create事件
                 * 忽略创建文件夹
                 */
                // if (isKind(type, 'create') && isKind(type.create, "file")) {
                //     debug(`create file, ${event.paths}`);
                // }

                /**
                 * modify any 为新增文件、文件夹
                 * 当对象是文件夹时忽略
                 */
                if (isKind(type, 'modify')) {
                    if (isKind(type.modify, "rename")) {
                        invoke<void>("rename", { model: { path: event.paths[0], newPath: event.paths[1] } })
                        console.log("modify rename", event.paths);
                    } else {
                        console.log("modify any", event.paths);
                    }
                }

                if (isKind(type, 'remove')) {
                    if (isKind(type.remove, "file")) {
                        console.log("remove file", event.paths);
                    }
                    if (isKind(type.remove, "folder")) {
                        console.log("remove folder", event.paths);
                    }
                }

            },
            {
                recursive: true,
                delayMs: 500,
            }
        );

        console.log("watching " + dir.root + " success");

    })
} 