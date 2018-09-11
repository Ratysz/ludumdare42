use ggez::graphics::{Align, Color, Scale, Text, TextFragment};
use ggez::input::mouse;
use ggez::{Context, GameResult};
use nalgebra as na;
use specs::prelude::*;
use std::f32::INFINITY;
use std::fmt::{self, Display, Formatter};

use super::{State, Transition};
use assets::{Assets, SoundHandle};
use ecs::{drawing, mapgen, tile_generator::TileCreationSystemData, Time};
use input::{Command, InputExtra};

pub struct Game<'a, 'b> {
    dispatcher_reset: Dispatcher<'a, 'b>,
    dispatcher_turn: Dispatcher<'a, 'b>,
    dispatcher_animation: Dispatcher<'a, 'b>,
    is_top: bool,
    skip_text: Text,
    rules_text: Text,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new<'c>(world: &'c mut World) -> Game<'a, 'b> {
        let mut dispatcher_reset = DispatcherBuilder::new()
            .with(mapgen::GenerateMap, "generate_map", &[])
            .build();
        dispatcher_reset.setup(&mut world.res);
        world.setup::<TileCreationSystemData>();

        let mut dispatcher_turn = DispatcherBuilder::new().build();
        dispatcher_turn.setup(&mut world.res);

        let mut dispatcher_animation = DispatcherBuilder::new().build();
        dispatcher_animation.setup(&mut world.res);
        world.setup::<drawing::DrawTilesSystemData>();

        dispatcher_reset.dispatch(&mut world.res);
        world.maintain();

        let mut skip_text = Text::new("[skip turn]");
        let mut rules_text = Text::new(
            TextFragment::new(
                "Population needs either food or housing to survive.\n\
                 Population requires food to grow.\n\
                 Sanctuaries require power to work.\n\
                 Negative nature makes sea rise faster!",
            ).scale(Scale::uniform(15.0)),
        );
        rules_text.set_bounds(na::Point2::new(640.0, INFINITY), Align::Right);

        Game {
            dispatcher_reset,
            dispatcher_turn,
            dispatcher_animation,
            is_top: false,
            skip_text,
            rules_text,
        }
    }
}

impl<'a, 'b> State for Game<'a, 'b> {
    fn start(
        &mut self,
        _ctx: &mut Context,
        _assets: &mut Assets,
        _world: &mut World,
    ) -> GameResult {
        self.is_top = true;
        Ok(())
    }

    fn stop(&mut self, _ctx: &mut Context, _assets: &mut Assets, _world: &mut World) -> GameResult {
        self.is_top = false;
        Ok(())
    }

    fn pause(
        &mut self,
        _ctx: &mut Context,
        _assets: &mut Assets,
        _world: &mut World,
    ) -> GameResult {
        self.is_top = false;
        Ok(())
    }

    fn resume(
        &mut self,
        _ctx: &mut Context,
        _assets: &mut Assets,
        _world: &mut World,
    ) -> GameResult {
        self.is_top = true;
        /*let passed = _world.read_resource::<Time>().turn_passed;
        if passed {
            let flood = _world.read_resource::<Time>().flood_timer;
            if flood < 1 {
                _assets.sound(SoundHandle::WaveCrash).play();
                mapgen::Flood.run_now(&mut _world.res);
                _world.write_resource::<Time>().flood_timer = 8;
            }
            AllThingsDoer.run_now(&mut _world.res);
        }
        _world.write_resource::<Time>().turn_passed = false;*/
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
        match _command {
            Command::ContextMenu => if let InputExtra::XY(x, y) = _extra {
                /*if let Some(menu) = ContextMenu::new(_ctx, _world, _assets) {
                    return Ok(Transition::Push(Box::new(menu)));
                }*/
            },
            Command::Click => if let InputExtra::XY(x, y) = _extra {
                /*if let Some(menu) = ContextMenu::new(_ctx, _world, _assets) {
                    return Ok(Transition::Push(Box::new(menu)));
                } else {
                    if ((260.0 - x as f32).abs() as u32) < self.skip_text.width(_ctx)
                        && ((5.0 - y as f32).abs() as u32) < self.skip_text.height(_ctx)
                    {
                        _world.write_resource::<Time>().turn_passed = true;
                        self.resume(_ctx, _assets, _world);
                    }
                }*/
            },
            _ => (),
        }
        Ok(Transition::None)
    }

    fn update(
        &mut self,
        _ctx: &mut Context,
        _assets: &mut Assets,
        _world: &mut World,
    ) -> GameResult<Transition> {
        let over = _world.read_resource::<Time>().game_over;
        if over {
            Ok(Transition::Push(Box::new(super::GameOver(
                _world.read_resource::<Time>().score,
            ))))
        } else if _world.read_resource::<Time>().game_over_transition_done {
            Ok(Transition::Pop)
        } else {
            Ok(Transition::None)
        }
    }

    fn draw(&mut self, _ctx: &mut Context, _assets: &mut Assets, _world: &mut World) -> GameResult {
        self.dispatcher_animation.dispatch(&mut _world.res);
        drawing::draw_tiles(_ctx, _assets, _world)?;
        /*let time = _world.read_resource::<Time>();
        let grid = _world.read_resource::<Grid>();
        let positions = _world.read_storage::<Position>();
        let tiles = _world.read_storage::<Tile>();
        let sealevel = grid.current_sealevel;
        let depth = grid.dimensions().2;
        let mut sorted = (&positions, &tiles).join().collect::<Vec<_>>();
        sorted.sort_by_key(|(&pos, _)| pos);
        for (pos, tile) in sorted.iter() {
            tile.draw(_ctx, _assets, pos, sealevel, depth, grid.is_top_tile(pos))?;
        }
        if self.is_top {
            for (pos, tile) in (&positions, &tiles).join() {
                if grid.is_top_tile(pos) && tile.draw_tooltip(_ctx, _assets, pos)? {
                    break;
                }
            }
        }
        //gui::draw_score(_ctx, &time)?;
        let offset = self.skip_text.width(_ctx) as f32;
        let mpos = mouse::get_position(_ctx);
        let color = if ((260.0 - mpos.x as f32).abs() as u32) < self.skip_text.width(_ctx)
            && ((5.0 - mpos.y as f32).abs() as u32) < self.skip_text.height(_ctx)
        {
            Color::new(0.5, 1.0, 0.5, 1.0)
        } else {
            Color::new(0.0, 0.8, 0.8, 1.0)
        };
        graphics::draw(
            _ctx,
            &self.skip_text,
            DrawParam::new()
                .dest(na::Point2::new(260.0 - offset, 5.0))
                .color(color),
        )?;
        graphics::draw(_ctx, &self.rules_text, DrawParam::new())?;*/
        Ok(())
    }

    fn draw_underlying(&self) -> bool {
        false
    }
}

impl<'a, 'b> Display for Game<'a, 'b> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Game")
    }
}
