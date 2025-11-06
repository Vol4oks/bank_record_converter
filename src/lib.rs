#![deny(unreachable_pub)]
#![warn(missing_docs)]

//! Модуль конвертации записей банковсковских транзакций, в нем реализованы 3 типа файла:
//! 1. Текстовый формат
//! 2. Бинарный формат
//! 3. CSV формат
//!
//! Реализовано преобразование каждого типа в другой.

/// Модуль ошибок
pub mod error;
use enum_display::EnumDisplay;
use error::Result;

mod convertor;
pub use convertor::Message;
use convertor::{BinYPBankRecord, CsvYPBankRecord, TxtYPBankRecord};

use crate::convertor::BankRecord;

/// Форматы данных
#[derive(Debug, EnumDisplay, Clone, Copy)]
pub enum DataFormat {
    /// Текстовый формат
    TXT,
    /// Бинарный формат
    BIN,
    /// CSV формат
    CSV,
}

/// Тип записи банковского счета
#[derive(Debug, PartialEq, EnumDisplay, Clone)]
pub enum BankRecordConvertor {
    /// Текстовый формат
    TXT(TxtYPBankRecord),
    /// Бинарный формат
    BIN(BinYPBankRecord),
    /// CSV формат
    CSV(CsvYPBankRecord),
}

/// Реализация BankRecord для BankRecordEnum
impl BankRecordConvertor {
    /// Создание из файла
    pub fn from_read<R: std::io::Read>(r: R, format: &DataFormat) -> Result<Self> {
        match format {
            DataFormat::TXT => {
                let record = TxtYPBankRecord::from_read(r)?;
                Ok(BankRecordConvertor::TXT(record))
            }
            DataFormat::BIN => {
                let record = BinYPBankRecord::from_read(r)?;
                Ok(BankRecordConvertor::BIN(record))
            }
            DataFormat::CSV => {
                let record = CsvYPBankRecord::from_read(r)?;
                Ok(BankRecordConvertor::CSV(record))
            }
        }
    }
    /// Конвертация в другой формат
    pub fn convert_to(self, dataformat: &DataFormat) -> Self {
        match (self, dataformat) {
            (Self::TXT(record), DataFormat::TXT) => Self::TXT(record),
            (Self::BIN(record), DataFormat::BIN) => Self::BIN(record),
            (Self::CSV(record), DataFormat::CSV) => Self::CSV(record),
            (Self::TXT(record), DataFormat::BIN) => Self::BIN(record.into()),
            (Self::TXT(record), DataFormat::CSV) => Self::CSV(record.into()),
            (Self::BIN(record), DataFormat::TXT) => Self::TXT(record.into()),
            (Self::BIN(record), DataFormat::CSV) => Self::CSV(record.into()),
            (Self::CSV(record), DataFormat::TXT) => Self::TXT(record.into()),
            (Self::CSV(record), DataFormat::BIN) => Self::BIN(record.into()),
        }
    }

    /// Запись в файл
    pub fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        match self {
            BankRecordConvertor::TXT(record) => record.write_to(writer),
            BankRecordConvertor::BIN(record) => record.write_to(writer),
            BankRecordConvertor::CSV(record) => record.write_to(writer),
        }
    }
    /// Добавление записи в конец
    pub fn push(&mut self, value: Message) {
        match self {
            BankRecordConvertor::TXT(record) => record.push(value),
            BankRecordConvertor::BIN(record) => record.push(value),
            BankRecordConvertor::CSV(record) => record.push(value),
        }
    }
    /// Количество записей
    pub fn len(&self) -> usize {
        match self {
            BankRecordConvertor::TXT(record) => record.len(),
            BankRecordConvertor::BIN(record) => record.len(),
            BankRecordConvertor::CSV(record) => record.len(),
        }
    }
    /// Проверка на пустоту
    pub fn is_empty(&self) -> bool {
        match self {
            BankRecordConvertor::TXT(record) => record.is_empty(),
            BankRecordConvertor::BIN(record) => record.is_empty(),
            BankRecordConvertor::CSV(record) => record.is_empty(),
        }
    }
    /// Удаление последней записи
    pub fn pop(&mut self) -> Option<Message> {
        match self {
            BankRecordConvertor::TXT(record) => record.pop(),
            BankRecordConvertor::BIN(record) => record.pop(),
            BankRecordConvertor::CSV(record) => record.pop(),
        }
    }
    /// Итератор по записям
    pub fn iter(&self) -> std::slice::Iter<'_, Message> {
        match self {
            BankRecordConvertor::TXT(record) => record.iter(),
            BankRecordConvertor::BIN(record) => record.iter(),
            BankRecordConvertor::CSV(record) => record.iter(),
        }
    }
}
