use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};

use sha2::{Digest, Sha256};
use walkdir::WalkDir;

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

pub fn thumbnail_dir(dir: &str) -> Result<PathBuf, AppError> {
    let p = Path::new(THUMBNAIL_DIR).join(dir);
    other_dir(p.as_path().to_str().unwrap())
}

pub fn remove_thumbnail_dir(dir: &str) -> Result<(), AppError> {
    let p = Path::new(THUMBNAIL_DIR).join(dir);
    let p = data_dir()?.join(p);
    if p.exists() {
        log::debug!("remove_thumbnail_dir: {}", p.display());
        std::fs::remove_dir_all(&p)?;
    }
    Ok(())
}

pub fn lancedb_dir() -> Result<PathBuf, AppError> {
    other_dir(LANCEDB_DIR)
}

/**
 * 重命名文件
 * target_name: 新的文件名, 不包含后缀
 */
pub fn rename(current_path: &Path, target_name: &str) -> Result<PathBuf, AppError> {
    let ext = if let Some(ext) = current_path.extension() {
        format!(".{}", ext.to_str().unwrap())
    } else {
        "".to_string()
    };

    let parent = current_path.parent().unwrap();

    let new_path = gen_new_valid_path(parent, target_name, &ext);

    log::debug!(
        "success rename from {} to {}",
        current_path.display(),
        new_path.display()
    );

    std::fs::rename(current_path, &new_path)?;

    Ok(new_path)
}

fn gen_new_valid_path(parent: &Path, target_name: &str, ext: &str) -> PathBuf {
    let mut new_path = parent.join(format!("{target_name}{ext}"));
    let mut i = 1;
    while new_path.exists() {
        new_path = parent.join(format!("{target_name}_{i}{ext}"));

        i += 1;
    }

    new_path
}

// 文件签名

// pub fn sign_file(path: &Path) -> Result<String, AppError> {
//     let data = std::fs::read(path)?;
//     Ok(sign(&data))
// }

/**
 * 文件签名
 */
pub fn sign(data: &[u8]) -> String {
    // 计算哈希
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();

    hex::encode(hash)
}
static IMAGE_VALID_SUBFIX: OnceLock<Vec<&str>> = OnceLock::new();

fn get_valid_subfix() -> &'static Vec<&'static str> {
    IMAGE_VALID_SUBFIX.get_or_init(|| vec!["jpg", "jpeg", "png", "webp"])
}
pub fn find_all_images(path: &Path) -> Result<Vec<PathBuf>, AppError> {
    let subfix = get_valid_subfix();

    let mut images = vec![];

    for entry in WalkDir::new(path) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let ext = path.extension().and_then(|s| s.to_str());

            if let Some(ext) = ext {
                if subfix.contains(&ext) {
                    images.push(path.to_path_buf());
                }
            }
        }
    }
    Ok(images)
}
pub fn is_support_file(path: &Path) -> bool {
    if path.is_dir() {
        return false;
    }

    let subfix = get_valid_subfix();
    let ext = path.extension().and_then(|s| s.to_str());

    if let Some(ext) = ext {
        subfix.contains(&ext)
    } else {
        false
    }
}

impl From<walkdir::Error> for AppError {
    fn from(e: walkdir::Error) -> Self {
        AppError::Internal(e.to_string())
    }
}

pub fn remove_file(ab_path: &Path) -> Result<(), AppError> {
    std::fs::remove_file(ab_path)?;
    Ok(())
}
