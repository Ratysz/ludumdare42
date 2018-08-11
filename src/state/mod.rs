use ggez::{
    graphics::{self, Color, DrawParam},
    Context, GameResult,
};
use specs::prelude::*;
use std::fmt::{Display, Formatter, Result};

use assets::{Assets, DrawableHandle};
use ecs::*;
use input::{Command, InputExtra};

mod game;
mod main_menu;

pub use self::game::Game;
pub use self::main_menu::MainMenu;

pub enum Transition {
    None,
    Push(Box<State>),
    Pop,
    PopAll,
    Replace(Box<State>),
}

pub trait State: Display {
    fn start(&mut self, _ctx: &mut Context, _world: &mut World) -> GameResult {
        Ok(())
    }

    fn stop(&mut self, _ctx: &mut Context, _world: &mut World) -> GameResult {
        Ok(())
    }

    fn pause(&mut self, _ctx: &mut Context, _world: &mut World) -> GameResult {
        Ok(())
    }

    fn resume(&mut self, _ctx: &mut Context, _world: &mut World) -> GameResult {
        Ok(())
    }

    fn input(
        &mut self,
        _ctx: &mut Context,
        _world: &mut World,
        _command: Command,
        _extra: InputExtra,
    ) -> GameResult<Transition> {
        Ok(Transition::None)
    }

    fn update(&mut self, _ctx: &mut Context, _world: &mut World) -> GameResult<Transition> {
        Ok(Transition::None)
    }

    fn draw(
        &mut self,
        _ctx: &mut Context,
        _world: &mut World,
        _assets: &Assets,
    ) -> GameResult<bool> {
        Ok(false)
    }
}
