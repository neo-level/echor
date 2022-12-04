use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Frederik Desmet <info@neo-level.com>")
        .about("Rust echo variant")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("The text to echo")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .long("omit-newline")
                .help("Do not print newline at the end"),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();

    println!("{}{}", text.join(" "), if matches.is_present("omit_newline") { "" } else { "\n" });
}
