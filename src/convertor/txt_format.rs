use std::{fmt::Display, io::BufRead};

use crate::{
    BinYPBankRecord, CsvYPBankRecord,
    convertor::{BankRecord, Message, StatusTransaction, TypeTransaction},
    error::{AppError, Result},
};

#[derive(Debug, PartialEq, Clone)]
pub struct TxtYPBankRecord {
    data: Vec<Message>,
}

impl TxtYPBankRecord {
    pub fn from_read<R: std::io::Read>(r: R) -> Result<Self> {
        let reader = std::io::BufReader::new(r);
        let mut block = Vec::new();
        let mut parse_data = Vec::new();

        for line in reader.lines() {
            match line {
                Ok(l) => {
                    if l.trim().is_empty() {
                        if !block.is_empty() {
                            parse_data.push(Message::from_str(block)?);
                            block = Vec::new();
                        }
                    } else {
                        block.push(l);
                    }
                }
                Err(e) => {
                    return Err(AppError::TxtParseError(e.to_string()));
                }
            }
        }

        Ok(Self { data: parse_data })
    }

    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl Default for TxtYPBankRecord {
    fn default() -> Self {
        Self::new()
    }
}

impl BankRecord for TxtYPBankRecord {
    fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        for message in &self.data {
            let mes = message.to_string();
            // dbg!(mes.len());
            write!(writer, "{}", mes)?;
        }
        Ok(())
    }

