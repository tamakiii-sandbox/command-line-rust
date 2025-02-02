mod args;

pub use args::get_args;
use args::Config;

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq, Default)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

#[derive(Default)]
struct Total {
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total = Total {
        ..Default::default()
    };

    for filename in &config.files {
        match open(filename) {
            Err(error) => eprintln!("{}: {}", filename, error),
            Ok(file) => match count(file) {
                Err(error) => eprintln!("{}: {}", filename, error),
                Ok(info) => {
                    print_result(&info, &config, filename);
                    add_total(&mut total, &info);
                }
            },
        }
    }

    if config.files.len() > 1 {
        print_total(&total, &config);
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    // let reader: Box<dyn BufRead> = if filename == "-" {
    //     Box::new(BufReader::new(io::stdin()))
    // } else {
    //     Box::new(BufReader::new(File::open(filename)?))
    // };
    // Ok(reader)

    Ok(match filename {
        "-" => Box::new(BufReader::new(io::stdin())),
        _ => Box::new(BufReader::new(File::open(filename)?)),
    })
}

fn count<R: BufRead>(mut file: R) -> MyResult<FileInfo> {
    let mut info = FileInfo {
        ..Default::default()
    };
    let mut line = String::new();

    while file.read_line(&mut line)? > 0 {
        info.num_lines += 1;
        info.num_bytes += line.len();
        info.num_words += line.split_whitespace().count();
        info.num_chars += line.chars().count();
        line.clear();
    }

    Ok(info)

    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;

    // let num_lines = contents.lines().count();
    // let num_words = contents.split_whitespace().count();
    // let num_bytes = contents.len();
    // let num_chars = contents.chars().count();

    // Ok(FileInfo {
    //     num_lines,
    //     num_words,
    //     num_bytes,
    //     num_chars,
    // })
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

fn print_result(info: &FileInfo, config: &Config, filename: &str) {
    println!(
        "{}{}{}{}{}",
        format_field(info.num_lines, config.lines),
        format_field(info.num_words, config.words),
        format_field(info.num_bytes, config.bytes),
        format_field(info.num_chars, config.chars),
        match filename {
            "-" => "".to_string(),
            _ => format!(" {}", filename),
        }
    );
}

fn print_total(total: &Total, config: &Config) {
    println!(
        "{}{}{}{} total",
        format_field(total.lines, config.lines),
        format_field(total.words, config.words),
        format_field(total.bytes, config.bytes),
        format_field(total.chars, config.chars),
    )
}

fn add_total(total: &mut Total, info: &FileInfo) {
    // *total = Total {
    //     lines: total.lines + info.num_lines,
    //     words: total.words + info.num_words,
    //     bytes: total.bytes + info.num_bytes,
    //     chars: total.chars + info.num_chars,
    // }
    total.lines += info.num_lines;
    total.words += info.num_words;
    total.bytes += info.num_bytes;
    total.chars += info.num_chars;
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
