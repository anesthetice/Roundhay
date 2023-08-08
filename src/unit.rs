use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    // the title of the movie or show should be formatted as such: [NAME] - [SEASON][EPISODE]
    // i.e. Kingdom - s02e05
    pub title: String,
    pub description: String,
    pub year: u16,
    pub languages: Vec<Language>,
    pub subtitles: Vec<Language>,
    pub resolution: Resolution,
    pub encoding: Encoding,
    // the size of the media in megabytes (MB)
    pub size: f64,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Language {
    English,
    French,
    German,
    Italian,
    Japanese,
    Other(String),
}

impl Language {
    pub fn to_string(&self) -> String {
        match self {
            Self::English => "En".to_string(),
            Self::French => "Fr".to_string(),
            Self::German => "De".to_string(),
            Self::Italian => "It".to_string(),
            Self::Japanese => "Jp".to_string(),
            Self::Other(string) => {
                if string.len() > 1 {
                    string[..2].to_string()
                } else {
                    "??".to_string()
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Encoding {
    H264,
    H265,
    VP8,
    VP9,
    Other,
}

impl Encoding {
    pub fn to_string(&self) -> String {
        match self {
            Self::H264 => "H264".to_string(),
            Self::H265 => "H265".to_string(),
            Self::VP8 => "VP8".to_string(),
            Self::VP9 => "VP9".to_string(),
            Self::Other => "Other".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    width: u16,
    height: u16,
}

impl Resolution {
    pub fn new(width: u16, height: u16) -> Self {
        Self {width, height}
    }
    pub fn to_string(&self) -> String {
        format!("{}x{}", self.width, self.height)
    }
}

