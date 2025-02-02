use clap::{Arg, Command};
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

struct Total {
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total = Total {
        lines: 0,
        words: 0,
        bytes: 0,
        chars: 0,
    };

    for filename in &config.files {
        match open(filename) {
            Err(error) => eprintln!("{}: {}", filename, error),
            Ok(file) => match count(file) {
                Err(error) => eprintln!("{}: {}", filename, error),
                Ok(info) => {
                    println!(
                        "{}{}{}{}{}",
                        format_field(info.num_lines, config.lines),
                        format_field(info.num_words, config.words),
                        format_field(info.num_bytes, config.bytes),
                        format_field(info.num_chars, config.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", filename)
                        }
                    );
                    add_total(&mut total, info);
                }
            },
        }
    }

    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total.lines, config.lines),
            format_field(total.words, config.words),
            format_field(total.bytes, config.bytes),
            format_field(total.chars, config.chars),
        )
    }

    Ok(())
}

fn add_total(total: &mut Total, info: FileInfo) {
    total.lines += info.num_lines;
    total.words += info.num_words;
    total.bytes += info.num_bytes;
    total.chars += info.num_chars;
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
                .short('l')
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
                .conflicts_with("chars")
        )
        .arg(
            Arg::new("chars")
                .value_name("CHARS")
                .short('m')
                .long("chars")
                .help("The number of characters in each input file is written to the standard output.  If the current locale does not support multibyte characters, this is equivalent to the -c option.  This will cancel out any prior usage of the -c option.")
                .num_args(0)
                .conflicts_with("bytes")
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

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
//     let mut num_lines = 0;
//     let mut num_words = 0;
//     let mut num_bytes = 0;
//     let mut num_chars = 0;
//     let mut line = String::new();

//     loop {
//         let line_bytes = file.read_line(&mut line)?;
//         if line_bytes == 0 {
//             break;
//         }
//         num_bytes += line_bytes;
//         num_lines += 1;
//         num_words += line.split_whitespace().count();
//         num_chars += line.chars().count();
//         line.clear();
//     }

//     Ok(FileInfo {
//         num_lines,
//         num_words,
//         num_bytes,
//         num_chars,
//     })
// }

fn count<R: BufRead>(mut file: R) -> MyResult<FileInfo> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let num_lines = contents.lines().count();
    let num_words = contents.split_whitespace().count();
    let num_bytes = contents.len();
    let num_chars = contents.chars().count();

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

// fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
//     let mut num_lines = 0;
//     let mut num_words = 0;
//     let mut num_bytes = 0;
//     let mut num_chars = 0;

//     let mut line = String::new();
//     let mut reader = file;

//     while {
//         line.clear();
//         reader.read_line(&mut line)?
//     } > 0
//     {
//         num_lines += 1;
//         num_bytes += line.len();
//         num_words += line.split_whitespace().count();
//         num_chars += line.chars().count();
//         line.clear();
//     }

//     Ok(FileInfo {
//         num_lines,
//         num_words,
//         num_bytes,
//         num_chars,
//     })
// }

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{count, format_field, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };

        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}
