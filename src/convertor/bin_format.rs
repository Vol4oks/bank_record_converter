use crate::{
    CsvYPBankRecord, TxtYPBankRecord,
    convertor::{BankRecord, Message, StatusTransaction, TypeTransaction},
    error::{AppError, BinParseError, Result},
};

const MAGIC_NUMBER: [u8; 4] = [0x59, 0x50, 0x42, 0x4E]; // "YPBN"

const MIN_SAIZE_MESSAGE: usize = 46;
const MAX_SAIZE_MESSAGE: usize = 1024; // TODO: уточнить максимальный размер записи

#[derive(Debug, PartialEq, Clone)]
pub struct BinYPBankRecord {
    data: Vec<Message>,
}

impl BinYPBankRecord {
    pub fn from_read<R: std::io::Read>(mut r: R) -> Result<Self> {
        let mut data = Self::new();
        loop {
            // Считываем магическое число
            let mut magic_buf = [0u8; 4];
            match r.read_exact(&mut magic_buf) {
                Ok(()) => {}
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    break;
                }
                Err(e) => {
                    return Err(AppError::BinParseError(BinParseError::MagicNumberError(
                        format!("Error parse magic number: {}", e),
                    )));
                }
            }

            if magic_buf != MAGIC_NUMBER {
                return Err(AppError::BinParseError(BinParseError::MagicNumberError(
                    format!(
                        "Invalid magic number: {:?}, expected: {:?}",
                        magic_buf, MAGIC_NUMBER
                    ),
                )));
            }

            // Считываем размер записи
            let mut record_size_buf = [0u8; 4];
            r.read_exact(&mut record_size_buf)?;
            // dbg!(record_size_buf);
            let record_size = u32::from_be_bytes(record_size_buf) as usize;
            // dbg!(record_size);
            // Проверям размер записи
            if record_size < MIN_SAIZE_MESSAGE {
                return Err(AppError::BinParseError(BinParseError::MessageSizeError(
                    format!(
                        "Message size is less than {} bytes: {}",
                        MIN_SAIZE_MESSAGE, record_size
                    ),
                )));
            }
            if record_size > MAX_SAIZE_MESSAGE {
                return Err(AppError::BinParseError(BinParseError::MessageSizeError(
                    format!(
                        "Message size is more than {} bytes: {}",
                        MAX_SAIZE_MESSAGE, record_size
                    ),
                )));
            }
            // Считываем саму запись
            let mut record_buf = vec![0u8; record_size];
            r.read_exact(&mut record_buf)?;
            // dbg!(&record_buf.len());
            let message = Message::parse_from_bin(&record_buf)?;
            data.push(message);
        }

