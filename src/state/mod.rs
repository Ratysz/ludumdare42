use ggez::{
    graphics::{self, Align, Color, DrawParam, Scale, Text, TextFragment},
    Context, GameResult,
};
use specs::prelude::*;
use std::fmt::{Display, Formatter, Result};

use assets::{random_color, Assets, DrawableHandle, SoundHandle};
use ecs::*;
use gui;
use input::{Command, InputExtra};
use time::{AllThingsDoer, Time};

mod context_menu;
mod game;
mod game_over;
mod main_menu;

pub use self::context_menu::ContextMenu;
pub use self::game::Game;
pub use self::game_over::GameOver;
pub use self::main_menu::MainMenu;

pub enum Transition {
    None,
    Push(Box<State>),
    Pop,
    PopAll,
    Replace(Box<State>),
}

pub trait State: Display {
    fn start(
        &mut self,
        _ctx: &mut Context,
        _assets: &mut Assets,
        _world: &mut World,
    ) -> GameResult {
        Ok(())
    }

    fn stop(&mut self, _ctx: &mut Context, _assets: &mut Assets, _world: &mut World) -> GameResult {
        Ok(())
    }

    fn pause(
        &mut self,
        _ctx: &mut Context,
        _assets: &mut Assets,
        _world: &mut World,
    ) -> GameResult {
        Ok(())
    }

    fn resume(
        &mut self,
        _ctx: &mut Context,
        _assets: &mut Assets,
        _world: &mut World,
    ) -> GameResult {
        Ok(())
    }

    fn input(
        &mut self,
        _ctx: &mut Context,
        _assets: &mut Assets,
        _world: &mut World,
        _command: Command,
        _extra: InputExtra,
    ) -> GameResult<Transition> {
        Ok(Transition::None)
    }

    fn update(
        &mut self,
        _ctx: &mut Context,
        _assets: &mut Assets,
        _world: &mut World,
    ) -> GameResult<Transition> {
        Ok(Transition::None)
    }

    fn update_underlying(&self) -> bool {
        false
    }

    fn draw(&mut self, _ctx: &mut Context, _assets: &mut Assets, _world: &mut World) -> GameResult {
        Ok(())
    }

    fn draw_underlying(&self) -> bool {
        false
    }
}
