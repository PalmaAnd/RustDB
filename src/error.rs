use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Page error: {0}")]
    Page(String),

    #[error("Storage error: {0}")]
    Storage(String),
}

pub type Result<T> = std::result::Result<T, DBError>;
