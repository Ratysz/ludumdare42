use ggez::graphics::{DrawMode, Mesh, Rect};
use ggez::input::mouse;
use ggez::{Context, GameResult};
use nalgebra as na;
use specs::prelude::*;
use std::f32::INFINITY;
use std::fmt::{self, Display, Formatter};

use super::{State, Transition};
use assets::Assets;
use input::{Command, InputExtra};

pub struct GameOver(pub i32);

impl State for GameOver {
    fn draw(&mut self, _ctx: &mut Context, _assets: &mut Assets, _world: &mut World) -> GameResult {
        /*let rect = Mesh::new_rectangle(_ctx, DrawMode::Fill, Rect::new(0.0, 0.0, 640.0, 380.0))?;
        graphics::draw(
            _ctx,
            &rect,
            DrawParam::new().color(Color::new(0.0, 0.0, 0.0, 0.3)),
        )?;
        let mut text = Text::new(TextFragment::new("GAME OVER\n\r").scale(Scale::uniform(30.0)));
        text.add(TextFragment::new("CLICK TO RESTART"));
        text.set_bounds(na::Point2::new(640.0, INFINITY), Align::Center);
        graphics::draw(
            _ctx,
            &text,
            DrawParam::new().dest(na::Point2::new(0.0, 100.0)),
        )*/
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
        /*_world.write_resource::<Time>().game_over = false;
        _world.write_resource::<Time>().game_over_transition_done = true;*/
        Ok(Transition::Pop)
    }

    fn draw_underlying(&self) -> bool {
        true
    }
}

impl Display for GameOver {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Game Over")
    }
}
