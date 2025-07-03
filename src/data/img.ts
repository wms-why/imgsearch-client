import { readDir } from '@tauri-apps/plugin-fs';
import { basename, join } from '@tauri-apps/api/path';
import { invoke } from '@tauri-apps/api/core';
import { generateThumbnailPath, renameto } from './file-tools';
import { reqImgIdxes } from './server';
import { ImgIdx, saveBatch } from './img-vec-idx';
import { path } from '@tauri-apps/api';

export const ImageValidSubfix = [".jpg", ".jpeg", ".png", ".webp"];
async function processDirRecursively(parent: string, result: ImagePath[]) {
  const entries = await readDir(parent);
  for (const entry of entries) {
    const abpath = await join(parent, entry.name);
    if (entry.isDirectory) {
      processDirRecursively(abpath, result);
    } else {
      ImageValidSubfix.find(subfix => entry.name.endsWith(subfix)) && result.push({ path: abpath, name: entry.name });
    }
  }
}

export interface ImagePath {
  path: string;
  name: string;
}

export async function getAllImageInfo(directory: string) {
  const result: ImagePath[] = [];
  processDirRecursively(directory, result);

  return result;
}

export async function indexImage(imagePath: ImagePath, rename: boolean) {

  await invoke("index_image", { model: { ...imagePath, rename } });

}

export interface ImagePaths {
  rootDir: string;
  paths: string[];
}
export async function indexImages(rootDir: string, paths: string[], rename: boolean) {

  const tps: string[] = [];
  const tphs = [];
  for (let p of paths) {
    const ext = "." + path.extname(p);
    const tp = await generateThumbnailPath(ext);
    tps.push(tp);
    tphs.push(
      invoke("generate_thumbnail", {
        model: {
          source_path: p,
          target_path: tp
        }
      })
    )
  }

  await Promise.all(tphs);

  const resp = await reqImgIdxes(tps, rename);

  if (rename) {
    for (let i = 0; i < paths.length; i++) {
      const newpath = await renameto(paths[i], resp[i].name!);
      paths[i] = newpath;
    }
  }

  const batch = await Promise.all(resp.map(async (e, i) => {
    let name = await basename(paths[i]);
    return {
      name,
      path: paths[i],
      root: rootDir,
      thumbnail: tps[i],
      desc: e.desc,
      vector: e.vec
    } satisfies ImgIdx
  }));
  await saveBatch(batch);

}

