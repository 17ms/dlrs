use clap::{Arg, ArgGroup, Command};
use regex::Regex;
use serde_json::Value;
use std::{path::PathBuf, process::exit};

// General error type to make error handling easier
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, PartialEq, Eq)]
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

fn create_thread_url(target: String) -> String {
    let url_vec: Vec<&str> = target.split("/").collect();
    let thread_id = url_vec.get(url_vec.len() - 1).unwrap();
    let board = url_vec.get(url_vec.len() - 3).unwrap();

    format!("https://a.4cdn.org/{}/thread/{}.json", board, thread_id)
}

fn create_board_url(target: String) -> String {
    let url_vec: Vec<&str> = target.split("/").collect();
    let board = url_vec.get(url_vec.len()).unwrap();

    format!("https://a.4cdn.org/{}/catalog.json", board)
}

async fn get_imagelist(json_url: &str) -> Result<Vec<(String, String)>> {
    let req_body = reqwest::get(json_url).await?.text().await?;
    let json_data: Value = serde_json::from_str(req_body.as_str())?;

    let mut thread_img_data: Vec<(String, String)> = Vec::new();
    for post in json_data["posts"].as_array().unwrap() {
        if post["tim"].is_i64() {
            thread_img_data.push((
                post["tim"].to_string(),
                post["ext"].as_str().unwrap().to_string(),
            ));
        } else {
            continue;
        }
    }

    Ok(thread_img_data)
}

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: add possible config-file for default output path (similar to wgdl.py)
    let (path, target, mode) = parse_cli_args()?;

    println!(
        "\nCONFIG:\n\tPATH: {:?}\n\tTARGET: {}\n\tMODE: {:?}\n",
        path, target, mode
    );

    match mode {
        Mode::Thread => {
            let json_url = create_thread_url(target);
            let id_list = get_imagelist(&json_url.as_str()).await?;
            println!("{:#?}", id_list);
            // 3.) download img based on json
        }
        Mode::Board => {
            let json_url = create_board_url(target);
            let id_list = get_imagelist(&json_url.as_str()).await?;
            println!("{:#?}", id_list);
            // 3.) download img based on json
        }
    };

    Ok(())
}
