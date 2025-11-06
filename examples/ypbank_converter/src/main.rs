#![warn(missing_docs)]
//! Програма для конвертации данных из одного формата в другой
//! Пример запуска: `ypbank_converter --input ../../tests/data/examples_file/records_example.bin -I bin -O txt > output_file.txt`
use clap::{Parser, ValueEnum};
use std::{io::Write, path::PathBuf};

use bank_record_converter::{BankRecordConvertor, DataFormat, error::AppError};

/// CLI arguments
#[derive(Parser, Debug)]
struct Cli {
    /// Input file
    #[arg(short, long)]
    #[arg(help = "Path to the input file")]
    input: PathBuf,

    /// input file format
    #[arg(short = 'I', long)]
    input_format: InputDataFormat,

    /// output file format
    #[arg(short = 'O', long)]
    output_format: InputDataFormat,
}

/// Data format
#[derive(ValueEnum, Clone, Debug)]
enum InputDataFormat {
    /// txt format YPBank
    Txt,
    /// bin format YPBank
    Bin,
    /// csv format YPBank
    Csv,
}

fn main() -> Result<(), AppError> {
    let args = Cli::parse();

    if !std::path::Path::new(&args.input).exists() {
        return Err(AppError::IOError(std::io::ErrorKind::NotFound.into()));
    }

    let file = std::fs::File::open(&args.input)?;

    let stdout = std::io::stdout();
    let mut writer = std::io::BufWriter::new(stdout.lock());

    match (args.input_format, args.output_format) {
        (InputDataFormat::Txt, InputDataFormat::Txt) => {
            BankRecordConvertor::from_read(file, &DataFormat::TXT)?
                .convert_to(&DataFormat::TXT)
                .write_to(&mut writer)?
        }
        (InputDataFormat::Txt, InputDataFormat::Bin) => {
            BankRecordConvertor::from_read(file, &DataFormat::TXT)?
                .convert_to(&DataFormat::BIN)
                .write_to(&mut writer)?
        }
        (InputDataFormat::Txt, InputDataFormat::Csv) => {
            BankRecordConvertor::from_read(file, &DataFormat::TXT)?
                .convert_to(&DataFormat::CSV)
                .write_to(&mut writer)?
        }
        (InputDataFormat::Bin, InputDataFormat::Txt) => {
            BankRecordConvertor::from_read(file, &DataFormat::BIN)?
                .convert_to(&DataFormat::TXT)
                .write_to(&mut writer)?
        }
        (InputDataFormat::Bin, InputDataFormat::Bin) => {
            BankRecordConvertor::from_read(file, &DataFormat::BIN)?
                .convert_to(&DataFormat::BIN)
                .write_to(&mut writer)?
        }
        (InputDataFormat::Bin, InputDataFormat::Csv) => {
            BankRecordConvertor::from_read(file, &DataFormat::BIN)?
                .convert_to(&DataFormat::CSV)
                .write_to(&mut writer)?
        }
        (InputDataFormat::Csv, InputDataFormat::Txt) => {
            BankRecordConvertor::from_read(file, &DataFormat::CSV)?
                .convert_to(&DataFormat::TXT)
                .write_to(&mut writer)?
        }
        (InputDataFormat::Csv, InputDataFormat::Bin) => {
            BankRecordConvertor::from_read(file, &DataFormat::CSV)?
                .convert_to(&DataFormat::BIN)
                .write_to(&mut writer)?
        }
        (InputDataFormat::Csv, InputDataFormat::Csv) => {
            BankRecordConvertor::from_read(file, &DataFormat::CSV)?
                .convert_to(&DataFormat::CSV)
                .write_to(&mut writer)?
        }
    }

    writer.flush()?;
    Ok(())
}
