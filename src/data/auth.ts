import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { LazyStore } from '@tauri-apps/plugin-store';
const AuthStore = new LazyStore('Auth.json');

interface LoginResp {
    token: string,
    user: Claims,
}

interface Claims {
    uid: number,
    username: string,
    email: string,
    picture: string | null,
    exp: number,
}

export async function getApikey(): Promise<string | undefined> {
    return await AuthStore.get('apikey');
}

export async function checkApiKey(apiKey: string): Promise<string | LoginResp> {

    const r = await fetch(`${process.env.NEXT_PUBLIC_IMGSEARCH_HOST}/api/check_apikey/v1`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            "Authorization": `Bearer ${apiKey}`,
        },
    });

    if (r.status != 200) {
        return await r.text();
    }

    return await r.json() as LoginResp;
}


export async function saveUserInfo(userInfo: LoginResp) {

    await AuthStore.set("apikey", userInfo.token);
    await AuthStore.set("user", userInfo);

    invoke("after_apikey_set");

}