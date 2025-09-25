use actix_web::web::Data;

use crate::AppState;


/// Constructs a path relative to frontend url from `path` 
pub fn get_path(state: &Data<AppState>, path: &str) -> String {
    match state.env.static_frontend {
        true => path.to_string(),
        false => format!("{}{}", state.env.frontend_url, path),
    }
}
