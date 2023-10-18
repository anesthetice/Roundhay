use crate::{traits::WebContent, unit::Unit};
use serde::{Deserialize, Serialize};
use tokio::{fs::OpenOptions, io::AsyncReadExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Superunit {
    pub units: Vec<Unit>,
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
            if (file.read_to_end(&mut buffer).await).is_ok() {
                if let Some(superunit) = Self::from_bytes(&buffer) {
                    return superunit;
                }
            }
        }
        Self::new()
    }
}

impl WebContent for Superunit {
    fn as_html_string(&self) -> String {
        let mut html: String = "    <table>
        <thead>
            <tr>
                <th>title</th>
                <th>year</th>
                <th>languages</th>
                <th>subtitles</th>
                <th>resolution</th>
                <th>encoding</th>
                <th>size [MB]</th>
                <th>download</th>
            </tr>
        </thead>
        <tbody>
".to_string();

        self.units
            .iter()
            .for_each(|unit| html.push_str(unit.as_html_string().as_ref()));

        let html: String = format!("{}        </tbody>
    </table>
", html);

        html
    }
}
