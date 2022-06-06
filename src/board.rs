use serde_json::Value;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn parse_url(url: &str) -> (String, String) {
    let url_split: Vec<&str> = url.split("/").collect();
    let board_name = url_split.get(url_split.len() - 2).unwrap();

    (
        format!("https://a.4cdn.org/{}/catalog.json", board_name),
        board_name.to_string(),
    )
}

pub async fn get_threadlist(json_url: &str, board_name: &str) -> Result<(usize, Vec<String>)> {
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

    Ok((board_data.len(), board_data))
}
