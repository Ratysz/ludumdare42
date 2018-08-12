use super::*;
use nalgebra as na;
use specs::world::Index;

pub struct ContextMenu {
    is_top: bool,
    target_id: Index,
    target_tile: Tile,
    target_pos: Position,
}

impl ContextMenu {
    pub fn new<'c>(ctx: &Context, world: &'c mut World, x: i32, y: i32) -> Option<ContextMenu> {
        let grid = world.read_resource::<Grid>();
        let entities = world.entities();
        let positions = world.read_storage::<Position>();
        let tiles = world.read_storage::<Tile>();
        for (entity, pos, tile) in (&*entities, &positions, &tiles).join() {
            if grid.is_top_tile(pos) && tile::hit_test(ctx, tile::map_pos_to_screen(pos)) {
                debug!("Target: {:?} at {:?} ({:?})", tile, pos, entity);
                return Some(ContextMenu {
                    is_top: false,
                    target_id: entity.id(),
                    target_tile: *tile,
                    target_pos: *pos,
                });
            }
        }
        None
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
