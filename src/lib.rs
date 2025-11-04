mod error;
use error::Result;
pub use error::AppError;

/// Модуль конвертор записей банковского счета, в нем реализованы 3 типа конвертора:
/// 1. Текстовый формат
/// 2. Бинарный формат
/// 3. CSV формат
/// И также 1 тип записи банковского счета
/// 
/// Реализовано преобразование каждого типа в другой.
mod convertor;
pub use convertor::{
    CsvYPBankRecord,
    BinYPBankRecord,
    TxtYPBankRecord,
    Message,
};

/// Трейт для работы с записями банковского счета
/// Реализовано методы:
/// 1. write_to - запись в файл
/// 2. push - добавление записи в конец
/// 3. len - количество записей
/// 4. pop - удаление последней записи
/// 5. iter - итератор по записям
pub trait BankRecord {
    fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<()>;
    fn push(&mut self, value: Message) where Self: Sized;
    fn len(&self) -> usize;
    fn pop(&mut self) -> Option<Message>;
    fn iter(&self) -> std::slice::Iter<'_, Message>;
}

/// Тип записи банковского счета для удобства
pub enum BankRecordEnum {
    TXT(TxtYPBankRecord),
    BIN(BinYPBankRecord),
    CSV(CsvYPBankRecord),
}

/// Реализация BankRecord для BankRecordEnum
impl BankRecord for BankRecordEnum {    
    /// Запись в файл
    fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        match self {
            BankRecordEnum::TXT(txt_ypbank_record) => txt_ypbank_record.write_to(writer),
            BankRecordEnum::BIN(bin_ypbank_record) => bin_ypbank_record.write_to(writer),
            BankRecordEnum::CSV(csv_ypbank_record) => csv_ypbank_record.write_to(writer),
        }
    }
    /// Добавление записи в конец
    fn push(&mut self, value: Message) where Self: Sized {
        match self {
            BankRecordEnum::TXT(txt_ypbank_record) => txt_ypbank_record.push(value),
            BankRecordEnum::BIN(bin_ypbank_record) => bin_ypbank_record.push(value),
            BankRecordEnum::CSV(csv_ypbank_record) => csv_ypbank_record.push(value),
        }
    }
    /// Количество записей
    fn len(&self) -> usize {
        match self {
            BankRecordEnum::TXT(txt_ypbank_record) => txt_ypbank_record.len(),
            BankRecordEnum::BIN(bin_ypbank_record) => bin_ypbank_record.len(),
            BankRecordEnum::CSV(csv_ypbank_record) => csv_ypbank_record.len(),
        }
    }
    /// Удаление последней записи
    fn pop(&mut self) -> Option<Message> {
         match self {
            BankRecordEnum::TXT(txt_ypbank_record) => txt_ypbank_record.pop(),
            BankRecordEnum::BIN(bin_ypbank_record) => bin_ypbank_record.pop(),
            BankRecordEnum::CSV(csv_ypbank_record) => csv_ypbank_record.pop(),
        }
    }
    /// Итератор по записям
    fn iter(&self) -> std::slice::Iter<'_, Message> {
        match self {
            BankRecordEnum::TXT(txt_ypbank_record) => txt_ypbank_record.iter(),
            BankRecordEnum::BIN(bin_ypbank_record) => bin_ypbank_record.iter(),
            BankRecordEnum::CSV(csv_ypbank_record) => csv_ypbank_record.iter(),
        }
    }
}

