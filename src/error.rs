use thiserror::Error;

#[derive(Debug, Error)]
pub enum SproutError {
    #[error(transparent)]
    Json(#[from] json::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