        Ok(data)
    }

    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl BankRecord for BinYPBankRecord {
    fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        for message in &self.data {
            //Серелизируем тело сообщения
            let body_data = message.to_bin()?;
            let record_syze = body_data.len() as u32;

            // Записываем заголовок
            writer.write_all(&MAGIC_NUMBER)?;
            writer.write_all(&record_syze.to_be_bytes())?;

            // Записываем тело сообщения
            writer.write_all(&body_data)?;
        }

        Ok(())
    }

    fn push(&mut self, value: Message) {
        self.data.push(value)
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

impl From<CsvYPBankRecord> for BinYPBankRecord {
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

impl From<TxtYPBankRecord> for BinYPBankRecord {
    fn from(value: TxtYPBankRecord) -> Self {
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

impl Message {
    fn parse_from_bin(buf: &[u8]) -> Result<Self> {
        if buf.len() < MIN_SAIZE_MESSAGE {
            return Err(AppError::BinParseError(BinParseError::MessageSizeError(
                format!(
                    "Message size is less than {} bytes: {}",
                    MIN_SAIZE_MESSAGE,
                    buf.len()
                ),
            )));
        }
        // dbg!(buf.len());
        let mut cursor = 0;

        //TX_ID 8 bytes
        let tx_id = u64::from_be_bytes(buf[cursor..cursor + 8].try_into()?);
        cursor += 8;

        // TX_TYPE 1 byte
        let tx_type = TypeTransaction::from_u8(buf[cursor])?;
        cursor += 1;

        // FROM_USER_ID 8 bytes
        let from_user_id = u64::from_be_bytes(buf[cursor..cursor + 8].try_into()?);
        cursor += 8;

        // TO_USER_ID 8 bytes
        let to_user_id = u64::from_be_bytes(buf[cursor..cursor + 8].try_into()?);
        cursor += 8;

        // AMOUNT 8 bytes
        let amount = u64::from_be_bytes(buf[cursor..cursor + 8].try_into()?);
        cursor += 8;

        // TIMESTAMP 8 bytes
        let timestamp = u64::from_be_bytes(buf[cursor..cursor + 8].try_into()?);
        cursor += 8;

        // STATUS 1 byte
        let status = StatusTransaction::from_u8(buf[cursor])?;
        cursor += 1;

        // DESCRIPTION_LEN 4 bytes
        let desc_len = u32::from_be_bytes(buf[cursor..cursor + 4].try_into()?);
        cursor += 4;

        // DESCRIPTION DESCRIPTION_LEN bytes, UTF-8
        let description = if desc_len > 0 {
            if cursor + desc_len as usize > buf.len() {
                return Err(AppError::BinParseError(BinParseError::DescriptionError(
                    "Description length is greater than buffer length".to_string(),
                )));
            }

            String::from_utf8(buf[cursor..cursor + desc_len as usize].to_vec())?
        } else {
            String::new()
        };
        let message = Message {
            tx_id,
            tx_type,
            from_user_id,
            to_user_id,
            amount,
            timestamp,
            status,
            description,
        };
        // dbg!(&message);
        Ok(message)
    }

    fn to_bin(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();

        //TX_ID 8 bytes
        buf.extend_from_slice(&self.tx_id.to_be_bytes());

        // TX_TYPE 1 byte
        buf.push(self.tx_type.to_u8());

        // FROM_USER_ID 8 bytes
        buf.extend_from_slice(&self.from_user_id.to_be_bytes());

        // TO_USER_ID 8 bytes
        buf.extend_from_slice(&self.to_user_id.to_be_bytes());

        // AMOUNT 8 bytes
        buf.extend_from_slice(&self.amount.to_be_bytes());

        // TIMESTAMP 8 bytes
        buf.extend_from_slice(&self.timestamp.to_be_bytes());

        // STATUS 1 byte
        buf.push(self.status.to_u8());

        // DESCRIPTION_LEN 4 bytes
        let desc_len = self.description.len() as u32;
        buf.extend_from_slice(&desc_len.to_be_bytes());

        // DESCRIPTION DESCRIPTION_LEN bytes, UTF-8
        buf.extend_from_slice(self.description.as_bytes());
        // dbg!(&buf.len());
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    static TEST_BIN: &str = "tests/data/examples_file/records_example.bin";
    static TEST_WRITE_BIN: &str = "records_example_write.bin";

    #[test]
    fn test_read() {
        let mut file = File::open(TEST_BIN).unwrap();
        let data = BinYPBankRecord::from_read(&mut file).unwrap();
        assert!(data.len() > 0);
    }

    #[test]
    fn test_write() {
        let mut file = File::open(TEST_BIN).unwrap();
        let data = BinYPBankRecord::from_read(&mut file).unwrap();

        let mut write_file = File::create(TEST_WRITE_BIN).unwrap();
        assert!(data.write_to(&mut write_file).is_ok());

        let mut file = File::open(TEST_WRITE_BIN).unwrap();
        let wr_data = BinYPBankRecord::from_read(&mut file).unwrap();
        assert_eq!(data, wr_data);
    }
}
