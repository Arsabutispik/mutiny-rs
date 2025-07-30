use std::fmt;
use serde::{Serialize, Deserialize, Deserializer, de};
use serde::de::{SeqAccess, Visitor};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub _id: String,
    pub username: String,
    pub discriminator: String,
    pub display_name: Option<String>,
    pub avatar: Option<Avatar>,
    #[serde(default, deserialize_with = "deserialize_relations")]
    pub relationship: Option<Vec<Relation>>,
    pub badges: Option<u8>,
    pub status: Option<crate::model::ready::Status>,
    pub flags: Option<usize>,
    pub privileged: Option<bool>,
    pub bot: Option<Bot>,
    pub online: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchProfile {
    pub background: Option<Background>,
    pub content: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Background {
    pub content_type: String,
    pub filename: String,
    pub metadata: Metadata,
    pub size: usize,
    pub tag: String,
    pub _id: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub height: usize,
    #[serde(rename = "type")]
    pub _type: String,
    pub width: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchUser {
    pub _id: String,
    pub username: String,
    pub avatar: Option<Avatar>,
    pub relations: Option<Vec<Relation>>,
    pub badges: usize,
    pub status: Option<Status>,
    pub relationship: String,
    pub online: bool,
    pub flags: Option<usize>,
    pub bot: Option<Bot>
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bot {
    pub owner: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub text: Option<String>,
    pub presence: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Avatar {
    pub _id: String,
    pub tag: String,
    pub filename: String,
    pub metadata: Metadata,
    pub content_type: String,
    pub size: usize,
    pub deleted: Option<bool>,
    pub reported: Option<bool>,
    pub message_id: Option<String>,
    pub user_id: Option<String>,
    pub server_id: Option<String>,
    pub object_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Relation {
Object { _id: String, status: String },
StatusOnly(String),
}
fn deserialize_relations<'de, D>(deserializer: D) -> Result<Option<Vec<Relation>>, D::Error>
where
    D: Deserializer<'de>,
{
    struct RelationsVisitor;

    impl<'de> Visitor<'de> for RelationsVisitor {
        type Value = Option<Vec<Relation>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("null, a string, or a sequence of relations")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // Single string treated as one Relation::StatusOnly
            Ok(Some(vec![Relation::StatusOnly(v.to_string())]))
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // Same as visit_str
            Ok(Some(vec![Relation::StatusOnly(v)]))
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut relations = Vec::new();

            while let Some(elem) = seq.next_element::<Relation>()? {
                relations.push(elem);
            }
            Ok(Some(relations))
        }
    }

    deserializer.deserialize_any(RelationsVisitor)
}