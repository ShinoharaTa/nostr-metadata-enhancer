use nostr_sdk::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::Duration;
// use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

fn load_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string(file_path)?;
    let config: Config = serde_json::from_str(&file_content)?;
    Ok(config)
}

#[derive(Serialize, Deserialize)]
struct Config {
    keys: Vec<String>,
    relays: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config_path = "./config.json";
    let config = load_config(config_path)?;
    let client = Client::default();
    client.add_relays(config.relays).await?;
    client.connect().await;

    for key_str in config.keys {
        let my_keys = Keys::from_sk_str(&key_str)?;
        let bech32_pubkey: String = my_keys.public_key().to_bech32()?;
        println!("Bech32 PubKey: {}", bech32_pubkey);
        println!("PubKey: {}", my_keys.public_key());
        let filters = Filter::new()
            .author(my_keys.public_key())
            .kind(Kind::Metadata)
            .limit(1);
        let events = client
            .get_events_of(vec![filters], Some(Duration::from_secs(10)))
            .await?;
        if let Some(latest_event) = events.get(0) {
            let content = &latest_event.content;
            println!("{content:#?}");
            let event = EventBuilder::new(Kind::Metadata, content, []).to_event(&my_keys)?;
            let _result = client.send_event(event).await?;
        }
    }

    Ok(())
}
