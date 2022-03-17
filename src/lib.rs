use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Cli {
    #[clap(help = "Input", default_value = "-")]
    files: Vec<String>,

    #[clap(help = "Number lines", short, long = "number", group("line_flags"))]
    number_lines: bool,

    #[clap(
        help = "Number nonblank lines",
        short = 'b',
        long = "number-nonblank",
        group("line_flags")
    )]
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Cli> {
    Ok(Cli::parse())
}

pub fn run(cli: Cli) -> MyResult<()> {
    for filename in cli.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {filename}: {e}"),
            Ok(buf_reader) => {
                let mut last_num = 0;
                for (line_number, line_result) in buf_reader.lines().enumerate() {
                    let line = line_result?;
                    if cli.number_lines {
                        println!("{:6}\t{line}", line_number + 1);
                    } else if cli.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:6}\t{line}", last_num);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{line}");
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
