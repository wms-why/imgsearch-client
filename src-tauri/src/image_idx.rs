use std::sync::{Arc, OnceLock};

use arrow_array::{
    builder::{BooleanBuilder, FixedSizeListBuilder, Float64Builder, StringBuilder},
    ArrayRef, RecordBatch, RecordBatchIterator,
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
    // img indexed ready?
    pub idxed: bool,
    pub desc: Option<String>,
    pub vec: Option<Vec<f64>>,
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
            Field::new("idxed", DataType::Boolean, false),
            Field::new("desc", DataType::Utf8, true),
            Field::new(
                "embedding",
                DataType::FixedSizeList(Arc::new(Field::new("item", DataType::Float64, true)), DIM),
                true,
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
    let mut name_builder = StringBuilder::new();
    let mut path_builder = StringBuilder::new();
    let mut root_builder = StringBuilder::new();
    let mut idxed_builder = BooleanBuilder::new();
    let mut desc_builder = StringBuilder::new();
    let mut vec_builder = FixedSizeListBuilder::new(
        Float64Builder::with_capacity(DIM as usize * records.len()),
        DIM,
    );

    for ImgIdx {
        name,
        path,
        root,
        idxed,
        desc,
        vec,
    } in records.into_iter()
    {
        name_builder.append_value(name);
        path_builder.append_value(path);
        root_builder.append_value(root);
        idxed_builder.append_value(idxed);
        desc_builder.append_option(desc);

        if let Some(vec) = vec {
            vec.into_iter().for_each(|f| {
                vec_builder.values().append_value(f);
            });
        } else {
            for _ in 0..DIM {
                vec_builder.values().append_null();
            }
        }
        vec_builder.append(true);
    }

    let schema = get_schema();

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(name_builder.finish()) as ArrayRef,
            Arc::new(path_builder.finish()) as ArrayRef,
            Arc::new(root_builder.finish()) as ArrayRef,
            Arc::new(idxed_builder.finish()) as ArrayRef,
            Arc::new(desc_builder.finish()) as ArrayRef,
            Arc::new(vec_builder.finish()) as ArrayRef,
        ],
    )?;

    let reader = RecordBatchIterator::new(vec![batch].into_iter().map(Ok), schema.clone());

    table.add(reader).execute().await?;

    let table = table.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = check_or_build_idx(table).await {
            log::error!("check_or_build_idx error: {e}");
        }
    });

    Ok(())
}
