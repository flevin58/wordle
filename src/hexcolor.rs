use serde::Deserialize;
use raylib::color::Color;

#[derive(Clone, Debug, Deserialize)]
pub struct HexColor(String);

impl From<Color> for HexColor {
    fn from(c: Color) -> Self {
        Self(format!("#{:2x}{:2x}{:2x}", c.r, c.g, c.b))
    }
}

impl From<String> for HexColor {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<HexColor> for Color {
    fn from(hc: HexColor) -> Color {
        let s: String = hc.0;
        Color {
            r: u8::from_str_radix(&s[1..3], 16).unwrap(),
            g: u8::from_str_radix(&s[3..5], 16).unwrap(),
            b: u8::from_str_radix(&s[5..7], 16).unwrap(),
            a: 255,
        }
    }
}

impl From<&HexColor> for Color {
    fn from(hc: &HexColor) -> Color {
        let s: String = hc.0.clone();
        Color {
            r: u8::from_str_radix(&s[1..3], 16).unwrap(),
            g: u8::from_str_radix(&s[3..5], 16).unwrap(),
            b: u8::from_str_radix(&s[5..7], 16).unwrap(),
            a: 255,
        }
    }
}
