use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> { // *
    println!("Hello, world!");
    Ok(())
}

// * returns either Ok, containing the unit type () or some error Err