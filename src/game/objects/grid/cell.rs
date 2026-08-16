use raylib::drawing::RaylibDraw;
use raylib::prelude::RaylibDrawHandle;
use crate::game::GameObject;
use crate::get_read_context;

#[derive(Clone, Debug)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub ch: char,
    pub selected: bool,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            ch: ' ',
            selected: false,
        }
    }
    
    pub fn set_coords(&mut self, row: usize, col: usize) {
        let ctx = get_read_context!();
        let grid = &ctx.cfg.grid;
        self.x = grid.box_gap + col * (grid.box_size + grid.box_gap);
        self.y = grid.box_gap + row * (grid.box_size + grid.box_gap);
    }
}

impl GameObject for Cell {
    fn update(&mut self, d: &mut RaylibDrawHandle) {}
    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        let ctx = get_read_context!();
        let cell_color = if self.selected {
            &ctx.cfg.grid.color_border
        } else {
            &ctx.cfg.grid.color_emptybox
        };
        d.draw_rectangle(
            self.x as i32,
            self.y as i32,
            ctx.cfg.grid.box_size as i32,
            ctx.cfg.grid.box_size as i32,
            cell_color,
        );
    }
}
