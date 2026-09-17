//! Session based authorization based on principles from [lucia-auth](https://lucia-auth.com)

use actix_web::{cookie::Cookie, http::StatusCode};
use sha2::{Digest, Sha256};
use sqlx::{Result, SqlitePool};
use time::{Duration, OffsetDateTime};
use hex;

use crate::{database::{crud, model}, error::{AppError, AuthError, DatabaseError}, utils};

/// Name of the cookie holding the session token, in `"<id>.<secret>"` format.
pub const SESSION_COOKIE: &str = "session-token";

/// A persisted login session. Only `secret_hash` is stored/compared server-side;
/// the raw secret is only ever held in the client's cookie (see [`Token`]).
#[derive(sqlx::FromRow, serde::Serialize, Clone)]
pub struct Session {
    pub id: String,
    secret_hash: String,
    created_at: i64,
    pub user: u32
}

/// A parsed session cookie: a public `id` used to look up the [`Session`] row, and
/// a secret checked (as its SHA-256 hash) against `Session::secret_hash`.
pub struct Token {
    id: String,
    secret: String
}

/// State of a pending email-address change flow.
pub enum EmailSwitchState {
    /// The user has verified the switch and it is ready to complete, for the new email address.
    Authorized(String),
    /// A switch is in progress but not yet verified.
    Active,
    /// No email switch is in progress.
    Inactive
}

/// Extracts and splits the [`SESSION_COOKIE`] cookie's value (`"<id>.<secret>"`)
/// into a [`Token`]. Returns `None` if the cookie is absent or malformed.
pub fn parse_auth_cookie(cookie: Option<Cookie<'static>>) -> Option<Token> {
    if let Some(cookie) = cookie {
        let session_token = cookie.to_string();
        if let Some(token) = session_token.strip_prefix(&(SESSION_COOKIE.to_string() + "=")) {
            let (id, secret) = match token.splitn(2, '.').collect::<Vec<_>>().as_slice() {
                [id, secret] => (id.to_string(), secret.to_string()),
                _ => return None
            };
            return Some(Token { id, secret });
        }
    }
    None
}

/// Resolves the logged-in user from a session cookie: parses it, looks up the
/// session by its id, then fetches the associated user row.
///
/// Note this only checks that a non-expired session with the cookie's id exists;
/// unlike [`validate_session`] it does not verify the token's secret against
/// `Session::secret_hash`.
///
/// # Errors
/// Returns `400 Bad Request` if the cookie is missing/malformed, or
/// `401 Unauthorized` if the session doesn't exist or has expired.
pub async fn get_user_from_cookie(pool: &SqlitePool, cookie: Option<Cookie<'static>>) -> Result<model::UserRow, AppError> {
    let Some(token) = parse_auth_cookie(cookie) else {
        // Or should it return internal server error becuase authentication has already been done
        // by middleware?
        return Err(AuthError::new("Could not parse auth cookie")
            .with_status(StatusCode::BAD_REQUEST)
            .into()
        );
    };

    let session = get_session(pool, token.id).await.map_err(|err| DatabaseError::from(err))?;

    match session {
        Some(session) => {
            let id = session.user;
            let user = crud::get_user(pool, Some(id), None).await?;
            return Ok(user);
        },
        None => {
            return Err(
                AuthError::new("Session not found or expired")
                    .with_status(StatusCode::UNAUTHORIZED)
                    .into()
            );
        }
    }
}

/// Creates and persists a new [`Session`] for `user_id`, returning it along with
/// the raw cookie token (`"<id>.<secret>"`) — the only time the raw secret is
/// available, since only its hash is stored.
pub async fn create_session(pool: &SqlitePool, user_id: u32) -> Result<(Session, String), AppError> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let (id, secret) = match (utils::gen_secure_random_str(), utils::gen_secure_random_str()) {
        (Some(id), Some(secret)) => (id, secret),
        _ => {
            log::debug!("Could not generate random string for session creation");
            return Err(AuthError::new("Could not create session").into());
        }
    };
    let secret_hash = hex::encode(Sha256::digest(secret.clone()));

    let token = id.clone() + "." + &secret;

    let session = Session {
        id: id.clone(), secret_hash: secret_hash.clone(), created_at: now, user: user_id
    };
    
    sqlx::query("
        INSERT INTO Session (id, secret_hash, created_at, user)
        VALUES (?, ?, ?, ?)").bind(id).bind(secret_hash).bind(now).bind(user_id)
        .execute(pool).await.map_err(|err| DatabaseError::from(err))?;
    
    return Ok((session, token));
}

/// Looks up the session referenced by `token.id` and verifies its secret matches
/// (by comparing SHA-256 hashes). Returns `None` if the session doesn't exist,
/// has expired, or the secret doesn't match.
pub async fn validate_session(pool: &SqlitePool, token: Token) -> Result<Option<Session>, DatabaseError> {
    let session = get_session(pool, token.id).await?;

    if let Some(session) = session {
        let token_secret_hash = Sha256::digest(token.secret).to_vec();
        if let Ok(db_secret_hash) = hex::decode(session.secret_hash.clone()) {
            if eq_hashes(token_secret_hash, db_secret_hash) {
                return Ok(Some(session));
            }
        }
    }

    Ok(None)
}

/// Deletes `session` from the database, logging the user out.
pub async fn invalidate_session(pool: &SqlitePool, session: &Session) -> Result<(), DatabaseError> {
    sqlx::query("
        DELETE FROM Session
        WHERE id = ?").bind(session.id.clone()).execute(pool).await?;
    Ok(())
}

/// Fetches the session with `session_id`, if it exists and is younger than 7 days.
/// An expired session is deleted (via [`delete_session`]) and `None` is returned.
async fn get_session(pool: &SqlitePool, session_id: String) -> Result<Option<Session>, DatabaseError> {
    let now = OffsetDateTime::now_utc().unix_timestamp(); 
    
    let session: Option<Session> = sqlx::query_as("
        SELECT id, secret_hash, created_at, user
        FROM Session
        WHERE id = ?").bind(&session_id).fetch_optional(pool).await?;
    
    let Some(session) = session else {
        return Ok(None);
    };

    if now - session.created_at < Duration::days(7).whole_seconds() {
        return Ok(Some(session));
    } else {
        delete_session(pool, session_id).await?;
        return Ok(None);
    }
}

/// Deletes the session with `session_id` from the database.
async fn delete_session(pool: &SqlitePool, session_id: String) -> Result<(), DatabaseError> {
    sqlx::query("DELETE FROM Session WHERE id = ?").bind(session_id).execute(pool).await?;
    Ok(())
}

/// Byte-for-byte equality check between two hashes.
///
/// Note this is not constant-time, so it is theoretically susceptible to timing
/// attacks; it is used here to compare secret hashes rather than raw secrets.
fn eq_hashes(hash1: Vec<u8>, hash2: Vec<u8>) -> bool {
    if hash1.len() != hash2.len() {
        return false;
    }
    for i in 0..hash1.len() {
        if hash1[i] != hash2[i] {
            return false;
        }
    }
    return true;
}
