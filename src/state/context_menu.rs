use super::*;
use nalgebra as na;

pub struct ContextMenu {
    is_top: bool,
}

impl ContextMenu {
    pub fn new<'c>(world: &'c mut World, x: i32, y: i32) -> ContextMenu {
        ContextMenu { is_top: false }
    }
}

impl State for ContextMenu {
    fn start(&mut self, _ctx: &mut Context, _world: &mut World) -> GameResult {
        self.is_top = true;
        Ok(())
    }

    fn stop(&mut self, _ctx: &mut Context, _world: &mut World) -> GameResult {
        self.is_top = false;
        Ok(())
    }

    fn pause(&mut self, _ctx: &mut Context, _world: &mut World) -> GameResult {
        self.is_top = false;
        Ok(())
    }

    fn resume(&mut self, _ctx: &mut Context, _world: &mut World) -> GameResult {
        self.is_top = true;
        Ok(())
    }

    fn input(
        &mut self,
        _ctx: &mut Context,
        _world: &mut World,
        _command: Command,
        _extra: InputExtra,
    ) -> GameResult<Transition> {
        Ok(Transition::Pop)
    }

    fn draw(&mut self, _ctx: &mut Context, _world: &mut World, _assets: &Assets) -> GameResult {
        Ok(())
    }

    fn draw_underlying(&self) -> bool {
        true
    }
}

impl Display for ContextMenu {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ContextMenu")
    }
}
