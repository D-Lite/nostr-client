// https://nickmonad.blog/2023/building-nostr-client-index-0/

use clap::Parser;
use std::time::{SystemTime, UNIX_EPOCH};

use nostr_client::{Config, EventData, Kind, Signer};

fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let config = Config::parse();

    let signer = Signer::from_config(&config)?;

    let created_at = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    let data = EventData {
        created_at,
        kind: Kind::Text,
        tags: vec![],
        content: "hello world! from this side of the world".into(),
    };

    let event = data.sign(&signer)?;
    println!("{:#?}", event);

    Ok(())
}
