mod config;
mod objects;

use config::Config;
use objects::{Grid, StatusLine, Title};
use raylib::prelude::*;

#[derive(PartialEq)]
enum GameState {
    Starting,
//    Running,
    Finished,
}

pub struct GameContext<'a> {
    cfg: &'a Config,
    state: GameState,
}

trait GameObject {
    fn update(&mut self, d: &mut RaylibDrawHandle);
    fn draw(&mut self, d: &mut RaylibDrawHandle);
}

pub fn run() {
    // Initialize raylib and window
    let cfg = Config::new();
    let (mut rl, thread) = raylib::init()
        .size(cfg.window.width, cfg.window.height)
        .title(&cfg.window.title)
        .log_level(TraceLogLevel::LOG_ERROR)
        .vsync()
        .build();

    rl.set_target_fps(60);

    let context = GameContext {
        cfg: &cfg,
        state: GameState::Starting,
    };

    // Register all game objects that will be updated and drawn
    let mut objects: Vec<Box<dyn GameObject>> = Vec::new();
    objects.push(Box::new(Title::new(&context)));
    objects.push(Box::new(Grid::new(&context)));
    objects.push(Box::new(StatusLine::new(&context)));

    // Game loop
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::from(&cfg.window.area_color));

        // Update all game objects
        for obj in &mut objects {
            obj.update(&mut d);
        }

        if context.state == GameState::Finished {
            break;
        }

        // Draw all game objects
        for obj in &mut objects {
            obj.draw(&mut d);
        }
    }
}
