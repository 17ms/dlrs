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
