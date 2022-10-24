use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>, // optional usize
}

// -----------------------------------------------------------------------------

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("raulminan")
        .about("head written in Rust")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
                //.conflicts_with("bytes"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .help("Number of bytes")
                .conflicts_with("lines")
                .takes_value(true) 
                // .takes_value is needed here and not in the others
                // bc it's of type Option<()>
                
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int) // unpacks a &str from Some and sends it to the fn
        // Option::map returns Option<Result>, use transpose()
        // to make it Result<Option>
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;
    
    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose() // whyyyy
        .map_err(|e| format!("illegal byte count -- {}", e))?;


    Ok(Config {
        // using .unwrap() it's safe when there should be a least value.
        // in files and lines there's always a value (the default)
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes, // idiomatic way to write bytes: bytes,
    })
}

// -----------------------------------------------------------------------------

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();

    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 {"\n"} else {""},
                        &filename
                    );
                }

                // check if config.bytes is some number of bytes to read
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64); // std::io::Read expects u64
                    let mut buffer = vec![0; num_bytes]; // create vector of zeros of len num_bytes
                    
                    // read the desired number of bytes from the filehandle into the 
                    // buffer.
                    let bytes_read = handle.read(&mut buffer)?;
                    print!(
                        "{}",
                        //convert selected bytes to a string
                        String::from_utf8_lossy(&buffer[..bytes_read])
                        // note the range operation .. to select only the bytes
                        // that actually read. 
                    );

                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }

            }
        }
    }
    Ok(())
}

// --------------------------------------------------

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
        //   other options:
        //   Err(val.into())
        //   Err(Into::into(val))
    }
}

#[test]
fn test_parse_positive_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // 0 is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

// ---------------------------------------------------------------------------

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// -----------------------------------------------------------------------------