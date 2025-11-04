use crate::{BankRecord, BinYPBankRecord, TxtYPBankRecord, convertor::Message, error::Result};

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CsvYPBankRecord {
    data: Vec<Message>,
}

impl CsvYPBankRecord {
    pub fn from_read<R: std::io::Read>(r: &mut R) -> Result<Self> {
        let mut reader = csv::Reader::from_reader(r);
        let mut res = Self::new();
        for result in reader.deserialize() {
            let record: Message = result?;
            res.push(record);
        }

        Ok(res)
    }
    
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl BankRecord for CsvYPBankRecord {
    fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
        let mut writer = csv::Writer::from_writer(writer);

        for record in &self.data {
            if let Err(e) = writer.serialize(record) {
                return Err(e.into());
            }
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
}

impl From<BinYPBankRecord> for CsvYPBankRecord {
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
                description: mes.description.clone().replace("\"", ""),
            });
        }
        record
    }
}

impl From<TxtYPBankRecord> for CsvYPBankRecord {
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
                description: mes.description.clone().replace("\"", ""),
            });
        }
        record
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    static TEST_CSV: &str = "tests/data/examples_file/records_example.csv";
    static TEST_WRITE_CSV: &str = "records_example_write.csv";

    #[test]
    fn test_read() {
        let mut file = File::open(TEST_CSV).unwrap();
        let data = CsvYPBankRecord::from_read(&mut file).unwrap();
        assert!(data.len() > 0);
    }

    #[test]
    fn test_write() {
        let mut file = File::open(TEST_CSV).unwrap();
        let data = CsvYPBankRecord::from_read(&mut file).unwrap();

        let mut write_file = File::create(TEST_WRITE_CSV).unwrap();
        assert!(data.write_to(&mut write_file).is_ok());

        let mut file = File::open(TEST_WRITE_CSV).unwrap();
        let wr_data = CsvYPBankRecord::from_read(&mut file).unwrap();
        assert_eq!(data, wr_data);
    }
}
