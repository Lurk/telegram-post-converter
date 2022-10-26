mod telegram_export;
use telegram_export::{ImportData, Messages, Text};

fn main() {
    let import: ImportData = ImportData::from_file("./data/result.json");
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
                        if !artist_and_album.is_empty() && v.text.trim_start().starts_with('-') {
                            let message_parts: Vec<&str> =
                                v.text.trim_start().split('\n').collect();
                            if let Some(album) = message_parts.first() {
                                artist_and_album.push_str(album);
                            }
                        }
                    }
                    Text::TextLink(_) => {
                        has_links = true;
                    }
                    Text::Link(_) => {
                        has_links = true;
                    }
                }
            }
            if !artist_and_album.is_empty() & has_links {
                println!("{}", artist_and_album);
            }
        }
    }
}
