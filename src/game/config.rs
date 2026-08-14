use toml;
use serde::Deserialize;
use crate::hexcolor::HexColor;

#[derive(Debug, Deserialize)]
pub struct Window {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub area_color: HexColor,
}

#[derive(Debug, Deserialize)]
pub struct Title {
    pub height: i32,
    pub font_face: String,
    pub font_size: f32,
    pub font_color: HexColor,
    pub area_color: HexColor,
}

#[derive(Debug, Deserialize)]
pub struct Grid {
    pub num_rows: i32,
    pub num_cols: i32,
    pub box_size: i32,
    pub box_gap: i32,
    pub font_face: String,
    pub font_size: f32,
    pub color_border: HexColor,
    pub color_emptybox: HexColor,
    pub color_noletters: HexColor,
    pub color_wrongpos: HexColor,
    pub color_rightpos: HexColor,
    pub color_answer: HexColor,
}

#[derive(Debug, Deserialize)]
pub struct StatusLine {
    pub height: i32,
    pub area_color: HexColor,
    pub text_color: HexColor,
    pub font_face: String,
    pub font_size: f32,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub window: Window,
    pub title: Title,
    pub grid: Grid,
    pub status_line: StatusLine,
}

const WORDLE_TOML: &str = include_str!("../../Wordle.toml"); 

impl Config {
    pub fn new() -> Self {
        let config: Config = toml::from_str(WORDLE_TOML).unwrap();
        config
    }
}
