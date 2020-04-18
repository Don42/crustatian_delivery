use std::fs::File;
use std::io::Read;

mod provider;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("./state.json")?;
    let mut payload = String::new();
    file.read_to_string(&mut payload)?;
    let parsed = provider::dhl::parse(&payload)?;
    println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
    Ok(())
}

#[tokio::main]
async fn _main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    args.next();
    let id: &str = &args.next().unwrap();
    let parsed = provider::dhl::get_shipment_data(id).await?;
    println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
    Ok(())
}
