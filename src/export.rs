use std::{collections::HashMap, fs};

use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
