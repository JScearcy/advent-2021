use clap::{App, Arg};

pub fn init<'a>() -> clap::ArgMatches<'a> {
    App::new("Advent of Code 2021")
        .arg(Arg::with_name("generate").short("g").long("generate").help("generate a new day template (used with -d)"))
        .arg(Arg::with_name("day").short("d").long("day").takes_value(true).required(true).help("integer indicating which day to run"))
        .arg(Arg::with_name("challenge").short("c").long("challenge").takes_value(true).required_unless("generate").help("integer indicating which challenge to run"))
        .arg(Arg::with_name("input").short("i").long("input").takes_value(true).help("path to input to read"))
        .arg(Arg::with_name("session").short("s").long("session").takes_value(true).help("session information if getting data remotely"))
        .arg(Arg::with_name("remote").short("r").long("remote").help("flag to allow remote input fetch").requires("session"))
        .arg(Arg::with_name("year").short("y").long("year").help("run challenge from a specific year"))
        .version("1.0.0")
        .get_matches()
}