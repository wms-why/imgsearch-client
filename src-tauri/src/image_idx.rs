use std::sync::{Arc, OnceLock};

use arrow_array::{
    builder::{BooleanBuilder, FixedSizeListBuilder, Float32Builder, StringBuilder},
    Array, ArrayRef, RecordBatch, RecordBatchIterator,
};
use futures::TryStreamExt;
use lancedb::{
    arrow::{
        arrow_schema::{DataType, Field, Schema},
        IntoArrowStream,
    },
    index::vector::IvfPqIndexBuilder,
    query::{ExecutableQuery, QueryBase},
    Connection, Table,
};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Deserialize, Serialize)]
pub struct ImgIdx {
    pub name: String,
    pub path: String,
    pub root: String,
    pub thumbnail: String,
    // img indexed ready?
    pub idxed: bool,
    pub desc: Option<String>,
    pub vec: Option<Vec<f32>>,
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
            Field::new("thumbnail", DataType::Utf8, false),
            Field::new("idxed", DataType::Boolean, false),
            Field::new("desc", DataType::Utf8, true),
            Field::new(
                "embedding",
                DataType::FixedSizeList(Arc::new(Field::new("item", DataType::Float32, true)), DIM),
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
    let mut thumbnail_builder = StringBuilder::new();
    let mut idxed_builder = BooleanBuilder::new();
    let mut desc_builder = StringBuilder::new();
    let mut vec_builder = FixedSizeListBuilder::new(
        Float32Builder::with_capacity(DIM as usize * records.len()),
        DIM,
    );

    for ImgIdx {
        name,
        path,
        root,
        thumbnail,
        idxed,
        desc,
        vec,
    } in records.into_iter()
    {
        name_builder.append_value(name);
        path_builder.append_value(path);
        root_builder.append_value(root);
        thumbnail_builder.append_value(thumbnail);
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
            Arc::new(thumbnail_builder.finish()) as ArrayRef,
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

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct ImgSearchResult {
    pub name: String,
    pub path: String,
    pub root: String,
    pub thumbnail: String,
    pub desc: Option<String>,
    pub score: f32,
}

fn map_batch_to_imgsearchresult(batch: &RecordBatch) -> Result<Vec<ImgSearchResult>, AppError> {
    let name_array = batch
        .column_by_name("name")
        .ok_or_else(|| AppError::Internal("Missing column: name".to_string()))?
        .as_any()
        .downcast_ref::<arrow_array::StringArray>()
        .ok_or_else(|| AppError::Internal("Invalid type for column: name".to_string()))?;

    let path_array = batch
        .column_by_name("path")
        .ok_or_else(|| AppError::Internal("Missing column: path".to_string()))?
        .as_any()
        .downcast_ref::<arrow_array::StringArray>()
        .ok_or_else(|| AppError::Internal("Invalid type for column: path".to_string()))?;

    let root_array = batch
        .column_by_name("root")
        .ok_or_else(|| AppError::Internal("Missing column: root".to_string()))?
        .as_any()
        .downcast_ref::<arrow_array::StringArray>()
        .ok_or_else(|| AppError::Internal("Invalid type for column: root".to_string()))?;

    let thumbnail_array = batch
        .column_by_name("thumbnail")
        .ok_or_else(|| AppError::Internal("Missing column: thumbnail".to_string()))?
        .as_any()
        .downcast_ref::<arrow_array::StringArray>()
        .ok_or_else(|| AppError::Internal("Invalid type for column: thumbnail".to_string()))?;

    let desc_array = batch
        .column_by_name("desc")
        .ok_or_else(|| AppError::Internal("Missing column: desc".to_string()))?
        .as_any()
        .downcast_ref::<arrow_array::StringArray>()
        .ok_or_else(|| AppError::Internal("Invalid type for column: desc".to_string()))?;

    let score_array = batch
        .column_by_name("_distance")
        .or_else(|| batch.column_by_name("score"))
        .ok_or_else(|| AppError::Internal("Missing column: score or _distance".to_string()))?
        .as_any()
        .downcast_ref::<arrow_array::Float32Array>()
        .ok_or_else(|| AppError::Internal("Invalid type for column: score".to_string()))?;

    let mut res = Vec::with_capacity(batch.num_rows());

    for row in 0..batch.num_rows() {
        let name = name_array.value(row).to_string();
        let path = path_array.value(row).to_string();
        let root = root_array.value(row).to_string();
        let thumbnail = thumbnail_array.value(row).to_string();
        let desc = if desc_array.is_null(row) {
            None
        } else {
            Some(desc_array.value(row).to_string())
        };

        let score = score_array.value(row);

        res.push(ImgSearchResult {
            name,
            path,
            root,
            thumbnail,
            desc,
            score,
        });
    }

    Ok(res)
}

pub async fn search(
    table: Arc<Table>,
    v: &[f32],
    top: usize,
) -> Result<Vec<ImgSearchResult>, AppError> {
    let stream = table
        .vector_search(v)?
        .limit(top)
        .execute()
        .await?
        .into_arrow()?;

    let mut results = Vec::new();

    // 消费 stream
    futures::pin_mut!(stream);
    while let Some(batch) = stream.try_next().await? {
        let mut items = map_batch_to_imgsearchresult(&batch)?;
        results.append(&mut items);
    }
    Ok(results)
}
