use thiserror::Error;

/// Result type alias
pub type Result<T> = std::result::Result<T, AppError>;

/// Application error type
#[derive(Debug, Error)]
pub enum AppError {
    /// I/O error
    #[error("I/O error: {0}")]
    IOError(#[from] std::io::Error),
    /// TXT parse error
    #[error("TXT parse error: {0}")]
    TxtParseError(String),
    /// CSV parse error
    #[error("CSV parse error: {0}")]
    CsvParseError(#[from] csv::Error),
    /// Bin parse error
    #[error("Bin parse error: {0}")]
    BinParseError(#[from] BinParseError),
    /// Bin parse error array
    #[error("Bin parse array error: {0}")]
    BinArrayError(#[from] std::array::TryFromSliceError),
    /// Bin parse error UTF-8
    #[error("Bin parse UTF-8 error: {0}")]
    BinUtf8Error(#[from] std::string::FromUtf8Error),
}

/// Bin parse error type
#[derive(Debug, Error)]
pub enum BinParseError {
    /// Bin parse magic number error
    #[error("Bin parse magic number error: {0}")]
    MagicNumberError(String),
    /// Bin parse version error
    #[error("Bin parse transaction error: {0}")]
    TransactionError(String),
    /// Bin parse description error
    #[error("Bin parse description error: {0}")]
    DescriptionError(String),
    /// Bin parse message size error
    #[error("Bin parse message size error: {0}")]
    MessageSizeError(String),
}
