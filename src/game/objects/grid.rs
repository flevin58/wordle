pub mod cell;
//
// This game object implements the wordle 5 x 6 grid
//
use crate::game::{GameContext, GameObject};
use raylib::ffi::Rectangle;
use raylib::prelude::*;

pub struct Grid <'a> {
    ctx: &'a GameContext<'a>,
    data: Vec<u8>,
    cursor: Vector2,
    rect: Rectangle,
}

impl<'a> Grid<'a> {
    pub fn new(ctx: &'a GameContext) -> Self {
        let cfg = &ctx.cfg.grid;
        let grid_width = (cfg.num_cols * cfg.box_size + (cfg.num_cols - 1) * cfg.box_gap) as f32;
        let grid_height = (cfg.num_rows * cfg.box_size + (cfg.num_rows - 1) * cfg.box_gap) as f32;
        let grid_x = ( ctx.cfg.window.width as f32 - grid_width) / 2.;
        Self {
            ctx: ctx,
            data: Vec::with_capacity(cfg.num_rows as usize * cfg.num_rows as usize),
            cursor: Vector2 { x: 0., y: 0. },
            rect: Rectangle{x: grid_x, y: cfg.y_pos, width: grid_width, height: grid_height},
        }
    }
}

impl<'a> GameObject for Grid<'a> {
    fn update(&mut self, _d: &mut RaylibDrawHandle) {
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(
            self.rect.x as i32,
            self.rect.y as i32,
            self.rect.width as i32,
            self.rect.height as i32,
            Color::from(&self.ctx.cfg.grid.color_area)
        );
    }
}
