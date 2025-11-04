use clap::{Parser, ValueEnum};
use std::path::PathBuf;

use bank_record_converter::{AppError, BankRecord, BinYPBankRecord, CsvYPBankRecord, TxtYPBankRecord};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    #[arg(help = "Path to the input file")]
    input: PathBuf,

    #[arg(short = 'I', long)]
    // #[arg(help = "Path to the input file")]
    input_format: DataFormat,

    #[arg(short = 'O', long)]
    // #[arg(help = "Path to the output file")]
    output_format: DataFormat,
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

    if !std::path::Path::new(&args.input).exists() {
        return Err(AppError::IOError(std::io::ErrorKind::NotFound.into()));
    }

    let mut file = std::fs::File::open(&args.input)?;

    match args.input_format {
        DataFormat::Txt => {
            let data = TxtYPBankRecord::from_read(&mut file)?;
            match args.output_format {
                DataFormat::Txt => {
                    println!("Convertation from txt to txt is not supported");
                }
                DataFormat::Bin => {
                    let convert: BinYPBankRecord = data.into();
                    convert.write_to(&mut &std::io::stdout())?;
                }
                DataFormat::Csv => {
                    let convert: CsvYPBankRecord = data.into();
                    convert.write_to(&mut &std::io::stdout())?;
                }
            }
        }
        DataFormat::Bin => {
            let data = BinYPBankRecord::from_read(&mut file)?;
            match args.output_format {
                DataFormat::Txt => {
                    let convert: TxtYPBankRecord = data.into();
                    convert.write_to(&mut &std::io::stdout())?;
                }
                DataFormat::Bin => {
                    println!("Convertation from bin to bin is not supported");
                }
                DataFormat::Csv => {
                    let convert: CsvYPBankRecord = data.into();
                    convert.write_to(&mut &std::io::stdout())?;
                }
            }
        }
        DataFormat::Csv => {
            let data = CsvYPBankRecord::from_read(&mut file)?;
            match args.output_format {
                DataFormat::Txt => {
                    let convert: TxtYPBankRecord = data.into();
                    convert.write_to(&mut &std::io::stdout())?;
                }
                DataFormat::Bin => {
                    let convert: BinYPBankRecord = data.into();
                    convert.write_to(&mut &std::io::stdout())?;
                }
                DataFormat::Csv => {
                    println!("Convertation from csv to csv is not supported");
                }
            }
        }
    }
    Ok(())
}
