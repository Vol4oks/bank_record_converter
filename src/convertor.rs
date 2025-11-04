pub mod bin_format;
pub mod csv_format;
pub mod txt_format;

use enum_display::EnumDisplay;

use crate::error::{AppError, BinParseError, Result};

pub use {bin_format::BinYPBankRecord, csv_format::CsvYPBankRecord, txt_format::TxtYPBankRecord};

/// Тип транзакции
#[derive(Debug, Clone, Copy, PartialEq, EnumDisplay, serde::Deserialize, serde::Serialize)]
pub enum TypeTransaction {
    DEPOSIT,
    TRANSFER,
    WITHDRAWAL,
    None,
}

/// Статус транзакции
#[derive(Debug, Clone, Copy, PartialEq, EnumDisplay, serde::Deserialize, serde::Serialize)]
pub enum StatusTransaction {
    SUCCESS,
    FAILURE,
    PENDING,
    None,
}

impl TypeTransaction {
    fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(TypeTransaction::DEPOSIT),
            1 => Ok(TypeTransaction::TRANSFER),
            2 => Ok(TypeTransaction::WITHDRAWAL),
            _ => Err(AppError::BinParseError(BinParseError::TransactionError(
                "Invalid transaction type".to_string(),
            ))),
        }
    }

    fn to_u8(&self) -> u8 {
        match self {
            TypeTransaction::DEPOSIT => 0,
            TypeTransaction::TRANSFER => 1,
            TypeTransaction::WITHDRAWAL => 2,
            TypeTransaction::None => 3,
        }
    }
}

impl StatusTransaction {
    fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(StatusTransaction::SUCCESS),
            1 => Ok(StatusTransaction::FAILURE),
            2 => Ok(StatusTransaction::PENDING),
            _ => Err(AppError::BinParseError(BinParseError::TransactionError(
                "Invalid transaction status".to_string(),
            ))),
        }
    }

    fn to_u8(&self) -> u8 {
        match self {
            StatusTransaction::SUCCESS => 0,
            StatusTransaction::FAILURE => 1,
            StatusTransaction::PENDING => 2,
            StatusTransaction::None => 3,
        }
    }
}

/// Структура сообщения
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Message {
    /// TX_ID
    #[serde(rename = "TX_ID")]
    pub tx_id: u64,
    
    /// TX_TYPE
    #[serde(rename = "TX_TYPE")]
    pub tx_type: TypeTransaction,
    
    /// FROM_USER_ID
    #[serde(rename = "FROM_USER_ID")]
    pub from_user_id: u64,
    
    /// TO_USER_ID
    #[serde(rename = "TO_USER_ID")]
    pub to_user_id: u64,
    
    /// AMOUNT
    #[serde(rename = "AMOUNT")]
    pub amount: u64,
    
    /// TIMESTAMP
    #[serde(rename = "TIMESTAMP")]
    pub timestamp: u64,
    
    /// STATUS
    #[serde(rename = "STATUS")]
    pub status: StatusTransaction,
    
    /// DESCRIPTION
    #[serde(rename = "DESCRIPTION")]
    pub description: String,
}