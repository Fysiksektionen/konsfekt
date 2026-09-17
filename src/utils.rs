//! Utility functions that could be useful throughout the codebase

use std::{fs, io::BufReader};

use rand::{rngs::OsRng, TryRngCore};
use actix_multipart::form::tempfile::TempFile;
use actix_web::web::Data;
use image::ImageReader;
use time::OffsetDateTime;

use crate::{AppState, error::GenericError};

// Human readable alphabet (a-z, 0-9 without l, o, 0, 1 to avoid confusion)
const READABLE_ALPHABET: &[u8] = b"abcdefghijkmnpqrstuvwxyz23456789";

/// Generates a 32-character cryptographically secure random string using a
/// human-readable alphabet (lowercase letters and digits, excluding easily
/// confused characters `l`, `o`, `0`, `1`). Used e.g. for session tokens and secrets.
///
/// Returns `None` if the OS RNG fails to provide randomness.
pub fn gen_secure_random_str() -> Option<String> {
    let mut rand_bytes = [0u8;32];
    OsRng.try_fill_bytes(&mut rand_bytes).ok()?;
    let mut result = String::new();
    for rand in rand_bytes {
        let i = (rand >> 3) as usize;
        result.push(READABLE_ALPHABET[i] as char);
    }
    return Some(result);
}

/// Constructs a path relative to frontend url from `path` 
pub fn get_path(state: &Data<AppState>, path: &str) -> String {
    match state.env.static_frontend {
        true => path.to_string(),
        false => format!("{}{}", state.env.frontend_url, path),
    }
}

/// Reads an entire file into a `String`. Thin wrapper around [`fs::read_to_string`].
pub fn read_to_string(path: &str) -> Result<String, std::io::Error> {
    return fs::read_to_string(path)
}

/// Width/height (in pixels) that product images are resized to before being saved.
pub const IMG_DISK_SIZE: u32 = 512;
/// Directory product images are stored in, keyed by product id/name.
pub const IMG_DISK_PATH: &str = "./db/uploads/images/product/";

/// Crops `img_file` to a centered square, resizes it to [`IMG_DISK_SIZE`] x
/// [`IMG_DISK_SIZE`], and saves it as `<IMG_DISK_PATH><name>.webp`.
///
/// Returns `None` if the upload can't be reopened, decoded as an image, or saved.
pub fn save_img_to_disk(img_file: TempFile, name: &str) -> Option<()> {
    let file = img_file.file.reopen().ok()?;
    let reader = BufReader::new(file);
    let img = ImageReader::new(reader).with_guessed_format().ok()?.decode().ok()?;

    // Crop a square centerd around the midde, with side = min(width, height)
    let side = std::cmp::min(img.width(), img.height());
    let mut x: u32 = 0; 
    let mut y: u32 = 0; 
    if img.width() > img.height() {
        x = (img.width() - side) / 2
    } else {
        y = (img.height() - side) / 2
    }

    let squared = img.crop_imm(x, y, side, side);
    let resized = squared.resize(IMG_DISK_SIZE, IMG_DISK_SIZE, image::imageops::FilterType::Triangle);

    resized.save(format!("{IMG_DISK_PATH}{}.webp", name)).ok()
}

/// Deletes the product image previously saved by [`save_img_to_disk`] for `name`.
pub fn delete_img_from_disk(name: &str) -> Result<(), GenericError> {
    match fs::remove_file(format!("{IMG_DISK_PATH}{name}.webp")) {
        Ok(_) => Ok(()),
        Err(_) => Err(GenericError::new("Failed to delete file")),
    }
}

/// Converts a Unix timestamp (`i64`, seconds since epoch) to [`OffsetDateTime`].
///
/// Falls back to [`OffsetDateTime::UNIX_EPOCH`] if `unix` is outside the valid range.
pub fn datetime_from_timestamp(unix: i64) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(unix).unwrap_or_else(|_| OffsetDateTime::UNIX_EPOCH)
}
