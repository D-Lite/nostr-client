use anyhow::Ok;
use serde::Serialize;
use serde_json as json;
use sha2::{Digest, Sha256};

use super::{Event, Kind, Signer, Tag};

#[derive(Serialize, Debug)]
pub struct EventData {
    // pub pubkey: String,
    pub created_at: u64,
    pub kind: Kind,
    pub tags: Vec<Tag>,
    pub content: String,
}

impl EventData {
    pub fn id(&self, pubkey: &str) -> String {
        hex::encode(
            Sha256::new()
                .chain_update(format!(
                    // just lean on serde here as well, for each element.
                    // makes the format string nicer to look at
                    "[0,{},{},{},{},{}]",
                    json::to_string(&pubkey).unwrap(),
                    json::to_string(&self.created_at).unwrap(),
                    json::to_string(&self.kind).unwrap(),
                    json::to_string(&self.tags).unwrap(),
                    json::to_string(&self.content).unwrap(),
                ))
                .finalize(),
        )
    }

    pub fn sign(self, signer: &Signer) -> anyhow::Result<Event> {
        let pubkey = signer.public_key();
        let id = self.id(&pubkey);
        let sig = signer.sign(&id)?;

        Ok(Event {
            id,
            pubkey,
            sig,
            data: self,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event_id() {
        // sample event
        // https://www.nostr.guru/e/c8d24e78cfedd658688bdfc23a2f7049f0032989096f3e1c5df1e5585efaa393
        //
        let pubkey = "5fe74dc9a7349be18269007d8e7bdf7599869cb677fe3f2794ebd821f146fe81";

        let data = EventData {
            created_at: 1675631070,
            kind: Kind::Text,
            tags: vec![],
            content: "hello, nostr".into(),
        };

        assert_eq!(
            data.id(pubkey),
            "c8d24e78cfedd658688bdfc23a2f7049f0032989096f3e1c5df1e5585efaa393"
        );
    }
}
