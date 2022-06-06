use colored::Colorize;
use futures::{stream, StreamExt};
use reqwest::Client;
use serde_json::Value;
use std::path::PathBuf;
use tokio::{fs::File, io::AsyncWriteExt};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn get_imagelist(
    json_url: &str,
    board_name: &str,
    output_path: &PathBuf,
) -> Result<Vec<(String, PathBuf)>> {
    let req_body = reqwest::get(json_url).await?.text().await?;
    let json_data: Value = serde_json::from_str(req_body.as_str())?;

    let mut img_data: Vec<(String, PathBuf)> = Vec::new();
    json_data["posts"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|post| post["tim"].is_i64())
        .for_each(|post| {
            let id = post["tim"].to_string();
            let ext = post["ext"].as_str().unwrap().to_string();
            let filepath = output_path.join(format!("{}{}", id, ext).as_str());

            img_data.push((
                format!("https://i.4cdn.org/{}/{}{}", board_name, id, ext),
                filepath,
            ))
        });

    Ok(img_data)
}

pub async fn get_images(img_data: &Vec<(String, PathBuf)>) -> Result<usize> {
    let client = Client::builder().build()?;

    let futures = stream::iter(img_data.iter().map(|data| async {
        let (url, path) = data;
        let send_fut = client.get(url).send();

        match send_fut.await {
            Ok(res) => match res.bytes().await {
                Ok(bytes) => {
                    let mut file = File::create(path).await.unwrap();
                    file.write_all(&bytes).await.unwrap();

                    println!(
                        "{}",
                        format!("{} bytes: {:?} -> {:?}", bytes.len(), url, path)
                            .italic()
                            .purple()
                    );
                }
                Err(_) => eprintln!(
                    "{}",
                    format!("Error reading bytes from {}", url).bold().red()
                ),
            },
            Err(_) => eprintln!("{}", format!("Error downloading {}", url).bold().red()),
        }
    }))
    .buffer_unordered(100)
    .collect::<Vec<()>>();

    futures.await;

    Ok(img_data.len())
}
