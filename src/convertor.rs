mod bin_format;
mod csv_format;
mod txt_format;

use enum_display::EnumDisplay;

use crate::error::{AppError, BinParseError, Result};

pub(crate) use {
    bin_format::BinYPBankRecord, csv_format::CsvYPBankRecord, txt_format::TxtYPBankRecord,
};

pub(crate) trait BankRecord {
    fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<()>;
    fn push(&mut self, value: Message);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn pop(&mut self) -> Option<Message>;
    fn iter(&self) -> std::slice::Iter<'_, Message>;
}

/// Тип транзакции
#[derive(Debug, Clone, Copy, PartialEq, EnumDisplay, serde::Deserialize, serde::Serialize)]
#[enum_display(case = "Upper")]
pub enum TypeTransaction {
    #[serde(rename = "DEPOSIT")]
    Deposit,
    #[serde(rename = "TRANSFER")]
    Transfer,
    #[serde(rename = "WITHDRAWAL")]
    Withdrawal,
}

/// Статус транзакции
#[derive(Debug, Clone, Copy, PartialEq, EnumDisplay, serde::Deserialize, serde::Serialize)]
#[enum_display(case = "Upper")]
pub enum StatusTransaction {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
    #[serde(rename = "PENDING")]
    Pending,
}

impl TypeTransaction {
    fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(TypeTransaction::Deposit),
            1 => Ok(TypeTransaction::Transfer),
            2 => Ok(TypeTransaction::Withdrawal),
            _ => Err(AppError::BinParseError(BinParseError::TransactionError(
                "Invalid transaction type".to_string(),
            ))),
        }
    }

    fn to_u8(self) -> u8 {
        match self {
            TypeTransaction::Deposit => 0,
            TypeTransaction::Transfer => 1,
            TypeTransaction::Withdrawal => 2,
        }
    }
}

impl StatusTransaction {
    fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(StatusTransaction::Success),
            1 => Ok(StatusTransaction::Failure),
            2 => Ok(StatusTransaction::Pending),
            _ => Err(AppError::BinParseError(BinParseError::TransactionError(
                "Invalid transaction status".to_string(),
            ))),
        }
    }

    fn to_u8(self) -> u8 {
        match self {
            StatusTransaction::Success => 0,
            StatusTransaction::Failure => 1,
            StatusTransaction::Pending => 2,
        }
    }
}

/// Структура сообщения
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, Clone)]
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
