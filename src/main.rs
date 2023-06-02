mod board;
mod downloader;
mod thread;

use clap::{Arg, ArgGroup, Command};
use colored::Colorize;
use regex::Regex;
use std::{path::PathBuf, process::exit};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
enum Mode {
    Thread,
    Board,
}

fn parse_cli_args() -> Result<(PathBuf, String, Mode)> {
    let matches = Command::new("dlrs")
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
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
        r"^((http|https)://)?boards.(4chan|4channel).org/[a-zA-Z]{1,4}/(catalog|thread/\d+)$",
    )?;

    let path = PathBuf::from(matches.value_of("output").unwrap());
    let target_match = matches.value_of("target").unwrap();
    let target = match re.is_match(target_match) {
        true => target_match,
        false => {
            eprintln!("{}", "Error: Invalid URL format".to_string().bold().red());
            exit(0x0100);
        }
    };
    let mode = match matches.is_present("thread") {
        true => Mode::Thread,
        false => Mode::Board,
    };

    Ok((path, target.to_string(), mode))
}

#[tokio::main]
async fn main() -> Result<()> {
    let (path, target, mode) = parse_cli_args()?;
    println!(
        "{}",
        format!(
            "\nDownload configuration:\n\tOUTPUT PATH: {:?}\n\tURL: {}\n\tDOWNLOAD MODE: {:?}\n",
            path, target, mode
        )
        .bold()
        .green()
    );

    match mode {
        Mode::Thread => {
            let (json_url, board_name) = thread::parse_url(&target);
            println!(
                "{}",
                format!("Parsing JSON from {}", json_url).bold().blue()
            );
            let img_data = downloader::get_imagelist(&json_url, &board_name, &path).await?;
            let filecount = downloader::get_images(&img_data).await?;

            println!(
                "{}",
                format!("Total of {} files downloaded from 1 thread.\n", filecount)
                    .bold()
                    .green()
            );
        }
        Mode::Board => {
            let (json_url, board_name) = board::parse_url(&target);
            let (thread_amt, thread_data) = board::get_threadlist(&json_url, &board_name).await?;
            let mut filecount: usize = 0;
            for url in &thread_data {
                println!("{}", format!("Parsing JSON from {}", url).bold().blue());
                let img_data = downloader::get_imagelist(url, &board_name, &path).await?;
                let total_amt = downloader::get_images(&img_data).await?;
                filecount += total_amt;
            }

            println!(
                "{}",
                format!(
                    "Total of {} files downloaded from {} threads.\n",
                    filecount, thread_amt
                )
                .bold()
                .green()
            );
        }
    }

    Ok(())
}
