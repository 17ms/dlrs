use clap::{Arg, ArgGroup, Command};
use futures::{stream, StreamExt};
use regex::Regex;
use reqwest::Client;
use serde_json::Value;
use std::{path::PathBuf, process::exit};
use tokio::{fs::File, io::AsyncWriteExt};

// General type to make error handling easier
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

fn create_thread_url(target: String) -> (String, String) {
    let url_vec: Vec<&str> = target.split("/").collect();
    let thread_id = url_vec.get(url_vec.len() - 1).unwrap();
    let board = url_vec.get(url_vec.len() - 3).unwrap();

    (
        format!("https://a.4cdn.org/{}/thread/{}.json", board, thread_id),
        board.to_string(),
    )
}

//fn _create_board_url(target: String) -> String {
//    let url_vec: Vec<&str> = target.split("/").collect();
//    let board = url_vec.get(url_vec.len()).unwrap();
//
//    format!("https://a.4cdn.org/{}/catalog.json", board)
//}

async fn get_imagelist(
    json_url: &str,
    board_name: &str,
    output_path: &PathBuf,
) -> Result<Vec<(String, PathBuf)>> {
    let req_body = reqwest::get(json_url).await?.text().await?;
    let json_data: Value = serde_json::from_str(req_body.as_str())?;

    let mut thread_img_data: Vec<(String, PathBuf)> = Vec::new();
    json_data["posts"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|post| post["tim"].is_i64())
        .for_each(|post| {
            let id = post["tim"].to_string();
            let ext = post["ext"].as_str().unwrap().to_string();
            let filepath = output_path.join(format!("{}{}", id, ext).as_str());

            thread_img_data.push((
                format!("https://i.4cdn.org/{}/{}{}", board_name, id, ext),
                filepath,
            ))
        });

    Ok(thread_img_data)
}

async fn download_images(input_output_data: &Vec<(String, PathBuf)>) -> Result<u16> {
    let client = Client::builder().build()?;

    let futures = stream::iter(input_output_data.iter().map(|data| async {
        let (url, path) = data;
        let send_fut = client.get(url).send();

        match send_fut.await {
            Ok(res) => match res.bytes().await {
                Ok(bytes) => {
                    let mut file = File::create(path).await.unwrap();
                    file.write_all(&bytes).await.unwrap();

                    println!("Got {} bytes from {:?} to {:?}", bytes.len(), &url, &path)
                }
                Err(_) => eprintln!("Error reading bytes from {}", url),
            },
            Err(_) => eprintln!("Error downloading {}", url),
        }
    }))
    .buffer_unordered(100)
    .collect::<Vec<()>>();

    futures.await;

    Ok(0)
}

#[tokio::main]
async fn main() -> Result<()> {
    let (path, target, mode) = parse_cli_args()?;

    println!(
        "\nDownload configuration:\n\tOUTPUT: {:?}\n\tURL: {}\n\tDOWNLOAD-MODE: {:?}\n",
        path, target, mode
    );

    match mode {
        Mode::Thread => {
            let (json_url, board_name) = create_thread_url(target);
            let targets = get_imagelist(&json_url.as_str(), &board_name.as_str(), &path).await?;
            let _total_file_amount = download_images(&targets).await?;

            //println!("Total files downloaded: {}", total_file_amount);
        }
        //Mode::Board => {
        //    let json_url = create_board_url(target);
        //    let id_list = get_imagelist(&json_url.as_str()).await?;
        //    //println!("{:#?}", id_list);
        //    get_images(id_list).await?;
        //}
        _ => println!("Not supported"),
    };

    Ok(())
}
