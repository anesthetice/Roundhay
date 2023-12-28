use crate::traits::WebContent;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitSingle {
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

impl WebContent for UnitSingle {
    fn as_html_string(&self) -> String {
        let mut languages: String = String::new();
        match self.languages.len() {
            0 => {
                languages.push('?');
            }
            1 => {
                languages.push_str(&self.languages[0].as_string());
            }
            length => {
                for (index, language) in self.languages.iter().enumerate() {
                    languages.push_str(&language.as_string());
                    if index != length - 1 {
                        languages.push_str(", ")
                    }
                }
            }   
        }

        let mut subtitles: String = String::new();
        match self.subtitles.len() {
            0 => {
                subtitles.push('?');
            }
            1 => {
                subtitles.push_str(&self.subtitles[0].as_string());
            }
            length => {
                for (index, subtitle) in self.subtitles.iter().enumerate() {
                    subtitles.push_str(&subtitle.as_string());
                    if index != length - 1 {
                        subtitles.push_str(", ")
                    }
                }
            }
        }

        indoc::formatdoc! {
           "<tr>
                <td>
                    <span title=\"{}\">
                        {}
                    </span>
                </td>
                <td>
                    {}
                </td>
                <td>
                    {}
                </td>
                <td>
                    {}
                </td>
                <td>
                    {}
                </td>
                <td>
                    {}
                </td>
                <td>
                    {}
                </td>
                <td>
                    <a href=\"/res/{}\" download>
                        ⬇
                    </a>
                </td>
                <td>
                    <a href=\"/stream/?source={}\">
                        ▶
                    </a>
                </td>
            </tr>
",
            self.description,
            self.title,
            self.year,
            languages,
            subtitles,
            self.resolution.as_string(),
            self.encoding.as_string(),
            ryu::Buffer::new().format_finite(self.size),
            self.path.to_str().unwrap_or("error"),
            self.path.to_str().unwrap_or("error"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitGroup {
    pub title: String,
    pub description: String,
    pub year: u16,
    pub units: Vec<UnitSingle>,
}

impl WebContent for UnitGroup {
    fn as_html_string(&self) -> String {
        let mut string: String = indoc::formatdoc!(
           "<tr>
                <td>
                    <span title=\"{}\">
                        <u onclick=\"showhideElements('{}')\">
                            {}
                        </u>
                    </span>
                </td>
                <td>
                    {}
                </td>
                <td></td>
                <td></td>
                <td></td>
                <td></td>
                <td></td>
                <td></td>
                <td></td>
            </tr>
",
            self.description, self.title, self.title, self.year
        );

        self.units.iter().for_each(|unit| {
            string.push_str(
                unit.as_html_string()
                    .replace(
                        "<tr>",
                        format!("<tr class=\"hidden\" id=\"{}\">", self.title).as_ref(),
                    )
                    .as_ref(),
            );
        });
        string
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Unit {
    Single(UnitSingle),
    Group(UnitGroup),
}

impl WebContent for Unit {
    fn as_html_string(&self) -> String {
        match self {
            Self::Single(unitsingle) => unitsingle.as_html_string(),
            Self::Group(unitgroup) => unitgroup.as_html_string(),
        }
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
    pub fn as_string(&self) -> String {
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
    pub fn as_string(&self) -> String {
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
        Self { width, height }
    }
    pub fn _720p() -> Self {
        Self {
            width: 1280,
            height: 720
        }
    }
    pub fn _1080p() -> Self {
        Self {
            width: 1920,
            height: 1080,
        }
    }
    pub fn _1440p() -> Self {
        Self {
            width: 2560,
            height: 1440,
        }
    }
    pub fn _2160p() -> Self {
        Self {
            width: 2160,
            height: 3840,
        }
    }
    pub fn as_string(&self) -> String {
        format!("{}x{}", self.width, self.height)
    }
}
