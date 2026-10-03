use serde::{Serialize, Serializer, ser::SerializeTuple};

#[derive(Debug)]
pub enum TagId {
    PubKey, // p
    Event,  // e
    Unknown,
}

#[derive(Debug)]
pub struct Tag(TagId, String, Option<String>);

impl Serialize for Tag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // serialize tag
        let mut tup = if let Some(_) = &self.2 {
            serializer.serialize_tuple(3)?
        } else {
            serializer.serialize_tuple(2)?
        };

        // serialize tag "id"
        tup.serialize_element(match self.0 {
            TagId::PubKey => "p",
            TagId::Event => "e",
            TagId::Unknown => "unknown",
        })?;

        // serialize tag "payload"
        tup.serialize_element(&self.1)?;

        // serialize recommended relay if present
        if let Some(relay) = &self.2 {
            tup.serialize_element(&relay)?;
        }

        tup.end()
    }
}
