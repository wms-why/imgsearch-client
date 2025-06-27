use std::sync::{Arc, OnceLock};

use arrow_array::{types::Float64Type, ArrayRef, FixedSizeListArray, RecordBatch, RecordBatchIterator, StringArray};
use lancedb::{
    arrow::arrow_schema::{DataType, Field, Schema},
    error::Error,
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

pub async fn create_empty_table(db: &Connection) -> Result<Table, AppError> {
    let r = db
        .create_empty_table("img_idx", get_schema().clone())
        .execute()
        .await;

    match r {
        Ok(table) => {
            if table.index_stats("embedding").await?.is_none() {
                table
                    .create_index(
                        &["embedding"],
                        lancedb::index::Index::IvfPq(IvfPqIndexBuilder::default()),
                    )
                    .execute()
                    .await?;
            }
            Ok(table)
        }
        Err(e) => match e {
            Error::TableAlreadyExists { .. } => {
                return Ok(db.open_table("img_idx").execute().await?);
            }
            _ => return Err(e.into()),
        },
    }
}

pub async fn save_batch(table: &Table, records: Vec<ImgIdx>) -> Result<(), AppError> {
    let schema = get_schema().clone();

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
            .map(|ImgIdx { vec, .. }| Some(vec))
            .collect::<Vec<_>>(),
        DIM,
    );

    let batch = RecordBatch::try_new(schema, vec![
        Arc::new(name),
        Arc::new(path),
        Arc::new(root) ,
        Arc::new(desc) ,
        Arc::new(embedding),
    ])?;

    let batch = RecordBatchIterator::new(batch, schema);

    return table.add(batch).await;
}
