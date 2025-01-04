fn main() {
    if let Err(error) = catr::run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
