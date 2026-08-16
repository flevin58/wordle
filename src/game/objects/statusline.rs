use raylib::drawing::RaylibDraw;
use raylib::prelude::RaylibDrawHandle;
use crate::{game::{GameObject, GameState}, get_read_context};

//
// This game object implements the status line
// whic consists od two text lines that contain a message.
//
pub struct StatusLine {
    pub title: String,
    pub text: String,
}

impl StatusLine {
    pub fn new() -> Self {
        Self {
            title: String::from(""),
            text: String::from(""),
        }
    }
}

impl GameObject for StatusLine {

    fn update(&mut self, _d: &mut RaylibDrawHandle) {
        let ctx = get_read_context!();
        (self.title, self.text) = match ctx.state {
            GameState::Starting => {
                (
                    ctx.cfg.messages.starting.title.clone(),
                    ctx.cfg.messages.starting.text.clone()
                )
            }
            GameState::Playing => {
                (
                    ctx.cfg.messages.playing.title.clone(),
                    ctx.cfg.messages.playing.text.clone()
                )
            }
            _ => todo!("Implement the other GameState branches")
        };
    }
    
    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        let ctx = get_read_context!();

        macro_rules! draw_centered {
            ($line:expr) => {
                {
                    let msg = if $line == 1 { &self.title } else { &self.text };
                    let x_pos = 10 + (ctx.cfg.window.width - d.measure_text(&msg, ctx.cfg.status_line.font_size as i32)) / 2;
                    let y_pos = ctx.cfg.window.height - ctx.cfg.status_line.height as i32 + 10 + 30 * ($line-1);
                    d.draw_text(
                        &msg,
                        x_pos,
                        y_pos,
                        ctx.cfg.status_line.font_size as i32,
                        &ctx.cfg.status_line.font_color
                    );
                }
            };
        }

        draw_centered!(1);
        draw_centered!(2);
    }
}
