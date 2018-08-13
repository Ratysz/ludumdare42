#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate chrono;
extern crate fern;
extern crate ggez;
#[macro_use]
extern crate log;
extern crate nalgebra;
extern crate noise;
extern crate rand;
extern crate ron;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate specs;
#[macro_use]
extern crate specs_derive;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::{ContextBuilder, GameResult};

mod assets;
mod ecs;
mod game;
mod input;
mod saveload;
mod state;
mod time;
mod tooltip;

fn wrapped() -> GameResult {
    let w_dim = nalgebra::Vector2::new(640.0, 380.0);

    let (ctx, events_loop) = &mut ContextBuilder::new("LD42", "Ratys")
        .window_setup(WindowSetup::default().title("LD42"))
        .window_mode(
            WindowMode::default()
                .dimensions(w_dim.x, w_dim.y)
                .max_dimensions(w_dim.x, w_dim.y)
                .min_dimensions(w_dim.x, w_dim.y),
        )
        .build()?;

    let state = &mut game::Game::new(ctx)?;
    event::run(ctx, events_loop, state)
}

fn main() {
    #[cfg(debug_assertions)]
    {
        use fern::colors::{Color, ColoredLevelConfig};
        let colors = ColoredLevelConfig::default()
            .trace(Color::BrightBlue)
            .debug(Color::Cyan)
            .info(Color::Green)
            .warn(Color::Yellow)
            .error(Color::Red);
        fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "[{}][{:<14}][{}] {}",
                    chrono::Local::now().format("%H:%M:%S"),
                    colors.color(record.level()).to_string(),
                    record.target(),
                    message
                ))
            })
            .level_for("gfx_device_gl", log::LevelFilter::Warn)
            .level_for("ggez", log::LevelFilter::Debug)
            .level(log::LevelFilter::Trace)
            .chain(std::io::stdout())
            .apply()
            .unwrap();
    }

    if let Err(e) = wrapped() {
        error!("{}", e);
    }
}
