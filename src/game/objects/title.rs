use raylib::drawing::RaylibDraw;
use raylib::color::Color;
use crate::game::GameContext;

use crate::game::GameObject;

pub struct Title<'a> {
    ctx: &'a GameContext<'a>,
    title: String,
}

impl<'a> Title<'a> {
    pub fn new(ctx: &'a GameContext) -> Self {
        Self {
            ctx: ctx,
            title: "Wordle".to_string(),
        }
    }
}

impl<'a> GameObject for Title<'a> {
    fn update(&mut self, _d: &mut raylib::prelude::RaylibDrawHandle) {     
    }

    fn draw(&mut self, d: &mut raylib::prelude::RaylibDrawHandle) {
        let cfg = &self.ctx.cfg;
        let pos_x = (d.get_screen_width() - d.measure_text(&self.title, cfg.title.font_size as i32)) /2;
        d.draw_text(
                &self.title,
                pos_x,
                cfg.title.y_pos as i32,
                cfg.title.font_size as i32,
                Color::from(&cfg.title.font_color),
        );
    }

}
