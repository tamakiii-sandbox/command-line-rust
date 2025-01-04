fn main() {
    if let Err(error) = catr::get_args().and_then(catr::run) {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
