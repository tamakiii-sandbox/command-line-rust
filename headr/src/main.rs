fn main() {
    if let Err(error) = headr::get_args().and_then(headr::run) {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
