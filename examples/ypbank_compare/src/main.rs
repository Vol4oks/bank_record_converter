use clap::{Parser, ValueEnum};
use std::path::PathBuf;

use bank_record_converter::{AppError, BankRecord, BankRecordEnum, BinYPBankRecord, CsvYPBankRecord, TxtYPBankRecord};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    #[arg(help = "Path to the input file 1")]
    file1: PathBuf,

    #[arg(long)]
    format1: DataFormat,

    #[arg(long)]
    #[arg(help = "Path to the input file 2")]
    file2: PathBuf,

    #[arg(long)]
    format2: DataFormat,
}

#[derive(ValueEnum, Clone, Debug)]
enum DataFormat {
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

    let mut file1 = std::fs::File::open(&args.file1)?;
    let mut file2 = std::fs::File::open(&args.file2)?;

    let records1: BankRecordEnum = match args.format1 {
        DataFormat::Txt => BankRecordEnum::TXT(TxtYPBankRecord::from_read(&mut file1)?),
        DataFormat::Bin => BankRecordEnum::BIN(BinYPBankRecord::from_read(&mut file1)?),
        DataFormat::Csv => BankRecordEnum::CSV(CsvYPBankRecord::from_read(&mut file1)?),
    };

    let records2: BankRecordEnum = match args.format2 {
        DataFormat::Txt => BankRecordEnum::TXT(TxtYPBankRecord::from_read(&mut file2)?),
        DataFormat::Bin => BankRecordEnum::BIN(BinYPBankRecord::from_read(&mut file2)?),
        DataFormat::Csv => BankRecordEnum::CSV(CsvYPBankRecord::from_read(&mut file2)?),
    };

    let mut flag = true;
    for (mes1, mes2) in records1.iter().zip(records2.iter()) {
        if !mes1.eq(mes2){
            flag = false;
            println!("{:?}/n{:?}", mes1, mes2);
        }
    }

    if flag{
        let name_file1 = &args.file1.file_name().unwrap_or_default().to_str().unwrap_or_default();
        let name_file2 = &args.file2.file_name().unwrap_or_default().to_str().unwrap_or_default();

        println!("The transaction records in '{}' and '{}' are identical.", name_file1, name_file2 );
    }

    Ok(())
}

