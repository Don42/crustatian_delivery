use scraper::{Html, Selector};
use serde_json::{Value};

const URI: &str = "https://www.dhl.de/int-verfolgen/search";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    args.next();
    let id: &str = &args.next().unwrap();
    let params = [
        ("idc", id),
    ];
    let client = reqwest::Client::new();
    let resp = client.get(URI)
        .query(&params)
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&resp);
    let selector = Selector::parse("script").unwrap();
    let script: String = document
            .select(&selector)
            .next()
            .unwrap()
            .text()
            .collect();
    let initial_state: &str = script
        .lines()
        .into_iter()
        .collect::<Vec<&str>>()[3];
    // JSON.parse("
    let start_json = initial_state.find("JSON.parse(\"").map(|i| i + 12).unwrap();
    let end_json = initial_state.rfind("\"),").map(|i| i).unwrap();
    let initial_state = initial_state
        .get(start_json..end_json)
        .map(|s| s.replace("\\\"", "\""))
        .unwrap();
    println!(
        "{}",
        initial_state
        );
    let parsed: Value = serde_json::from_str(&initial_state)?;
    //println!("{:?}", parsed);
    Ok(())
}