    fn push(&mut self, value: Message) {
        self.data.push(value);
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn pop(&mut self) -> Option<Message> {
        self.data.pop()
    }

    fn iter(&self) -> std::slice::Iter<'_, Message> {
        self.data.iter()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl From<BinYPBankRecord> for TxtYPBankRecord {
    fn from(value: BinYPBankRecord) -> Self {
        let mut record = Self::new();
        for mes in value.iter() {
            record.push(Message {
                tx_id: mes.tx_id,
                tx_type: mes.tx_type,
                from_user_id: mes.from_user_id,
                to_user_id: mes.to_user_id,
                amount: mes.amount,
                timestamp: mes.timestamp,
                status: mes.status,
                description: mes.description.clone(),
            });
        }
        record
    }
}

impl From<CsvYPBankRecord> for TxtYPBankRecord {
    fn from(value: CsvYPBankRecord) -> Self {
        let mut record = Self::new();
        for mes in value.iter() {
            record.push(Message {
                tx_id: mes.tx_id,
                tx_type: mes.tx_type,
                from_user_id: mes.from_user_id,
                to_user_id: mes.to_user_id,
                amount: mes.amount,
                timestamp: mes.timestamp,
                status: mes.status,
                description: format!("\"{}\"", mes.description.clone()),
            });
        }
        record
    }
}

impl Message {
    fn from_str(lines: Vec<String>) -> Result<Self> {
        let mut tx_id = 0;
        let mut tx_type = TypeTransaction::Transfer;
        let mut from_user_id = 0;
        let mut to_user_id = 0;
        let mut amount = 0;
        let mut timestamp = 0;
        let mut status = StatusTransaction::Failure;
        let mut description = String::new();

        for line in lines {
            match line {
                // пропускаем комментарий
                l if l.starts_with("#") => continue,

                // неотрицательное целое число, идентифицирующее транзакцию
                l if l.starts_with("TX_ID") => {
                    if let Some(num) = l.split(": ").nth(1)
                        && let Ok(num) = num.parse::<u64>()
                    {
                        tx_id = num;
                        continue;
                    }
                    return Err(AppError::TxtParseError(l.to_string()));
                }

                //  тип транзакции
                l if l.starts_with("TX_TYPE") => match l.split(": ").nth(1) {
                    Some("DEPOSIT") => tx_type = TypeTransaction::Deposit,
                    Some("TRANSFER") => tx_type = TypeTransaction::Transfer,
                    Some("WITHDRAWAL") => tx_type = TypeTransaction::Withdrawal,
                    _ => return Err(AppError::TxtParseError(l.to_string())),
                },

                //  неотрицательное целое число, идентифицирующее отправитель счета (0 для Deposit).
                l if l.starts_with("FROM_USER_ID") => {
                    if let Some(num) = l.split(": ").nth(1)
                        && let Ok(num) = num.parse::<u64>()
                    {
                        from_user_id = num;
                        continue;
                    }
                    return Err(AppError::TxtParseError(l.to_string()));
                }

                // неотрицательное целое число, идентифицирующее получателя счета (0 для Withdrawal)
                l if l.starts_with("TO_USER_ID") => {
                    if let Some(num) = l.split(": ").nth(1)
                        && let Ok(num) = num.parse::<u64>()
                    {
                        to_user_id = num;
                        continue;
                    }
                    return Err(AppError::TxtParseError(l.to_string()));
                }

                // неотрицательное целое число, представляющее сумму в наименьшей единице валюты.
                l if l.starts_with("AMOUNT") => {
                    if let Some(num) = l.split(": ").nth(1)
                        && let Ok(num) = num.parse::<u64>()
                    {
                        amount = num;
                        continue;
                    }

                    return Err(AppError::TxtParseError(l.to_string()));
                }

                // Unix epoch timestamp в миллисекундах
                l if l.starts_with("TIMESTAMP") => {
                    if let Some(num) = l.split(": ").nth(1)
                        && let Ok(num) = num.parse::<u64>()
                    {
                        timestamp = num;
                        continue;
                    }
                    return Err(AppError::TxtParseError(l.to_string()));
                }

                //состояние транзакции
                l if l.starts_with("STATUS") => match l.split(": ").nth(1) {
                    Some("SUCCESS") => status = StatusTransaction::Success,
                    Some("FAILURE") => status = StatusTransaction::Failure,
                    Some("PENDING") => status = StatusTransaction::Pending,
                    _ => return Err(AppError::TxtParseError(l.to_string())),
                },

                // произвольное текстовое описание, UTF-8 в двойныхкавычках
                l if l.starts_with("DESCRIPTION") => {
                    if let Some(desc) = l.split(": ").nth(1) {
                        description = desc.to_string();
                        continue;
                    }
                    return Err(AppError::TxtParseError(l.to_string()));
                }

                // строки, которые не подошди под формат
                l => {
                    dbg!(l);
                    continue;
                }
            }
        }

        Ok(Message {
            tx_id,
            tx_type,
            from_user_id,
            to_user_id,
            amount,
            timestamp,
            status,
            description,
        })
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TX_ID: {}\nTX_TYPE: {}\nFROM_USER_ID: {}\nTO_USER_ID: {}\nAMOUNT: {}\nTIMESTAMP: {}\nSTATUS: {}\nDESCRIPTION: {}\n\n",
            self.tx_id,
            self.tx_type,
            self.from_user_id,
            self.to_user_id,
            self.amount,
            self.timestamp,
            self.status,
            self.description,
        )
    }
}
#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    static TEST_TXT: &str = "tests/data/examples_file/records_example.txt";
    static TEST_WRITE_TXT: &str = "records_example_write.txt";

    #[test]
    fn test_read() {
        let mut file = File::open(TEST_TXT).unwrap();
        let data = TxtYPBankRecord::from_read(&mut file).unwrap();
        assert!(data.len() > 0);
    }

    #[test]
    fn test_write() {
        let mut file = File::open(TEST_TXT).unwrap();
        let data = TxtYPBankRecord::from_read(&mut file).unwrap();

        let mut write_file = File::create(TEST_WRITE_TXT).unwrap();
        assert!(data.write_to(&mut write_file).is_ok());

        let mut file = File::open(TEST_WRITE_TXT).unwrap();
        let wr_data = TxtYPBankRecord::from_read(&mut file).unwrap();
        assert_eq!(data, wr_data);
    }
}
