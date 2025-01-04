use clap::{Arg, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cat")
        .arg(Arg::new("files").help("files").required(true).num_args(1..))
        .arg(
            Arg::new("number_lines")
                .short('n')
                .help("Number the output lines, starting at 1.")
                .num_args(0),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .help("Number the non-blank output lines, starting at 1.")
                .num_args(0),
        )
        .get_matches();

    let files = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|s| s.to_string())
        .collect();

    Ok(Config {
        files: files,
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank_lines"),
    })
}
