use chrono::prelude::*;
use serde::Deserialize;
use serde_aux::prelude::*;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Clone, Debug)]
struct SimpleText {
    text: String,
}

#[derive(Deserialize, Clone, Debug)]
struct TextLink {
    text: String,
    href: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Text {
    Hashtag(SimpleText),
    Plain(SimpleText),
    TextLink(TextLink),
    Link(SimpleText),
}

#[derive(Deserialize, Debug, Clone)]
struct ServiceMessage {
    id: usize,
    #[serde(
        rename = "date_unixtime",
        deserialize_with = "deserialize_datetime_utc_from_milliseconds"
    )]
    date: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone)]
struct UserMessage {
    id: usize,
    #[serde(
        rename = "date_unixtime",
        deserialize_with = "deserialize_datetime_utc_from_milliseconds"
    )]
    date: DateTime<Utc>,
    photo: Option<String>,
    witdh: Option<usize>,
    height: Option<usize>,
    text_entities: Vec<Text>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Messages {
    Service(ServiceMessage),
    Message(UserMessage),
}

#[derive(Deserialize, Debug)]
struct ImportData {
    name: String,
    #[serde(rename = "type")]
    channel_type: String,
    id: usize,
    messages: Vec<Messages>,
}

fn main() {
    let mut file = File::open("./data/result.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let import: ImportData = serde_json::from_str(&data).expect("JSON was not well-formatted");
    for message in import.messages {
        if let Messages::Message(user_message) = message {
            let mut artist_and_album: String = String::new();
            let mut has_links: bool = false;
            for text_entity in user_message.text_entities {
                match text_entity {
                    Text::Hashtag(v) => {
                        artist_and_album.push_str(&format!("{} ", v.text));
                    }
                    Text::Plain(v) => {
                        if !artist_and_album.is_empty() {
                            if v.text.trim_start().starts_with("-") {
                                let message_parts: Vec<&str> =
                                    v.text.trim_start().split("\n").collect();
                                if let Some(album) = message_parts.get(0) {
                                    artist_and_album.push_str(&album);
                                }
                            }
                        }
                    }
                    Text::TextLink(_) => {has_links=true;}
                    Text::Link(_) => {has_links=true;}
                }
            }
            if !artist_and_album.is_empty() & has_links{
                println!("{}", artist_and_album);
            }
        }
    }
}
