use std::sync::{Arc, OnceLock};

use arrow_array::{
    types::Float64Type, ArrayRef, FixedSizeListArray, RecordBatch, RecordBatchIterator, StringArray,
};
use lancedb::{
    arrow::arrow_schema::{DataType, Field, Schema},
    index::vector::IvfPqIndexBuilder,
    Connection, Table,
};
use serde::Deserialize;

use crate::error::AppError;

#[derive(Clone, Deserialize, Debug)]
pub struct ImgIdx {
    pub name: String,
    pub path: String,
    pub root: String,
    pub desc: String,
    pub vec: Vec<f64>,
}

static SCHEMA: OnceLock<Arc<Schema>> = OnceLock::new();
static DIM: i32 = 768;
static IMG_IDX_TABLE_NAME: &str = "img_idx";
static IMG_IDX_BUILD_DIVIDER: usize = 256;
fn get_schema() -> &'static Arc<Schema> {
    SCHEMA.get_or_init(|| {
        Arc::new(Schema::new(vec![
            Field::new("name", DataType::Utf8, false),
            Field::new("path", DataType::Utf8, false),
            Field::new("root", DataType::Utf8, false),
            Field::new("desc", DataType::Utf8, false),
            Field::new(
                "embedding",
                DataType::FixedSizeList(Arc::new(Field::new("item", DataType::Float32, true)), DIM),
                false,
            ),
        ]))
    })
}

pub async fn get_table(db: &Connection) -> Result<Table, AppError> {
    let tbls = db.table_names().execute().await?;
    if tbls.contains(&IMG_IDX_TABLE_NAME.to_string()) {
        return Ok(db.open_table(IMG_IDX_TABLE_NAME).execute().await?);
    }

    let r = db
        .create_empty_table(IMG_IDX_TABLE_NAME, get_schema().clone())
        .execute()
        .await?;

    Ok(r)
}

async fn check_or_build_idx(table: Arc<Table>) -> Result<(), AppError> {
    let count = table.count_rows(None).await?;

    if count >= IMG_IDX_BUILD_DIVIDER && table.index_stats("embedding").await?.is_none() {
        table
            .create_index(
                &["embedding"],
                lancedb::index::Index::IvfPq(IvfPqIndexBuilder::default()),
            )
            .execute()
            .await?;
    }
    Ok(())
}

pub async fn save_batch(table: Arc<Table>, records: Vec<ImgIdx>) -> Result<(), AppError> {
    let name = StringArray::from_iter_values(
        records
            .iter()
            .map(|ImgIdx { name, .. }| name)
            .collect::<Vec<_>>(),
    );

    let path = StringArray::from_iter_values(
        records
            .iter()
            .map(|ImgIdx { path, .. }| path)
            .collect::<Vec<_>>(),
    );

    let root = StringArray::from_iter_values(
        records
            .iter()
            .map(|ImgIdx { root, .. }| root)
            .collect::<Vec<_>>(),
    );

    let desc = StringArray::from_iter_values(
        records
            .iter()
            .map(|ImgIdx { desc, .. }| desc)
            .collect::<Vec<_>>(),
    );

    let embedding = FixedSizeListArray::from_iter_primitive::<Float64Type, _, _>(
        records
            .into_iter()
            .map(|ImgIdx { vec, .. }| {
                Some(vec.into_iter().map(Some).collect::<Vec<_>>())
                // Some(vec)
            })
            .collect::<Vec<_>>(),
        DIM,
    );

    let schema = get_schema();

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(name) as ArrayRef,
            Arc::new(path) as ArrayRef,
            Arc::new(root) as ArrayRef,
            Arc::new(desc) as ArrayRef,
            Arc::new(embedding) as ArrayRef,
        ],
    )?;

    let reader = RecordBatchIterator::new(vec![batch].into_iter().map(Ok), schema.clone());

    table.add(reader).execute().await?;

    let table = table.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = check_or_build_idx(table).await {
            log::error!("check_or_build_idx error: {}", e);
        }
    });

    Ok(())
}
