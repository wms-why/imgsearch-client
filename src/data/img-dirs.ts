import { LazyStore } from '@tauri-apps/plugin-store';
import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';
import { watch, WatchEventKind, WatchEventKindCreate } from '@tauri-apps/plugin-fs';
import { invoke } from '@tauri-apps/api/core';
import { set } from 'date-fns';
const ImgDirStore = new LazyStore('ImgDirStore.json');

export interface ImgDir {
    name: string
    root: string
    enableRename: boolean
    createTime: Date
}


export async function getAll(): Promise<ImgDir[]> {
    return ImgDirStore.values();
}

export async function addImgDir(imgDir: ImgDir) {
    // 判断imgdirs里面的元素，是否是imgdir的父目录

    const ImgDirs = await getAll();
    ImgDirs.forEach(e => {
        if (imgDir.root.startsWith(e.root) || e.root.startsWith(imgDir.root)) {
            throw new Error("Imgdirs already contains a directory that is a parent or child of the specified directory");
        }
    });

    await ImgDirStore.set(imgDir.root, imgDir);
    invoke("after_add_imgdir", { root: imgDir.root, rename: imgDir.enableRename }).then(() => {
        watchImgdir(imgDir.root);
    })
}

export async function removeImgDir(imgDirPath: string) {
    await ImgDirStore.delete(imgDirPath);
    invoke("after_remove_imgdir", { root: imgDirPath });
}

async function watchImgdir(root: string) {

    function isKind<K extends string>(obj: any, kind: K): obj is Record<K, any> {
        return typeof obj === 'object' && obj !== null && (kind in obj || kind == obj.kind);
    }

    let modifyAnyTimeout: NodeJS.Timeout | null = null;
    let modifyAnyList = [];
    await watch(
        root,
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


            if (isKind(type, 'modify')) {
                if (isKind(type.modify, "rename")) {
                    invoke("rename", { model: { path: event.paths[0], newPath: event.paths[1] } })
                    console.log("modify rename", event.paths);
                } else {

                    /**
                     * modify any 为新增文件、文件夹、覆盖文件
                     * 当对象是文件夹时忽略
                     */
                    modifyAnyList.push(event.paths[0]);
                    if (modifyAnyTimeout != null) {
                        console.log("continue modify any", modifyAnyList);
                        clearTimeout(modifyAnyTimeout);
                    } else {
                        console.log("start modify any", modifyAnyList);
                    }
                    modifyAnyTimeout = setTimeout(() => {
                        ImgDirStore.get<ImgDir>(root).then((imgdir) => {
                            imgdir = imgdir!;
                            invoke("modify_content", { root: root, path: modifyAnyList, rename: imgdir.enableRename });
                        });
                        console.log("modify any success", modifyAnyList);
                        modifyAnyTimeout = null;
                        modifyAnyList.length = 0;
                    }, 5000);
                }
            }

            if (isKind(type, 'remove')) {
                console.log("remove file", event.paths);
                invoke("remove", { path: event.paths });
            }

        },
        {
            recursive: true,
            delayMs: 500,
        }
    );

    console.log("watching " + root + " success");
}
export async function onStartup() {
    const ps = await getAll();
    ps.forEach(async dir => {
        watchImgdir(dir.root)
    });
} 