use futures::{stream, StreamExt};
use reqwest::Client;
use std::path::PathBuf;
use tokio::{fs::File, io::AsyncWriteExt};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn download_images(img_data: &Vec<(String, PathBuf)>) -> Result<usize> {
    let client = Client::builder().build()?;

    let futures = stream::iter(img_data.iter().map(|data| async {
        let (url, path) = data;
        let send_fut = client.get(url).send();

        match send_fut.await {
            Ok(res) => match res.bytes().await {
                Ok(bytes) => {
                    let mut file = File::create(path).await.unwrap();
                    file.write_all(&bytes).await.unwrap();
                    println!("{} bytes from {:?} to {:?}", bytes.len(), &url, &path);
                }
                Err(_) => eprintln!("Error reading bytes from {}", url),
            },
            Err(_) => eprintln!("Error downloading {}", url),
        }
    }))
    .buffer_unordered(100)
    .collect::<Vec<()>>();

    futures.await;

    Ok(img_data.len())
}
