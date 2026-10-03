mod data;
mod kind;
mod sign;
mod tag;

pub use data::EventData;
pub use kind::Kind;
pub use sign::{Config, Signer};
pub use tag::{Tag, TagId};

use serde::Serialize;

// Event is the NOSTR main data structure
#[derive(Serialize, Debug)]
pub struct Event {
    pub id: String,
    pub pubkey: String,
    pub sig: String,

    #[serde(flatten)]
    pub data: EventData,
}
