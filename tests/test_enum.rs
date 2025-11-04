use std::fs::File;

use bank_record_converter::{BankRecord, BankRecordEnum, BinYPBankRecord, CsvYPBankRecord, TxtYPBankRecord};
use rstest::rstest;
#[rstest]
#[case("txt")]
#[case("bin")]
#[case("csv")]
fn test_enum_convert(
    #[case] format: &str,
    #[files("tests/data/examples_file/records_example.*")] path: std::path::PathBuf
) {
    // TODO: Дописать тесты на неправильный передоваемый формат
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if ext == format {
            println!("Обрабатываем {} файл: {}", ext, path.display());
        }
        else {
            return;
        }
    }
    
    let mut file = File::open(path).unwrap();
    let recorder = match format {
        "txt" => BankRecordEnum::TXT(TxtYPBankRecord::from_read(&mut file).unwrap()),
        "bin" => BankRecordEnum::BIN(BinYPBankRecord::from_read(&mut file).unwrap()),
        "csv" => BankRecordEnum::CSV(CsvYPBankRecord::from_read(&mut file).unwrap()),
        _ => panic!("Unknown format")
    }; 

    match recorder {
        BankRecordEnum::TXT(_) => {
            assert_eq!(format, "txt");
        },
        BankRecordEnum::BIN(_) => {
            assert_eq!(format, "bin");
        },
        BankRecordEnum::CSV(_) => {
            assert_eq!(format, "csv");
        }
    }

    assert!(recorder.len() > 0);

}