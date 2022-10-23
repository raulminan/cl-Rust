fn main() {
    // check if return value of catr::run matches Err(e)
    // where e is some value that implemets the Error trait
    if let Err(e) = catr::get_args().and_then(catr::run) {
        eprintln!("{}", e); // error print line
        std::process::exit(1);
    }
}
