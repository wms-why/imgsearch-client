/**
 * 支持多种服务提供商，暂未开始
 */

import { getApikey } from "./auth";
import { readThumbnail } from "./file-tools";
import { fetch } from '@tauri-apps/plugin-http';

export interface ImageIndexResp {
  vec: number[],
  desc: string,
  name: string | undefined | null,
}

export async function reqImgIdxes(thumbnails: string[], rename: boolean) {

  const apikey = await getApikey();

  if (!apikey) {
    throw new Error("auth error, lack of apikey");
  }

  const reads = thumbnails.map(p => {
    return readThumbnail(p)
  });

  const thumbnailsBytes = await Promise.all(reads);

  const form = new FormData();
  form.append("rename", rename ? "true" : "false")
  thumbnailsBytes.forEach((b, i) => {
    form.append("thumbnail_" + i, b);
  });

  const r = await fetch(`${process.env.NEXT_PUBLIC_IMGSEARCH_HOST}/api/image_index/v1`, {
    method: "POST",
    headers: { "Authorization": `Bearer ${apikey}`, "content-type": "multipart/form-data" },
    body: form,
  });

  if (r.ok) {
    return r.json() as Promise<ImageIndexResp[]>;
  }

  throw new Error(await r.text());

}