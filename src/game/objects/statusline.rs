use raylib::color::Color;
use raylib::drawing::RaylibDraw;
use crate::game::{GameObject, GameContext, GameState};

//
// This game object implements the status line
// whic consists od two text lines that contain a message.
//
pub struct StatusLine<'a> {
    pub ctx: &'a GameContext<'a>,
    pub hidden: bool,
}

impl<'a> StatusLine<'a> {
    pub fn new(ctx: &'a GameContext) -> Self {
        Self {
            ctx: ctx,
            hidden: false,
        }
    }
}

impl<'a> GameObject for StatusLine<'a> {
    fn update(&mut self, _d: &mut raylib::prelude::RaylibDrawHandle) {
    }

    fn draw(&mut self, d: &mut raylib::prelude::RaylibDrawHandle) {
        if self.hidden {
            return;
        }
        let cfg = &self.ctx.cfg;
        d.draw_rectangle(
            0,
            cfg.window.height - cfg.status_line.height,
            cfg.window.width,
            cfg.status_line.height,
            Color::from(&cfg.status_line.area_color),
        );
        let (title, text) = match self.ctx.state {
            GameState::Starting => {
                (
                    &self.ctx.cfg.messages.starting.title,
                    &self.ctx.cfg.messages.starting.text
                )
            }
            _ => (&String::from(""), &String::from(""))
        };

        macro_rules! center {
            ($text:ident) => {
                10 + (self.ctx.cfg.window.width - d.measure_text($text, self.ctx.cfg.status_line.font_size as i32)) / 2
            };
        }

        macro_rules! line_pos {
            ($num:expr) => {
                cfg.window.height - cfg.status_line.height + 10 + 30 * ($num-1)
            };
        }

        //let font_size = self.ctx.cfg.status_line.font_size as i32;
        //let title_x = 10 + (self.ctx.cfg.window.width - d.measure_text(title, font_size)) / 2;
        d.draw_text(
            title,
            center!(title),
            line_pos!(1),
            self.ctx.cfg.status_line.font_size as i32,
            &self.ctx.cfg.status_line.text_color
        );
        d.draw_text(
            text,
            center!(text),
            line_pos!(2),
            self.ctx.cfg.status_line.font_size as i32,
            &self.ctx.cfg.status_line.text_color
        );
    }
}
