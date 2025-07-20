use std::{
    io::{BufWriter, IntoInnerError},
    path::{Path, PathBuf},
};

use crate::{
    path_utils::{self, sign},
    uuid_utils,
};
use bytes::Bytes;
use fast_image_resize::{images::Image, IntoImageView, Resizer};
use image::{
    codecs::{jpeg, png, webp},
    ImageEncoder, ImageFormat,
};

use super::error::AppError;

const IMAGE_WIDTH: u32 = 512;

pub fn guess_format(buf: &[u8]) -> Result<image::ImageFormat, AppError> {
    Ok(image::guess_format(buf)?)
}
fn save_local(root: &Path, bs: &[u8], format: ImageFormat) -> Result<PathBuf, AppError> {
    let root_hex = sign(root.display().to_string().as_bytes());
    let mut p = path_utils::thumbnail_dir(&root_hex)?.join(format!(
        "{}.{}",
        uuid_utils::get(),
        format.extensions_str()[0]
    ));

    while p.exists() {
        p = path_utils::thumbnail_dir(&root_hex)?.join(format!(
            "{}.{}",
            uuid_utils::get(),
            format.extensions_str()[0]
        ));
    }

    std::fs::write(&p, bs)?;

    Ok(p)
}

pub fn remove_dir(root: &str) -> Result<(), AppError> {
    let root_hex = sign(root.as_bytes());

    path_utils::remove_thumbnail_dir(&root_hex)?;

    Ok(())
}

pub fn gen_thumbnail(root: &str, path: &Path) -> Result<(String, PathBuf), AppError> {
    let source_bs = std::fs::read(path)?;

    let sign = path_utils::sign(&source_bs);

    let format = guess_format(source_bs.as_slice())?;
    let bs = downscale(&source_bs, format)?;

    let thumbnail_path = match bs {
        Some(bs) => save_local(Path::new(root), bs.as_ref(), format)?,
        None => save_local(Path::new(root), &source_bs, format)?,
    };

    Ok((sign, thumbnail_path))
}

/**
 * 缩放图片，当传入图片的宽度小于512，则不进行缩放
 */
pub fn downscale(buf: &[u8], format: ImageFormat) -> Result<Option<Bytes>, AppError> {
    let src_image = image::load_from_memory_with_format(buf, format)?;

    if src_image.width() < IMAGE_WIDTH {
        return Ok(None);
    }

    let target_width = IMAGE_WIDTH;
    let target_height: u32 = IMAGE_WIDTH * src_image.height() / src_image.width();
    let pixel_type = src_image.pixel_type();
    if pixel_type.is_none() {
        return Err(AppError::ImgFormat("pixel_type is none".to_string()));
    }
    let mut dst_image = Image::new(target_width, target_height, pixel_type.unwrap());

    // Create Resizer instance and resize source image
    // into buffer of destination image
    let mut resizer = Resizer::new();
    resizer.resize(&src_image, &mut dst_image, None)?;

    // Write destination image as PNG-file
    let mut writer = BufWriter::new(Vec::new());
    match format {
        ImageFormat::Png => {
            png::PngEncoder::new(&mut writer)
                .write_image(
                    dst_image.buffer(),
                    target_width,
                    target_height,
                    src_image.color().into(),
                )
                .unwrap();
        }
        ImageFormat::Jpeg => {
            jpeg::JpegEncoder::new(&mut writer)
                .write_image(
                    dst_image.buffer(),
                    target_width,
                    target_height,
                    src_image.color().into(),
                )
                .unwrap();
        }
        ImageFormat::WebP => {
            webp::WebPEncoder::new_lossless(&mut writer)
                .write_image(
                    dst_image.buffer(),
                    target_width,
                    target_height,
                    src_image.color().into(),
                )
                .unwrap();
        }
        _ => {}
    };

    // 将 writer  写入到本地的image.png文件中

    let bs: Bytes = Bytes::from(writer.into_inner()?);

    Ok(Some(bs))
}

impl<T> From<IntoInnerError<T>> for AppError {
    fn from(e: IntoInnerError<T>) -> Self {
        AppError::Internal(format!("IntoInnerError: {:?}", e.error()))
    }
}
