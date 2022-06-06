mod board;
mod downloader;
mod thread;

use clap::{Arg, ArgGroup, Command};
use regex::Regex;
use std::{path::PathBuf, process::exit};
use tokio;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
enum Mode {
    Thread,
    Board,
}

fn parse_cli_args() -> Result<(PathBuf, String, Mode)> {
    let matches = Command::new("WGDL-imagescraper written in Rust")
        .version("0.1.0")
        .author("Arttu Einistö <einisto@proton.me>")
        .about("Scrapes images efficiently from 4chan.org")
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
            eprintln!("Error: Invalid URL");
            exit(0x0100);
        }
    };
    let mode = match matches.is_present("thread") {
        true => Mode::Thread,
        false => Mode::Board,
    };

    Ok((path, String::from(target), mode))
}

#[tokio::main]
async fn main() -> Result<()> {
    let (path, target, mode) = parse_cli_args()?;
    println!(
        "\nDownload configuration:\n\tOUTPUT PATH: {:?}\n\tURL: {}\n\tDOWNLOAD MODE: {:?}\n",
        path, target, mode
    );

    match mode {
        Mode::Thread => {
            let (json_url, board_name) = thread::parse_url(&target);
            let img_data = thread::get_imagelist(&json_url, &board_name, &path).await?;
            let total_amt = downloader::download_images(&img_data).await?;
            println!("Total of {} files downloaded from 1 thread.\n", total_amt);
        }
        Mode::Board => {
            let (json_url, board_name) = board::parse_url(&target);
            let (img_data, thread_amt) =
                board::get_imagelist(&json_url, &board_name, &path).await?;
            let total_amt = downloader::download_images(&img_data).await?;
            println!(
                "Total of {} files downloaded from {} threads.\n",
                total_amt, thread_amt
            );
        }
    }

    Ok(())
}
