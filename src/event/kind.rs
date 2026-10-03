use serde::{Serialize, Serializer};

#[derive(Debug)]
pub enum Kind {
    SetMetadata,     // Kind 0
    Text,            // Kind 1
    RecommendServer, // Kind 2
}

impl Serialize for Kind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(match self {
            Kind::SetMetadata => 0,
            Kind::Text => 1,
            Kind::RecommendServer => 2,
        })
    }
}
