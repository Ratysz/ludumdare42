use ggez::{Context, GameResult};
use rand;
use specs::prelude::*;
use std::fmt::{self, Display, Formatter};

use super::{State, Transition};
use assets::{Assets, SoundHandle};
use ecs::{Grid, Time, TILE_SIZE};
use input::{Command, InputExtra};

const MULTIPLIER: f32 = 32.0 / TILE_SIZE.0 as f32;

pub struct MainMenu;

impl State for MainMenu {
    fn start(
        &mut self,
        _ctx: &mut Context,
        _assets: &mut Assets,
        _world: &mut World,
    ) -> GameResult {
        _assets.sound(SoundHandle::Waves).set_repeat(true);
        _assets.sound(SoundHandle::Waves).play()
    }

    fn update(
        &mut self,
        _ctx: &mut Context,
        _assets: &mut Assets,
        _world: &mut World,
    ) -> GameResult<Transition> {
        _world.delete_all();
        _world.maintain();
        *_world.res.entry::<Grid>().or_insert_with(|| {
            Grid::new(
                (8.0 * MULTIPLIER).floor() as usize,
                (8.0 * MULTIPLIER).floor() as usize,
                (16.0 * MULTIPLIER).floor() as usize,
                rand::random(),
            )
        }) = Grid::new(
            (8.0 * MULTIPLIER).floor() as usize,
            (8.0 * MULTIPLIER).floor() as usize,
            (16.0 * MULTIPLIER).floor() as usize,
            rand::random(),
        );
        *_world.res.entry::<Time>().or_insert_with(Time::new) = Time::new();
        Ok(Transition::Push(Box::new(super::Game::new(_world))))
    }
}

impl Display for MainMenu {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Main Menu")
    }
}
