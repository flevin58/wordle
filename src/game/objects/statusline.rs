use raylib::drawing::RaylibDraw;
use raylib::prelude::RaylibDrawHandle;
use crate::game::{GameObject, GameContext, GameState};

//
// This game object implements the status line
// whic consists od two text lines that contain a message.
//
pub struct StatusLine<'a> {
    pub ctx: &'a GameContext<'a>,
    pub title: &'a str,
    pub text: &'a str,
}

impl<'a> StatusLine<'a> {
    pub fn new(ctx: &'a GameContext) -> Self {
        Self {
            ctx: ctx,
            title: "",
            text: "",
        }
    }
}

impl<'a> GameObject for StatusLine<'a> {
    fn update(&mut self, _d: &mut RaylibDrawHandle) {
        let cfg = &self.ctx.cfg;
        (self.title, self.text) = match self.ctx.state {
            GameState::Starting => {
                (
                    &cfg.messages.starting.title,
                    &cfg.messages.starting.text
                )
            }
            _ => todo!("Implement the other GameState branches")
        };
    }
    
    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        
        macro_rules! draw_centered {
            ($line:expr) => {
                {
                    let msg = if $line == 1 { self.title } else { self.text };
                    let x_pos = 10 + (self.ctx.cfg.window.width - d.measure_text(msg, self.ctx.cfg.status_line.font_size as i32)) / 2;
                    let y_pos = self.ctx.cfg.window.height - self.ctx.cfg.status_line.height as i32 + 10 + 30 * ($line-1);
                    d.draw_text(
                        msg,
                        x_pos,
                        y_pos,
                        self.ctx.cfg.status_line.font_size as i32,
                        &self.ctx.cfg.status_line.font_color
                    );
                }
            };
        }

        draw_centered!(1);
        draw_centered!(2);
    }
}
