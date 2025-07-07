use std::path::{Path, PathBuf};

use crate::error::AppError;

static LOG_DIR: &str = "logs";
static THUMBNAIL_DIR: &str = "thumbnails";
static LANCEDB_DIR: &str = "db";

fn data_dir() -> Result<PathBuf, AppError> {
    let p = dirs::home_dir().unwrap().join(".imgsearch");
    if !p.exists() {
        std::fs::create_dir(&p)?;
    }
    Ok(p)
}

fn other_dir(name: &str) -> Result<PathBuf, AppError> {
    let p = data_dir()?.join(name);
    if !p.exists() {
        println!("create dir: {}", p.display());
        std::fs::create_dir(&p)?;
    }
    Ok(p)
}

pub fn logs_dir() -> Result<PathBuf, AppError> {
    other_dir(LOG_DIR)
}

pub fn thumbnail_dir() -> Result<PathBuf, AppError> {
    other_dir(THUMBNAIL_DIR)
}

pub fn lancedb_dir() -> Result<PathBuf, AppError> {
    other_dir(LANCEDB_DIR)
}

/**
 * 重命名文件
 * target_name: 新的文件名, 不包含后缀
 */
pub fn rename(current_path: &str, target_name: &str) -> Result<PathBuf, AppError> {
    let current_path = std::path::Path::new(current_path);

    let ext = if let Some(ext) = current_path.extension() {
        format!(".{}", ext.to_str().unwrap())
    } else {
        "".to_string()
    };

    let parent = current_path.parent().unwrap();

    let new_path = gen_new_valid_path(parent, target_name, &ext);

    std::fs::rename(current_path, &new_path)?;

    Ok(new_path)
}

fn gen_new_valid_path(parent: &Path, target_name: &str, ext: &str) -> PathBuf {
    let mut new_path = parent.join(target_name).join(ext);
    let mut i = 1;
    while new_path.exists() {
        new_path = parent.join(target_name).join(format!("_{i}")).join(ext);

        i += 1;
    }

    new_path
}
