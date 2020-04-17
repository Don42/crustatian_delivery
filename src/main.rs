use serde::{Serialize, Deserialize};
use scraper::{Html, Selector};
use serde_json::{Value};
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

const URI: &str = "https://www.dhl.de/int-verfolgen/search";

#[derive(Serialize, Deserialize, Debug)]
struct DHLContent {
    #[serde(alias = "sendungen")]
    shipments: Vec<Shipment>,
}


#[derive(Serialize, Deserialize, Debug)]
struct Shipment {
    #[serde(alias = "hasCompleteDetails")]
    has_complete_details: bool,
    id: String,
    #[serde(alias = "sendungsdetails")]
    shipment_details: ShipmentDetails,
}

#[derive(Serialize, Deserialize, Debug)]
struct ShipmentDetails {
    #[serde(alias = "istZugestellt")]
    is_delivered : bool,
    #[serde(alias = "eigenschaften")]
    properties: HashMap<String, String>,
    #[serde(alias = "sendungsverlauf")]
    shipment_history: ShipmentHistory,
}


#[derive(Serialize, Deserialize, Debug)]
struct ShipmentHistory {
    #[serde(alias = "datumAktuellerStatus")]
    date_of_current_status: String,
    #[serde(alias = "aktuellerStatus")]
    current_status: String,
    #[serde(alias = "kurzStatus")]
    short_status: String,
    events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    #[serde(alias = "datum")]
    date: String,
    #[serde(alias = "ort")]
    location: Option<String>,
    #[serde(alias = "status")]
    status: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("./state.json")?;
    let reader = BufReader::new(file);
    let parsed: DHLContent = serde_json::from_reader(reader)?;
    println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
    Ok(())
}

#[tokio::main]
async fn _main() -> Result<(), Box<dyn std::error::Error>> {
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
    let parsed: Value = serde_json::from_str(&initial_state)?;
    println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
    Ok(())
}
