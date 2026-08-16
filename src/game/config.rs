use toml;
use serde::Deserialize;
use crate::hexcolor::HexColor;

#[derive(Clone, Debug, Deserialize)]
pub struct Window {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub area_color: HexColor,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Title {
    pub y_pos: f32,
    pub font_face: String,
    pub font_size: f32,
    pub font_color: HexColor,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Grid {
    pub y_pos: f32,
    pub num_rows: usize,
    pub num_cols: usize,
    pub box_size: usize,
    pub box_gap: usize,
    pub font_face: String,
    pub font_size: f32,
    pub color_area: HexColor,
    pub color_border: HexColor,
    pub color_emptybox: HexColor,
    pub color_noletters: HexColor,
    pub color_wrongpos: HexColor,
    pub color_rightpos: HexColor,
    pub color_answer: HexColor,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StatusLine {
    pub height: f32,
    pub font_face: String,
    pub font_size: f32,
    pub font_color: HexColor,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Message {
    pub title: String,
    pub text: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Messages {
    pub starting: Message,
    pub playing: Message,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub window: Window,
    pub title: Title,
    pub grid: Grid,
    pub status_line: StatusLine,
    pub messages: Messages,
}

const WORDLE_TOML: &str = include_str!("../../Wordle.toml"); 

impl Config {
    pub fn new() -> Self {
        let config: Config = toml::from_str(WORDLE_TOML).unwrap();
        config
    }
}
