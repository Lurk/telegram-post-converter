use chrono::prelude::*;
use serde::{Deserialize, Deserializer};
use serde_aux::prelude::*;
use std::{fs::File, io::Read};

pub fn deserialize_datetime_utc_from_seconds<'de, D>(
    deserializer: D,
) -> Result<chrono::DateTime<chrono::Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let number = deserialize_number_from_string::<i64, D>(deserializer)?;

    Ok(DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(number, 0),
        Utc,
    ))
}

#[derive(Deserialize, Clone, Debug)]
pub struct SimpleText {
    pub text: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TextLink {
    pub text: String,
    pub href: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Text {
    Hashtag(SimpleText),
    Plain(SimpleText),
    TextLink(TextLink),
    Link(SimpleText),
}

#[derive(Deserialize, Debug, Clone)]
pub struct ServiceMessage {
    pub id: usize,
    #[serde(
        rename = "date_unixtime",
        deserialize_with = "deserialize_datetime_utc_from_seconds"
    )]
    pub date: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserMessage {
    pub id: usize,
    #[serde(
        rename = "date_unixtime",
        deserialize_with = "deserialize_datetime_utc_from_seconds"
    )]
    pub date: DateTime<Utc>,
    pub photo: Option<String>,
    pub witdh: Option<usize>,
    pub height: Option<usize>,
    pub text_entities: Vec<Text>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Messages {
    Service(ServiceMessage),
    Message(UserMessage),
}

#[derive(Deserialize, Debug)]
pub struct ImportData {
    pub name: String,
    #[serde(rename = "type")]
    pub channel_type: String,
    pub id: usize,
    pub messages: Vec<Messages>,
}

impl ImportData {
    pub fn from_file(path: &str) -> Self {
        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        serde_json::from_str(&data).expect("JSON was not well-formatted")
    }
}
