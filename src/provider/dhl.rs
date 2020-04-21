use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const URI_API: &str = "https://www.dhl.de/int-verfolgen/data/search";

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    #[serde(alias = "sendungen")]
    shipments: Vec<Shipment>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Shipment {
    #[serde(alias = "hasCompleteDetails")]
    has_complete_details: bool,
    id: String,
    #[serde(alias = "sendungsdetails")]
    shipment_details: ShipmentDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipmentDetails {
    #[serde(alias = "istZugestellt")]
    is_delivered: bool,
    #[serde(alias = "eigenschaften")]
    properties: HashMap<String, String>,
    #[serde(alias = "sendungsverlauf")]
    shipment_history: ShipmentHistory,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipmentHistory {
    #[serde(alias = "datumAktuellerStatus")]
    date_of_current_status: String,
    #[serde(alias = "aktuellerStatus")]
    current_status: String,
    #[serde(alias = "kurzStatus")]
    short_status: String,
    events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    #[serde(alias = "datum")]
    date: String,
    #[serde(alias = "ort")]
    location: Option<String>,
    #[serde(alias = "status")]
    status: String,
}

pub fn parse(payload: &str) -> Result<Content, Box<dyn std::error::Error>> {
    let parsed: Content = serde_json::from_str(payload)?;
    Ok(parsed)
}

pub async fn get_shipment_data(id: &str) -> Result<Content, Box<dyn std::error::Error>> {
    let params = [("piececode", id), ("language", "en")];
    let client = reqwest::Client::new();

    let resp = client
        .get(URI_API)
        .query(&params)
        .send()
        .await?
        .text()
        .await?;

    let parsed = parse(&resp)?;
    Ok(parsed)
}
