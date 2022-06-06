use serde_json::Value;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn parse_url(url: &str) -> (String, String) {
    let url_split: Vec<&str> = url.split("/").collect();
    let thread_id = url_split.get(url_split.len() - 1).unwrap();
    let board_name = url_split.get(url_split.len() - 3).unwrap();

    (
        format!(
            "https://a.4cdn.org/{}/thread/{}.json",
            board_name, thread_id
        ),
        board_name.to_string(),
    )
}

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
