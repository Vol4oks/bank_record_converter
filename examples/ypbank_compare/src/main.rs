#![warn(missing_docs)]
//! Данная программа сравнивает два файла по записям
//! Пример команды:
//! `ypbank_compare --file1 records_example.bin --format1 binary --file2 records_example.csv --format2 csv`
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

use bank_record_converter::{BankRecordConvertor, DataFormat, error::AppError};
#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    #[arg(help = "Path to the input file 1")]
    file1: PathBuf,

    #[arg(long)]
    format1: InputDataFormat,

    #[arg(long)]
    #[arg(help = "Path to the input file 2")]
    file2: PathBuf,

    #[arg(long)]
    format2: InputDataFormat,
}

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

    if !std::path::Path::new(&args.file1).exists() {
        return Err(AppError::IOError(std::io::ErrorKind::NotFound.into()));
    }

    if !std::path::Path::new(&args.file2).exists() {
        return Err(AppError::IOError(std::io::ErrorKind::NotFound.into()));
    }

    let file1 = std::fs::File::open(&args.file1)?;
    let file2 = std::fs::File::open(&args.file2)?;

    let format1 = match &args.format1 {
        InputDataFormat::Txt => DataFormat::TXT,
        InputDataFormat::Bin => DataFormat::BIN,
        InputDataFormat::Csv => DataFormat::CSV,
    };

    let format2 = match &args.format2 {
        InputDataFormat::Txt => DataFormat::TXT,
        InputDataFormat::Bin => DataFormat::BIN,
        InputDataFormat::Csv => DataFormat::CSV,
    };

    let records1 = BankRecordConvertor::from_read(file1, &format1)?;
    let records2 = BankRecordConvertor::from_read(file2, &format2)?;

    let mut flag = true;
    for (mes1, mes2) in records1.iter().zip(records2.iter()) {
        if !mes1.eq(mes2) {
            flag = false;
            println!("{:?}/n{:?}", mes1, mes2);
        }
    }

    if flag {
        let name_file1 = &args
            .file1
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        let name_file2 = &args
            .file2
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        println!(
            "The transaction records in '{}' and '{}' are identical.",
            name_file1, name_file2
        );
    }

    Ok(())
}
