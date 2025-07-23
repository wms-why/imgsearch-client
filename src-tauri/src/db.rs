use lancedb::Table;

use crate::{error::AppError, image_api::get_table, path_utils};

// 初始化 LanceDB，只调用一次
pub async fn init_db() -> Result<Table, AppError> {
    let db_path = path_utils::lancedb_dir()?;
    let db_path = db_path.to_str().unwrap();

    let db = lancedb::connect(db_path).execute().await?;

    let table = get_table(&db).await?;

    Ok(table)
}
