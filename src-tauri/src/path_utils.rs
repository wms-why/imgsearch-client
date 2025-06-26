use std::path::PathBuf;

pub fn data_dir() -> PathBuf {
    let p = dirs::home_dir().unwrap().join(".imgsearch");
    std::fs::create_dir(&p);
    p
}

pub fn thumbnail_dir() -> PathBuf {
    let p = data_dir().join("thumbnails");
    std::fs::create_dir(&p);
    p
}

pub fn image_idx_file() -> PathBuf {
    let p = data_dir().join("image_idx.db");
    std::fs::create_dir(&p);
    p
}
