use axum::response::IntoResponse;
pub use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomErrors {
    #[error("unknown data store error")]
    Unknown,

    #[error("invalid data (expected {expected:?}, found {found:?})")]
    InvalidData { expected: String, found: String },
}

impl IntoResponse for CustomErrors {
    
}
