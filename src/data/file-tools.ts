
import { stat, BaseDirectory, exists, mkdir, readFile, rename } from '@tauri-apps/plugin-fs';
import { dirname, extname, join } from '@tauri-apps/api/path';
import { randomUUID } from 'node:crypto';

const appDataDir = ".imgsearch";
const dbDir = "db_dir";
const thumbnailDir = "thumbnail_dir";

export async function init() {
  await mkdir(await getDBDir(), {
    baseDir: BaseDirectory.Home,
    recursive: true
  });

  await mkdir(await join(appDataDir, thumbnailDir), {
    baseDir: BaseDirectory.Home,
    recursive: true
  });
}

export async function getDBDir() {
  return await join(appDataDir, dbDir);
}

/**
 * 
 * @param ext 文件后缀 包含.
 * @returns 
 */
export async function generateThumbnailPath(ext: string) {

  let newname = randomUUID().replaceAll("-", "");
  let p = await join(appDataDir, thumbnailDir, newname + ext);

  while (await exists(p, {
    baseDir: BaseDirectory.Home,
  })) {
    newname = randomUUID().replaceAll("-", "");
    p = await join(appDataDir, thumbnailDir, newname + ext);
  }

  return p;
}

export async function readThumbnail(path: string) {
  const bs = await readFile(path, { baseDir: BaseDirectory.Home })
  return new Blob([bs], { type: 'application/octet-stream' });
}

/**
 * 
 * @param p 
 * @param newname 
 */
export async function renameto(sp: string, newname: string) {

  const dir = await dirname(sp);
  const ext = await extname(sp);
  let newpath = await join(dir, newname + "." + ext);

  let i = 1;
  while (await exists(newpath, { baseDir: BaseDirectory.Home })) {
    newpath = await join(dir, newname + "_" + i + "." + ext);
    i++;
  }

  await rename(sp, newpath, { oldPathBaseDir: BaseDirectory.Home, newPathBaseDir: BaseDirectory.Home });

  return newpath;
}