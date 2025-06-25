import { LazyStore } from '@tauri-apps/plugin-store';

const GuideStore = new LazyStore('Guide.json');

export type GuideType = undefined | 'auth' | 'imgdir' | 'finished';
export async function next(): Promise<GuideType> {
    const guide = await GuideStore.get("current");
    if (!guide) {
        GuideStore.set("current", "auth");
        GuideStore.save();
        return "auth";
    }

    if (guide == "auth") {
        GuideStore.set("current", "imgdir");
        GuideStore.save();
        return "imgdir";
    }

    if (guide == "imgdir") {
        GuideStore.set("current", "finished");
        GuideStore.save();
        return "finished";
    }

    if (guide == "finished") {
        return "finished";
    }
}


export async function get(): Promise<GuideType> {
    return await GuideStore.get("current");
}
