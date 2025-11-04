use pretty_assertions::assert_eq;


use std::fs::File;

use bank_record_converter::{BinYPBankRecord, CsvYPBankRecord, TxtYPBankRecord};

static TEST_CSV: &str = "tests/data/examples_file/records_example.csv";
static TEST_BIN: &str = "tests/data/examples_file/records_example.bin";
static TEST_TXT: &str = "tests/data/examples_file/records_example.txt";

#[test]
fn test_convert_csv_to_bin() {
    let mut file = File::open(TEST_CSV).unwrap();
    let data = CsvYPBankRecord::from_read(&mut file).unwrap();
    let new_data: BinYPBankRecord = data.into();

    let mut file = File::open(TEST_BIN).unwrap();
    let test_data = BinYPBankRecord::from_read(&mut file).unwrap();

    assert_eq!(new_data, test_data);
}

#[test]
fn test_convert_csv_to_txt() {
    let mut file = File::open(TEST_CSV).unwrap();
    let data = CsvYPBankRecord::from_read(&mut file).unwrap();
    let new_data: TxtYPBankRecord = data.into();

    let mut file = File::open(TEST_TXT).unwrap();
    let test_data = TxtYPBankRecord::from_read(&mut file).unwrap();


    assert_eq!(new_data, test_data);
}

#[test]
fn test_convert_txt_to_bin() {
    let mut file = File::open(TEST_TXT).unwrap();
    let data = TxtYPBankRecord::from_read(&mut file).unwrap();
    let new_data: BinYPBankRecord = data.into();

    let mut file = File::open(TEST_BIN).unwrap();
    let test_data = BinYPBankRecord::from_read(&mut file).unwrap();

    assert_eq!(new_data, test_data);
}

#[test]
fn test_convert_txt_to_csv() {
    let mut file = File::open(TEST_TXT).unwrap();
    let data = TxtYPBankRecord::from_read(&mut file).unwrap();
    let new_data: CsvYPBankRecord = data.into();

    let mut file = File::open(TEST_CSV).unwrap();
    let test_data = CsvYPBankRecord::from_read(&mut file).unwrap();

    assert_eq!(new_data, test_data);
}

#[test]
fn test_convert_bin_to_csv() {
    let mut file = File::open(TEST_BIN).unwrap();
    let data = BinYPBankRecord::from_read(&mut file).unwrap();
    let new_data: CsvYPBankRecord = data.into();

    let mut file = File::open(TEST_CSV).unwrap();
    let test_data = CsvYPBankRecord::from_read(&mut file).unwrap();

    assert_eq!(new_data, test_data);
}

#[test]
fn test_convert_bin_to_txt() {
    let mut file = File::open(TEST_BIN).unwrap();
    let data = BinYPBankRecord::from_read(&mut file).unwrap();
    let new_data: TxtYPBankRecord = data.into();

    let mut file = File::open(TEST_TXT).unwrap();
    let test_data = TxtYPBankRecord::from_read(&mut file).unwrap();

    assert_eq!(new_data, test_data);
}


