import { ServerType } from '@/servers';
import { LazyStore } from '@tauri-apps/plugin-store';

const ImgDirStore = new LazyStore('Server-Item.json');

export async function getCurrent(): Promise<ServerType> {
    return
}