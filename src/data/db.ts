import * as lancedb from "@lancedb/lancedb";
import { getDBDir } from "./file-tools";

let db: lancedb.Connection;

export async function getDB() {
  if (!db) {
    db = await lancedb.connect(await getDBDir());
  }

  return db;
}