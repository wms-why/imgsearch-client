/**
 * 支持多种服务提供商，暂未开始
 */

// import { LazyStore } from '@tauri-apps/plugin-store';

// const ServerStore = new LazyStore('Server-Item.json');

// export type ServerType = "imgsearch-official";
// export async function currentValid() {
//     const current = await ServerStore.get("current");
//     if (!current) return false;

//     if (current == "imgsearch-official") {
//         return imgsearchOfficialValid();
//     }

// }
// export async function getCurrent(): Promise<ServerType | undefined> {
//     return ServerStore.get("current");
// }

// export async function setCurrent(server: ServerType) {
//     await ServerStore.set("current", server);
// }

// function imgsearchOfficialValid() {
//     return !!ServerStore.get("imgsearch-official-token");
// }

