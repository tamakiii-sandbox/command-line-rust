use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do no print newline")
                .num_args(0),
        )
        .get_matches();

    let text: Vec<_> = matches
        .get_many::<String>("text")
        .unwrap()
        .map(|s| s.to_string())
        .collect();

    let ending = if matches.get_flag("omit_newline") {
        ""
    } else {
        "\n"
    };

    print!("{}{}", text.join(" "), ending);
}
