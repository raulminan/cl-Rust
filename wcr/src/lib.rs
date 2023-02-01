use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    Ok(FileInfo { num_lines, num_words, num_bytes, num_chars })
}



pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprint!("Failed to open {}: {}", filename, e),
            Ok(_) => println!("Opened {}", filename),
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("raulminan")
        .about("wc written in Rust")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input File(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .help("Show number of lines")
                .takes_value(false)
                .short("l")
                .long("lines"),
        )
        .arg(
            Arg::with_name("words")
                .value_name("WORDS")
                .help("Show number of words")
                .takes_value(false)
                .short("w")
                .long("words"),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .help("Show numebr of bytes")
                .takes_value(false)
                .short("c")
                .long("bytes"),
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .help("Show number of characters")
                .takes_value(false)
                .short("m")
                .long("chars")
                .conflicts_with("bytes"),
        )
        .get_matches();
    
    let mut lines = matches.is_present("lines"); 
    let mut words = matches.is_present("words"); 
    let mut bytes = matches.is_present("bytes"); 
    let chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        // equivalente a lambda x: x == False.
        // the values are references, so they have to be compared to &false
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(
        Config { 
            files: matches.values_of_lossy("files").unwrap(), 
            lines, 
            words, 
            bytes, 
            chars,
        }
    )
}


#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };

        assert_eq!(info.unwrap(), expected)
    }
}