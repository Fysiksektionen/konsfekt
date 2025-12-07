use std::{fs, io::BufReader};

use actix_multipart::form::tempfile::TempFile;
use actix_web::web::Data;
use image::ImageReader;

use crate::AppState;


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

pub fn save_img_to_disk(img_file: TempFile, name: &str) -> Option<()> {
    let file = img_file.file.reopen().ok()?;
    let reader = BufReader::new(file);
    let img = ImageReader::new(reader).with_guessed_format().ok()?.decode().ok()?;

    // TODO Image resizing and compression
    
    img.save(format!("./db/uploads/images/product/{}.webp", name)).ok()
}
