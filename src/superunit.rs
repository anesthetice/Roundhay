use crate::unit::{
    Unit,
    Encoding,
    Resolution,
    Language,
};
use serde::{Serialize, Deserialize};
use serde_json;
use tokio::{fs::OpenOptions, io::AsyncReadExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Superunit {
    pub units: Vec<Unit>
}

impl Superunit {
    pub fn new() -> Self {
        Self { units: Vec::new() }
    }
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        serde_json::from_slice(data).ok()
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec_pretty(self).unwrap()
    }
    pub async fn load() -> Self {
        if let Ok(mut file) = OpenOptions::new().read(true).open("./index.json").await {
            let mut buffer: Vec<u8> = Vec::new();
            if let Ok(n) = file.read_to_end(&mut buffer).await {
                if let Some(superunit) = Self::from_bytes(&buffer) {
                    return superunit
                }
            }
        }
        Self::new()
    }
    pub fn to_html_string(&self) -> String {
        let mut html: String = String::from("<table><thead><tr><th>title</th><th>year</th><th>languages</th><th>subtitles</th><th>resolution</th><th>encoding</th><th>size</th><th><download></th></tr></thead><tbody>");
        self.units.iter().for_each(|unit| {
            html.push_str("<tr>");
            
            html.push_str(&format!("<td><span title=\"{}\">{}</span></td>", unit.description, unit.title));

            html.push_str(&format!("<td>{}</td>", unit.year));

            let mut languages: String = String::new();
            let length: usize = unit.languages.len();
            if length > 1 {
                for (index, language) in unit.languages.iter().enumerate() {
                    languages.push_str(&language.to_string());
                    if index != length-1 {languages.push_str(", ")}
                }
            } else if length == 1 {languages.push_str(&unit.languages[0].to_string());
            } else {languages.push_str("?")}
            html.push_str(&format!("<td>{}</td>", languages));

            let mut subtitles: String = String::new();
            let length: usize = unit.subtitles.len();
            if length > 1 {
                for (index, subtitle) in unit.subtitles.iter().enumerate() {
                    subtitles.push_str(&subtitle.to_string());
                    if index != length-1 {subtitles.push_str(", ")}
                }
            } else if length == 1 {subtitles.push_str(&unit.subtitles[0].to_string());
            } else {subtitles.push_str("?")}
            html.push_str(&format!("<td>{}</td>", subtitles));

            html.push_str(&format!("<td>{}</td>", unit.resolution.to_string()));

            html.push_str(&format!("<td>{}</td>", unit.encoding.to_string()));

            html.push_str(&format!("<td>{} MB</td>", unit.size));

            html.push_str(&format!("<td><a href=\"/download/{}\">⬇</a></td>", unit.path.to_str().unwrap_or("error")));

            html.push_str("</tr>");
        });
        html.push_str("</tbody></table>");
        html
    }
}


