use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("I/O error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("TXT parse error: {0}")]
    TxtParseError(String),

    #[error("CSV parse error: {0}")]
    CsvParseError(#[from] csv::Error),

    #[error("Bin parse error: {0}")]
    BinParseError(#[from] BinParseError),

    #[error("Bin parse array error: {0}")]
    BinArrayError(#[from] std::array::TryFromSliceError),

    #[error("Bin parse UTF-8 error: {0}")]
    BinUtf8Error(#[from] std::string::FromUtf8Error),
}

#[derive(Debug, Error)]
pub enum BinParseError {
    #[error("Bin parse magic number error: {0}")]
    MagicNumberError(String),

    #[error("Bin parse transaction error: {0}")]
    TransactionError(String),

    #[error("Bin parse description error: {0}")]
    DescriptionError(String),
}