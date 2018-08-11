use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color};
use ggez::timer;
use ggez::{Context, GameResult};
use specs::prelude::*;

use assets::Assets;
use input::{Command, InputExtra, InputHandler};
use state::{MainMenu, State, Transition};
use time::Time;

pub struct Game {
    assets: Assets,
    input: InputHandler,
    state_stack: Vec<Box<State>>,
    world: World,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let mut world = World::new();
        world.res.entry::<Time>().or_insert_with(Time::new);
        let mut state_stack = Vec::<Box<State>>::new();
        state_stack.push(Box::new(MainMenu));
        if let Some(current_state) = state_stack.last_mut() {
            trace!("Starting state {}", &current_state);
            if let Err(e) = current_state.start(ctx, &mut world) {
                error!("Error starting state {}: {:?}", &current_state, e);
            }
        }
        Ok(Game {
            assets: Assets::new(ctx)?,
            input: InputHandler::default(),
            state_stack,
            world,
        })
    }

    fn handle_transition(&mut self, ctx: &mut Context, transition: Transition) {
        match transition {
            Transition::None => (),
            Transition::Push(new_state) => {
                trace!("Pushing state {}", &new_state);
                if let Some(old_state) = self.state_stack.last_mut() {
                    trace!("Pausing state {}", &old_state);
                    if let Err(e) = old_state.pause(ctx, &mut self.world) {
                        error!("Error pausing state {}: {:?}", &old_state, e);
                    }
                }
                self.state_stack.push(new_state);
                if let Some(current_state) = self.state_stack.last_mut() {
                    trace!("Starting state {}", &current_state);
                    if let Err(e) = current_state.start(ctx, &mut self.world) {
                        error!("Error starting state {}: {:?}", &current_state, e);
                    }
                }
            }
            Transition::Pop => {
                trace!("Popping state");
                if let Some(mut old_state) = self.state_stack.pop() {
                    trace!("Stopping state {}", &old_state);
                    if let Err(e) = old_state.stop(ctx, &mut self.world) {
                        error!("Error stopping state {}: {:?}", &old_state, e);
                    }
                }
                if let Some(current_state) = self.state_stack.last_mut() {
                    trace!("Resuming state {}", &current_state);
                    if let Err(e) = current_state.resume(ctx, &mut self.world) {
                        error!("Error resuming state {}: {:?}", &current_state, e);
                    }
                }
            }
            Transition::PopAll => {
                trace!("Popping all states");
                while let Some(mut state) = self.state_stack.pop() {
                    trace!(" stopping {}", &state);
                    if let Err(e) = state.stop(ctx, &mut self.world) {
                        error!("Error stopping state {}: {:?}", &state, e);
                    }
                }
            }
            Transition::Replace(new_state) => {
                if let Some(mut old_state) = self.state_stack.pop() {
                    trace!("Replacing state {} with {}", &old_state, &new_state);
                    if let Err(e) = old_state.stop(ctx, &mut self.world) {
                        error!("Error stopping state {}: {:?}", &old_state, e);
                    }
                } else {
                    warn!("Tried to replace non-existing state with {}", &new_state);
                }
                self.state_stack.push(new_state);
                if let Some(current_state) = self.state_stack.last_mut() {
                    trace!("Starting state {}", &current_state);
                    if let Err(e) = current_state.start(ctx, &mut self.world) {
                        error!("Error starting state {}: {:?}", &current_state, e);
                    }
                }
            }
        }
    }

    fn propagate_input(&mut self, ctx: &mut Context, resolved: Option<(Command, InputExtra)>) {
        if let Some((command, extra)) = resolved {
            let transition = match self.state_stack.last_mut() {
                Some(state) => match state.input(ctx, &mut self.world, command, extra) {
                    Ok(transition) => transition,
                    Err(e) => {
                        error!("State {} input error: {:?}", &state, e);
                        Transition::None
                    }
                },
                None => {
                    trace!("State stack empty, quitting.");
                    ctx.quit();
                    return;
                }
            };
            self.handle_transition(ctx, transition);
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let transition = match self.state_stack.last_mut() {
            Some(state) => match state.update(ctx, &mut self.world) {
                Ok(transition) => transition,
                Err(e) => {
                    error!("State {} update error: {:?}", state, e);
                    Transition::None
                }
            },
            None => {
                trace!("State stack empty, quitting.");
                ctx.quit();
                return Ok(());
            }
        };
        self.handle_transition(ctx, transition);
        self.world.maintain();
        while timer::check_update_time(ctx, 60) {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from([0.0, 0.0, 0.0, 1.0]));
        self.world
            .write_resource::<Time>()
            .update_delta(timer::get_delta(ctx));
        let mut draw_depth = 0;
        while let Some(state) = self.state_stack.iter().next_back() {
            draw_depth += 1;
            if !state.draw_underlying() {
                break;
            }
        }
        while let Some(state) = self.state_stack.iter_mut().next_back() {
            if let Err(e) = state.draw(ctx, &mut self.world, &self.assets) {
                error!("State {} drawing error: {:?}", state, e)
            }
            draw_depth -= 1;
            if draw_depth == 0 {
                break;
            }
        }
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, repeat: bool) {
        let resolved = self.input.key_down_event(ctx, key, mods.into(), repeat);
        self.propagate_input(ctx, resolved);
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        trace!("quit_event() callback called, quitting...");
        self.handle_transition(_ctx, Transition::PopAll);
        false
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, width, height))
            .unwrap();
    }
}
