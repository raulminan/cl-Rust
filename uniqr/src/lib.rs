use clap::{App, Arg};
use std::{
    error::Error, 
    fs::File,
    io::{self, BufReader, BufRead, Write}
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches =  App::new("uniqr")
        .version("0.1.0")
        .author("raulminan")
        .about("uniq written in Rust")
        .arg(
            Arg::with_name("in_file")
                .value_name("IN FILE")
                .help("Input file")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("count")
                .value_name("COUNT")
                .help("prefix lines by number of occurences")
                .takes_value(false)
                .short("c")
                .long("count"),
        )
        .arg(
            Arg::with_name("out_file")
                .value_name("OUT FILE")
                .help("Print output to this file instead of STDOUT")
        )
        .get_matches();
    
    let in_file = matches.value_of_lossy("in_file").unwrap().to_string();
    let out_file = matches.value_of("out_file")
        .map(|v| v.to_string());
        // converts Option<&str> to Option<String>
        // .map(str::to_string) is also an option
        // .map(String::from) also works and is used in the book

    let count = matches.is_present("count");


    Ok( Config { in_file, out_file, count } ) 

}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;
    
    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;

    let mut print = | count: u64, text: &str | -> MyResult<()> {
        if count > 0 {
            if config.count {
                write!(out_file, "{:>4} {}", count, text)?;
            } else {
                write!(out_file, "{}", text)?;
            }
        };
        Ok(())
    };

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }
    
    print(count, &previous)?;
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

