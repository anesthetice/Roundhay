use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    title: String,
    description: String,
    languages: Vec<Language>,
    subtitles: Vec<Language>,
    resolution: (u16, u16),
    encoding: Encoding,
    size: usize,
    paths: Vec<PathBuf>
}

impl Unit {
    pub fn from_bytes(s: &[u8]) -> Option<Self> {
        serde_json::from_slice(s).ok()
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec_pretty(self).unwrap()
    }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    width: u16,
    height: u16,
}
