mod telegram_export;
use telegram_export::{ImportData, Messages, Text};
use convert_case::{Case, Casing};

#[derive(Debug)]
struct ExportData {
    artist: Option<String>,
    album: Option<String>,
    tags: Vec<String>,
}

impl ExportData {
    pub fn new() -> Self {
        ExportData {
            artist: None,
            album: None,
            tags: vec![],
        }
    }
}

fn clean_artist_name(mut hashtag: String) -> String {
    if hashtag.starts_with('#'){
        hashtag.remove(0);
    }
    hashtag.from_case(Case::Pascal).to_case(Case::Title)
}

fn main() {
    let import: ImportData = ImportData::from_file("./data/result.json");
    for message in import.messages {
        if let Messages::Message(user_message) = message {
            let mut export_data = ExportData::new();
            let mut has_links: bool = false;
            for text_entity in user_message.text_entities {
                match text_entity {
                    Text::Hashtag(v) => {
                        let artist_name = clean_artist_name(v.text);
                        if export_data.artist.is_none() {
                            export_data.artist = Some(artist_name.clone());
                        }
                        export_data.tags.push(artist_name);
                    }
                    Text::Plain(v) => {
                        let text = v.text.trim_start();
                        if export_data.artist.is_some() && text.starts_with('-') {
                            let message_parts: Vec<&str> = text.split('\n').collect();
                            if let Some(album) = message_parts.first() {
                                let mut album_without_minus = album.to_string();
                                album_without_minus.remove(0);
                                export_data.album = Some(album_without_minus.trim().to_string());
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
            if export_data.artist.is_some() && has_links {
                println!("{:?}", export_data);
            }
        }
    }
}
