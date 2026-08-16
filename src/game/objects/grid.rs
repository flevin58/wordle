pub mod cell;
//
// This game object implements the wordle 5 x 6 grid
//
use crate::game::GameObject;
use crate::game::objects::grid::cell::Cell;
use crate::get_read_context;
use raylib::ffi::KeyboardKey::{KEY_LEFT, KEY_RIGHT};
use raylib::ffi::Rectangle;
use raylib::prelude::*;

pub struct Grid {
    data: Vec<Vec<Cell>>,
    cursor_x: usize,
    cursor_y: usize,
    rect: Rectangle,
}

impl Grid {
    pub fn new() -> Self {
        let ctx = get_read_context!();
        let grid = &ctx.cfg.grid;
        let grid_width = (grid.num_cols * (grid.box_size + grid.box_gap) - grid.box_gap) as f32;
        let grid_height = (grid.num_rows * (grid.box_size + grid.box_gap) - grid.box_gap) as f32;
        let grid_x = ( ctx.cfg.window.width as f32 - grid_width) / 2.;
        let mut new_grid = Self {
            data: vec![vec![Cell::new(); 5]; 6],
            cursor_x: 0,
            cursor_y: 0,
            rect: Rectangle{x: grid_x, y: grid.y_pos, width: grid_width, height: grid_height},
        };
        for row in 0..ctx.cfg.grid.num_rows as usize {
            for col in 0..ctx.cfg.grid.num_cols as usize {
                new_grid.data[row][col].set_coords(row, col);
            }
        }
        new_grid
    }
}

impl GameObject for Grid {
    fn update(&mut self, d: &mut RaylibDrawHandle) {
        let ctx = get_read_context!();
        self.data[self.cursor_y][self.cursor_x].selected = false;
        if d.is_key_pressed(KEY_LEFT) {
            if self.cursor_x > 0 {
                self.cursor_x -= 1;
            }
        } else if d.is_key_pressed(KEY_RIGHT) {
            if self.cursor_x < ctx.cfg.grid.num_cols {
                self.cursor_x += 1;
            }
        }
        self.data[self.cursor_y][self.cursor_x].selected = true;
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        let ctx = get_read_context!();
        d.draw_rectangle(
            self.rect.x as i32,
            self.rect.y as i32,
            self.rect.width as i32,
            self.rect.height as i32,
            &ctx.cfg.grid.color_area
        );
        for row in 0..ctx.cfg.grid.num_rows as usize {
            for col in 0..ctx.cfg.grid.num_cols as usize {
                self.data[row][col].draw(d);
            }
        }
    }
}
