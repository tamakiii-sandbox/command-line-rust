use clap::{Arg, Command};
use std::convert::From;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(i) if i > 0 => Ok(i),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3は正の整数なのでOK
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // 数字でない文字列の場合はエラー
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // 0の場合もエラー
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust head")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(0..)
                .default_value("-"),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("count")
                .help("Print count lines of each of the specified files.")
                .num_args(1)
                .default_value("0")
                .value_parser(clap::value_parser!(usize))
                .conflicts_with("bytes"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .help("Print bytes of each of the specified files.")
                .num_args(1)
                .value_parser(clap::value_parser!(usize)),
        )
        .get_matches();

    let files = matches
        .get_many::<String>("files")
        .unwrap()
        .map(ToOwned::to_owned)
        .collect();

    let lines = matches.get_one::<usize>("lines").copied().unwrap();
    let bytes = matches.get_one::<usize>("bytes").copied();

    Ok(Config {
        files,
        lines,
        bytes,
    })
}
