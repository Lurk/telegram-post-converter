use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use slug::slugify;
use std::{collections::HashMap, fs};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Album {
    pub artist: Vec<String>,
    pub name: Option<String>,
    pub description: String,
    pub tags: Vec<String>,
    #[serde(with = "ts_milliseconds")]
    pub date: DateTime<Utc>,
    pub image: Option<String>,
    pub links: HashMap<String, String>,
    pub ready: bool,
}

impl Album {
    pub fn new(date: DateTime<Utc>, image: Option<String>) -> Self {
        Album {
            artist: vec![],
            name: None,
            description: String::new(),
            tags: vec![],
            date,
            image,
            links: HashMap::new(),
            ready: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Export {
    pub albums: Vec<Album>,
}

impl Export {
    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        fs::write(path, serde_json::to_string(self).unwrap())
    }
    pub fn from_file(path: &str) -> Self {
        let data = fs::read_to_string(path).unwrap();
        serde_json::from_str(&data).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlbumWithSlug {
    pub artist: Vec<String>,
    pub slug: String,
    pub name: Option<String>,
    pub description: String,
    pub tags: Vec<String>,
    #[serde(with = "ts_milliseconds")]
    pub date: DateTime<Utc>,
    pub image: Option<String>,
    pub links: HashMap<String, String>,
    pub ready: bool,
}

impl AlbumWithSlug {
    pub fn new(date: DateTime<Utc>, image: Option<String>, slug: String) -> Self {
        AlbumWithSlug {
            artist: vec![],
            name: None,
            slug,
            description: String::new(),
            tags: vec![],
            date,
            image,
            links: HashMap::new(),
            ready: false,
        }
    }

    pub fn from_album(album: &Album) -> Self {
        AlbumWithSlug {
            artist: album.artist.clone(),
            slug: slugify(format!(
                "{} by {}",
                album.name.as_ref().unwrap(),
                album.artist.join(" and ")
            )),
            name: album.name.clone(),
            description: album.description.clone(),
            tags: album.tags.clone(),
            date: album.date,
            image: album.image.clone(),
            links: album.links.clone(),
            ready: album.ready,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportAlbumWithSlug {
    pub albums: Vec<AlbumWithSlug>,
}

impl ExportAlbumWithSlug {
    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        fs::write(path, serde_json::to_string(self).unwrap())
    }
    pub fn from_file(path: &str) -> Self {
        let data = fs::read_to_string(path).unwrap();
        serde_json::from_str(&data).unwrap()
    }
}
