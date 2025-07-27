use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    let config = get_args()?;
    let files = &config.files;
    let number_lines = &config.number_lines;
    let number_nonblank_lines = &config.number_nonblank_lines;

    let mut line_num = 0;

    for filename in files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(reader) => {
                for line in reader.lines() {
                    let line = line?;
                    let is_empty_line = line.trim().is_empty();

                    if *number_lines {
                        line_num += 1;
                        println!("{:6}\t{}", line_num, line);
                    } else if *number_nonblank_lines {
                        if !is_empty_line {
                            line_num += 1;
                            println!("{:6}\t{}", line_num, line);
                        } else {
                            println!("{}", line);
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("cat_03")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short('n')
                .long("number")
                .help("Number lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false)
                .conflicts_with("number_lines"),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .values_of_lossy("files")
            .unwrap_or_else(|| vec!["-".to_string()]),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
