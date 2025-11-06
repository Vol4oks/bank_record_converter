use pretty_assertions::assert_eq;
use rstest::rstest;

use std::{fs::File, path::PathBuf};

use bank_record_converter::{BankRecordConvertor, DataFormat};

#[rstest]
#[case(DataFormat::TXT)]
#[case(DataFormat::BIN)]
#[case(DataFormat::CSV)]
fn test_convert(
    #[case] target_format: DataFormat,
    #[files("tests/data/examples_file/*")] path: PathBuf,
) {
    let test_path = path.with_extension(target_format.to_string().to_lowercase());

    let inut_format = match path.extension().and_then(|ext| ext.to_str()) {
        Some("txt") => DataFormat::TXT,
        Some("bin") => DataFormat::BIN,
        Some("csv") => DataFormat::CSV,
        _ => panic!("Unknown file extension"),
    };

    println!(
        "Convert file '{:?}' and compare with '{:?}'",
        path, test_path
    );

    let file = File::open(&path).unwrap();

    let new_data = BankRecordConvertor::from_read(file, &inut_format)
        .unwrap()
        .convert_to(&target_format);

    let test_file = File::open(test_path).unwrap();

    let test_data = BankRecordConvertor::from_read(test_file, &target_format).unwrap();

    assert_eq!(new_data, test_data);
}
