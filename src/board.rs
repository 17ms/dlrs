use serde_json::Value;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn parse_url(url: &str) -> (String, String) {
    let url_split: Vec<&str> = url.split("/").collect();
    let board_name = url_split.get(url_split.len() - 2).unwrap();

    (
        format!("https://a.4cdn.org/{}/catalog.json", board_name),
        board_name.to_string(),
    )
}

// TODO: use futures
pub async fn get_imagelist(
    json_url: &str,
    board_name: &str,
    output_path: &PathBuf,
) -> Result<(Vec<(String, PathBuf)>, usize)> {
    let mut img_data: Vec<(String, PathBuf)> = Vec::new();
    let board_data = get_threadlist(json_url, board_name).await?;

    for thread_url in &board_data {
        let res_body = reqwest::get(thread_url).await?.text().await?;
        let json_data: Value = serde_json::from_str(res_body.as_str())?;

        json_data["posts"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|post| post["tim"].is_i64())
            .for_each(|post| {
                let id = post["tim"].to_string();
                let ext = post["ext"].as_str().unwrap().to_string();
                let filepath = output_path.join(format!("{}{}", id, ext).as_str());

                println!("{:#?}", filepath);

                img_data.push((
                    format!("https://i.4cdn.org/{}/{}{}", board_name, id, ext),
                    filepath,
                ));
            });
    }

    Ok((img_data, board_data.len()))
}

async fn get_threadlist(json_url: &str, board_name: &str) -> Result<Vec<String>> {
    let req_body = reqwest::get(json_url).await?.text().await?;
    let json_data: Value = serde_json::from_str(req_body.as_str())?;
    let board: Vec<Value> = json_data
        .as_array()
        .unwrap()
        .iter()
        .map(|page| page["threads"].clone())
        .collect();

    let mut board_data: Vec<String> = Vec::new();
    board.iter().for_each(|thread_arr| {
        thread_arr.as_array().unwrap().iter().for_each(|thread| {
            let url = format!(
                "https://a.4cdn.org/{}/thread/{}.json",
                board_name, thread["no"]
            );
            board_data.push(url);
        });
    });

    Ok(board_data)
}
