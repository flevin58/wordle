use raylib::drawing::RaylibDraw;
use raylib::prelude::RaylibDrawHandle;

use crate::game::GameObject;
use crate::get_read_context;

pub struct Title(String);

impl Title {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl GameObject for Title {
    fn update(&mut self, _d: &mut RaylibDrawHandle) {}

    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        let ctx = get_read_context!();
        let cfg = &ctx.cfg.title;
        let pos_x = (d.get_screen_width() - d.measure_text(&self.0, cfg.font_size as i32)) /2;
        d.draw_text(
                &self.0,
                pos_x,
                cfg.y_pos as i32,
                cfg.font_size as i32,
                &cfg.font_color,
        );
    }
}
