//
// This game object implements the wordle 5 x 6 grid
//
use crate::game::{GameObject, config::Config};
use crate::hexcolor::HexColor;
use rand::RngExt;
use raylib::ffi::Rectangle;
use raylib::prelude::*;

pub struct Grid<'a> {
    cfg: &'a Config,
    data: Vec<u8>,
    cursor: Vector2,
    rect: Rectangle,
}

impl<'a> Grid<'a> {
    pub fn new(cfg: &'a Config) -> Self {
        let grid_width = (cfg.grid.num_cols * cfg.grid.box_size + (cfg.grid.num_cols - 1) * cfg.grid.box_gap) as f32;
        let grid_height = (cfg.grid.num_rows * cfg.grid.box_size + (cfg.grid.num_rows - 1) * cfg.grid.box_gap) as f32;
        let grid_x = (cfg.window.width as f32 - grid_width) / 2.;
        let grid_y = 40.;
        Self {
            cfg: cfg,
            data: Vec::with_capacity(cfg.grid.num_rows as usize * cfg.grid.num_rows as usize),
            cursor: Vector2 { x: 0., y: 0. },
            rect: Rectangle{x: grid_x, y: grid_y, width: grid_width, height: grid_height},
        }
    }
}

impl<'a> GameObject for Grid<'a> {
    fn update(&mut self, d: &mut RaylibDrawHandle) {
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(
            self.rect.x as i32,
            self.rect.y as i32,
            self.rect.width as i32,
            self.rect.height as i32,
            Color::from(&self.cfg.grid.color_border)
        );
    }
}
