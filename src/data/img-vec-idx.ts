import { Table, Index } from "@lancedb/lancedb";
import { getDB } from "./db"
import * as arrow from "apache-arrow";

const vector_size = 768;
const table_name = "img_idx";
let tbl: Table;
export interface ImgIdx {
    name: string;
    path: string;
    root: string;
    thumbnail: string;
    desc: string,
    vector: number[];

    [key: string]: any;
}

function getSchema() {
    return new arrow.Schema([
        new arrow.Field("name", new arrow.Utf8(), false),
        new arrow.Field("path", new arrow.Utf8(), false),
        new arrow.Field("root", new arrow.Utf8(), false),
        new arrow.Field("thumbnail", new arrow.Utf8(), true),
        new arrow.Field("desc", new arrow.Utf8(), false),
        new arrow.Field("vector", new arrow.List(new arrow.Field("item", new arrow.Float32(), false)), false),
    ]);
}

async function getTable() {
    if (!tbl) {
        const db = await getDB();
        const tbls = await db.tableNames();
        if (tbls.includes(table_name)) {
            tbl = await db.openTable(table_name);
        } else {
            tbl = await db.createEmptyTable(
                table_name,
                getSchema(),
                { mode: "create", existOk: true },
            );

            await tbl.createIndex("vector", {
                config: Index.ivfPq({
                    numPartitions: 128,
                    numSubVectors: 32,
                })
            });
        }
    }

    return tbl;

}

export async function saveBatch(records: ImgIdx[]) {
    const tbl = await getTable();
    await tbl.add(records);
}

export async function save(item: ImgIdx) {
    const tbl = await getTable();
    await tbl.add([item]);
}