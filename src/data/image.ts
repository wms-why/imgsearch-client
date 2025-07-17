import { invoke } from '@tauri-apps/api/core';

export interface SearchResult {
  name: string,
  path: string,
  root: string,
  thumbnail: string,
  idxed: boolean,
  desc: string | null,
  score: number,
}
export async function search(keyword: string, top: number) {
  return await invoke<SearchResult[]>("search", { model: { keyword, top } });
}

export async function getAll() {
  return await invoke<SearchResult[]>("show_all", {});
}