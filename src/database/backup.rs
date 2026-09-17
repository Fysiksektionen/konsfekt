//! Database and uploads backup creation.

use std::{format, path::PathBuf};

use time::{OffsetDateTime, macros::format_description};

use crate::error::{AppError, DatabaseError, GenericError};

/// Writes a timestamped copy of the database to `backup/backup-database_<timestamp>.db`
/// via SQLite's `VACUUM INTO`.
///
/// Returns `(database_backup_path, uploads_backup_path)`; uploads backup is not yet
/// implemented, so the second value is always the placeholder string `"Not yet implemented."`.
pub async fn create_backup(pool: &sqlx::SqlitePool) -> Result<(String, String), AppError> {
    let timestamp = OffsetDateTime::now_utc()
    .format(&format_description!("[year]-[month]-[day]_[hour]-[minute]-[second]"))
    .map_err(|_| GenericError::new("Failed to create timestamp"))?;

    let backup_dir = PathBuf::from("backup");
    let path = backup_dir.join(format!("backup-database_{}.db", timestamp));

    // TODO: read uploads folder and compress it?

    sqlx::query(r#"VACUUM INTO ?"#)
        .bind(path.to_string_lossy().to_string())
        .execute(pool)
        .await
        .map_err(DatabaseError::from)?;

    Ok((path.to_string_lossy().to_string(), String::from("Not yet implemented.")))
}