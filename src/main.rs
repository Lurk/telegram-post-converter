mod export;
mod telegram_export;

use cloudinary::{upload::result::UploadResult, upload::UploadOptions, Cloudinary};
use convert_case::{Case, Casing};
use dotenv::dotenv;
use export::{Album, AlbumWithSlug, Export, ExportAlbumWithSlug};
use telegram_export::{ImportData, Messages, Text};
use tokio::{self};

fn clean_artist_name(mut hashtag: String) -> String {
    if hashtag.starts_with('#') {
        hashtag.remove(0);
    }
    hashtag.from_case(Case::Pascal).to_case(Case::Title)
}

fn convert() -> Result<(), std::io::Error> {
    let import: ImportData = ImportData::from_file("./data/result.json");
    let mut export: Export = Export { albums: vec![] };
    for message in import.messages {
        if let Messages::Message(user_message) = message {
            let mut album = Album::new(user_message.date, user_message.photo);
            let mut has_links: bool = false;
            for text_entity in user_message.text_entities {
                match text_entity {
                    Text::Hashtag(v) => {
                        let artist_name = clean_artist_name(v.text);
                        album.artist.push(artist_name.clone());
                        album.tags.push(artist_name);
                    }
                    Text::Plain(v) => {
                        let text = v.text.trim_start();
                        if !album.artist.is_empty() && text.starts_with('-') {
                            let message_parts: Vec<&str> = text.split('\n').collect();
                            if let Some((head, tail)) = message_parts.split_first() {
                                let mut album_without_minus = head.to_string();
                                album_without_minus.remove(0);
                                album.name = Some(album_without_minus.trim().to_string());
                                album.description.push_str(&tail.join("\n"));
                            }
                        } else {
                            album.description.push_str(text)
                        }
                    }
                    Text::TextLink(link) => {
                        album.links.insert(link.text, link.href);
                        has_links = true;
                    }
                    Text::Link(link) => {
                        album.links.insert(link.text.clone(), link.text);
                        has_links = true;
                    }
                }
            }
            if !album.artist.is_empty() && has_links {
                export.albums.push(album);
            }
        }
    }
    export.save("./data/export.json")
}

async fn upload(mut album: Album) -> Album {
    let cloudinary = Cloudinary::new(
        dotenv::var("api_key").unwrap(),
        dotenv::var("cloud_name").unwrap(),
        dotenv::var("api_secret").unwrap(),
    );
    let public_id = format!(
        "{} {}",
        album.artist.join(" "),
        album.name.as_ref().unwrap()
    );
    let options = UploadOptions::new()
        .add_tags(&["music".to_string()])
        .set_folder("music".to_string())
        .set_public_id(slug::slugify(public_id.to_case(Case::Pascal)));

    let mut path = "./data/".to_string();
    path.push_str(&album.image.unwrap());
    match cloudinary.upload_image(path, &options).await {
        UploadResult::Succes(result) => album.image = Some(result.url),
        UploadResult::Error(e) => panic!("error {:?} while trying to upload {:?}", e, options),
    }

    album.clone()
}

async fn convert_to_data() {
    dotenv().ok();
    let export = Export::from_file("./data/export.json");
    let mut data = ExportAlbumWithSlug::from_file("./data/data.json");
    let albums: Vec<Album> = export
        .albums
        .into_iter()
        .filter(|album| album.ready)
        .collect();

    let albums = futures::future::join_all(albums.iter().cloned().map(upload)).await;
    data.albums.append(
        &mut albums
            .iter()
            .cloned()
            .map(|a| AlbumWithSlug::from_album(&a))
            .collect(),
    );
    match data.save("./data/data.json") {
        Ok(_) => {
            println!("done.json is saved");
            let mut export = Export::from_file("./data/export.json");
            let albums: Vec<Album> = export
                .albums
                .into_iter()
                .filter(|album| !album.ready)
                .collect();
            export.albums = albums;
            match export.save("./data/export.json") {
                Ok(_) => println!("export.json is saved"),
                Err(err) => println!("{}", err),
            }
        }
        Err(err) => println!("{}", err),
    }
}

#[tokio::main]
async fn main() {
    convert_to_data().await
}
