import path from "path";
import { loadFromFile, saveToFile } from ".";

const ImgDirFilename = "img-dirs.json";
let ImgDirs: string[] = [];
export async function loadLocal() {
    const content = await loadFromFile(ImgDirFilename);
    if (content) {
        ImgDirs = JSON.parse(content);
    }
}

export async function addImgDir(imgDir: string) {
    // 判断imgdirs里面的元素，是否是imgdir的父目录
    ImgDirs.forEach(e => {
        if (imgDir.startsWith(e) || e.startsWith(imgDir)) {
            throw new Error("imgdirs already contains a directory that is a parent or child of the specified directory");
        }
    });
    ImgDirs.push(imgDir);
    await saveToFile(JSON.stringify(ImgDirs), ImgDirFilename);
}

export async function removeImgDir(imgDir: string) {
    ImgDirs = ImgDirs.filter((dir) => dir !== imgDir);
    await saveToFile(JSON.stringify(ImgDirs), ImgDirFilename);
}