mod config;
mod objects;

use config::Config;
use objects::{Grid, StatusLine, Title};
use raylib::{ffi::KeyboardKey::KEY_ENTER, prelude::*};
use std::sync::{OnceLock, RwLock};

static CONTEXT: OnceLock<RwLock<GameContext>> = OnceLock::new();

#[derive(Clone, Copy, Debug, PartialEq)]
enum GameState {
    Starting,
    Playing,
    Finished,
}

#[derive(Clone, Debug)]
pub struct GameContext {
    cfg: Config,
    state: GameState,
}

trait GameObject {
    fn update(&mut self, d: &mut RaylibDrawHandle);
    fn draw(&mut self, d: &mut RaylibDrawHandle);
}

pub fn setup() {
    let _ = CONTEXT.set(RwLock::new(GameContext {
            cfg: Config::new(),
            state: GameState::Starting,
        }));
}

pub fn run() {
    let ctx = CONTEXT.get().expect("Uninitialized GameContext");
    let window = {
        let guard = ctx.read().unwrap();
        guard.cfg.window.clone()
    };

    let (mut rl, thread) = raylib::init()
        .size(window.width, window.height)
        .title(&window.title)
        .log_level(TraceLogLevel::LOG_ERROR)
        .vsync()
        .build();

    rl.set_target_fps(60);

    // Game loop
    while !rl.window_should_close() {

        let current_state = {
            let guard = ctx.read().unwrap();
            guard.state
        };

        match current_state {
            GameState::Finished => break,
            GameState::Starting => {
                if rl.is_key_pressed(KEY_ENTER) {
                    let mut guard = ctx.write().unwrap();
                    guard.state = GameState::Playing;
                }
            },
            GameState::Playing => {},
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(&window.area_color);

        let mut objects = Vec::<Box<dyn GameObject>>::new();
        objects.push(Box::new(Title::new("Wordle")));
        objects.push(Box::new(Grid::new()));
        objects.push(Box::new(StatusLine::new()));

        // Update all game objects
        for obj in &mut objects {
            obj.update(&mut d);
        }

        // Draw all game objects
        for obj in &mut objects {
            obj.draw(&mut d);
        }
    }
}
