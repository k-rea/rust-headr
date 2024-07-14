use clap::Parser;
use std::error::Error;
use std::io::{BufRead, BufReader, Read, Write};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(author, about, version)]
struct Args {
    #[arg(default_value = "-")]
    files: Vec<String>,
    #[arg(short = 'n', long, default_value_t = 10)]
    lines: u64,
    #[arg(short = 'c', long, conflicts_with = "lines")]
    bytes: Option<u64>,
}
pub fn run() -> MyResult<()> {
    let args = Args::parse();
    let file_len = args.files.len();
    args.files.iter().enumerate().try_for_each(|(index, filename)| {
        match open(filename) {
            Err(e) => {
                eprintln!("Error opening file {}: {}", filename, e);
                Ok(())
            }
            Ok(mut reader) => {
                if file_len > 1 { println!("==> {} <==", filename) }
                if let Some(bytes) = args.bytes {
                    let mut buffer = vec![0; bytes as usize];
                    let n = reader.take(bytes).read(&mut buffer)?;
                    std::io::stdout().write_all(&buffer[..n])?;
                } else {
                    let mut line = String::new();
                    for _ in 0..args.lines {
                        let buf = reader.read_line(&mut line)?;
                        if buf == 0 { break }
                        print!("{}", line);
                        line.clear();
                    }
                }
                if file_len > 1 && index < file_len - 1 {
                    println!()
                }
                Ok(())
            }
        }
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(std::fs::File::open(filename)?)))
    }
}