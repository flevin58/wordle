use raylib::color::Color;
use raylib::drawing::RaylibDraw;
use crate::game::GameObject;
use crate::game::config::Config;
use crate::hexcolor::HexColor;

//
// This game object implements the status line
// whic consists od two text lines that contain a message.
//
pub struct StatusLine<'a> {
    pub cfg: &'a Config,
    pub hidden: bool,
}

impl<'a> StatusLine<'a> {
    pub fn new(cfg: &'a Config) -> Self {
        Self {
            cfg: cfg,
            hidden: false,
        }
    }
}

impl<'a> GameObject for StatusLine<'a> {
    fn update(&mut self, d: &mut raylib::prelude::RaylibDrawHandle) {
    }

    fn draw(&mut self, d: &mut raylib::prelude::RaylibDrawHandle) {
        if self.hidden {
            return;
        }
        d.draw_rectangle(
            0,
            self.cfg.window.height - self.cfg.status_line.height,
            self.cfg.window.width,
            self.cfg.status_line.height,
            Color::from(&self.cfg.status_line.area_color),
        );
    }
}
