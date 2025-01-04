use clap::{Arg, Command};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("wcr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust wc")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(0..)
                .default_value("-"),
        )
        .arg(
            Arg::new("lines")
                .value_name("LINES")
                .short('n')
                .long("lines")
                .help("The number of lines in each input file is written to the standard output.")
                .num_args(0)
        )
        .arg(
            Arg::new("words")
                .value_name("WORDS")
                .short('w')
                .long("words")
                .help("The number of words in each input file is written to the standard output.")
                .num_args(0)
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .short('c')
                .long("bytes")
                .help("The number of bytes in each input file is written to the standard output.  This will cancel out any prior usage of the -m option.")
                .num_args(0)
        )
        .arg(
            Arg::new("chars")
                .value_name("CHARS")
                .short('m')
                .long("chars")
                .help("The number of characters in each input file is written to the standard output.  If the current locale does not support multibyte characters, this is equivalent to the -c option.  This will cancel out any prior usage of the -c option.")
                .num_args(0)
        )
        .get_matches();

    let files = matches
        .get_many::<String>("files")
        .unwrap()
        .map(ToOwned::to_owned)
        .collect();

    let mut lines = matches.get_flag("lines");
    let mut words = matches.get_flag("words");
    let mut bytes = matches.get_flag("bytes");
    let chars = matches.get_flag("chars");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files,
        lines,
        words,
        bytes,
        chars,
    })
}
