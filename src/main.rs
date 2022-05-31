use clap::{Arg, ArgGroup, Command};
use regex::Regex;
use std::{path::PathBuf, process::exit};

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    Thread,
    Board,
}

fn parse_cli_args() -> (PathBuf, String, Mode) {
    let matches = Command::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("Set an output directory")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("thread")
                .short('t')
                .long("thread")
                .value_name("URL")
                .help("Set a thread URL")
                .takes_value(true),
        )
        .arg(
            Arg::new("board")
                .short('b')
                .long("board")
                .value_name("URL")
                .help("Set a board URL")
                .takes_value(true),
        )
        .group(
            ArgGroup::new("target")
                .args(&["thread", "board"])
                .required(true),
        )
        .get_matches();

    let re = Regex::new(
        r"(http|https)://boards.(4chan|4channel).org/[a-zA-Z]{1,4}/(catalog|thread/\d+)",
    )
    .unwrap();

    let path = PathBuf::from(matches.value_of("path").unwrap());
    let target_match = matches.value_of("target").unwrap();
    let target = match re.is_match(target_match) {
        true => target_match,
        false => {
            eprintln!("Error: Invalid URL");
            exit(0x0100);
        }
    };
    let mode = match matches.is_present("thread") {
        true => Mode::Thread,
        false => Mode::Board,
    };

    (path, String::from(target), mode)
}

fn main() {
    // TODO: add var for default output path (similar to wgdl.py)
    let (path, target, mode) = parse_cli_args();

    println!(
        "CONFIG:\n\tPATH: {:?}\n\tTARGET: {}\n\tMODE: {:?}",
        path, target, mode
    );
}
