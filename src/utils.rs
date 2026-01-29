use std::{fs, io::BufReader};

use actix_multipart::form::tempfile::TempFile;
use actix_web::web::Data;
use image::{GenericImageView, ImageReader};

use crate::{AppError, AppState};


/// Constructs a path relative to frontend url from `path` 
pub fn get_path(state: &Data<AppState>, path: &str) -> String {
    match state.env.static_frontend {
        true => path.to_string(),
        false => format!("{}{}", state.env.frontend_url, path),
    }
}

pub fn read_to_string(path: &str) -> Result<String, std::io::Error> {
    return fs::read_to_string(path)
}

pub const IMG_DISK_SIZE: u32 = 512;
pub const IMG_DISK_PATH: &str = "./db/uploads/images/product/";

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

pub fn delete_img_from_disk(name: &str) -> Result<(), AppError> {
    match  fs::remove_file(format!("{IMG_DISK_PATH}{name}.webp")) {
        Ok(_) => Ok(()),
        Err(_) => Err(AppError::GenericError("Failed to delete file".to_string())),
    }
}