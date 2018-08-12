use super::*;
use nalgebra as na;
use specs::world::Index;
use std::f32::INFINITY;

pub struct ContextMenu {
    is_top: bool,
    text: Text,
    target_id: Index,
    target_tile: Tile,
    target_pos: Position,
}

impl ContextMenu {
    pub fn new<'c>(ctx: &Context, world: &'c mut World) -> Option<ContextMenu> {
        let grid = world.read_resource::<Grid>();
        let entities = world.entities();
        let positions = world.read_storage::<Position>();
        let tiles = world.read_storage::<Tile>();
        for (entity, pos, tile) in (&*entities, &positions, &tiles).join() {
            if grid.is_top_tile(pos) && tile::hit_test(ctx, tile::map_pos_to_screen(pos)) {
                debug!("Target: {:?} at {:?} ({:?})", tile, pos, entity);
                let mut text = Text::new(
                    TextFragment::new("I'm a context menu!\n").scale(Scale::uniform(20.0)),
                );
                text.add(format!("This is {:?}\n", tile))
                    .add(format!("Height is {}\n", pos.z()))
                    .add(format!("X {} Y {}\n", pos.x(), pos.y()));
                return Some(ContextMenu {
                    is_top: false,
                    text,
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
        let pos = tile::map_pos_to_screen(&self.target_pos);
        graphics::draw(
            _ctx,
            _assets.fetch_drawable(DrawableHandle::TileSelector),
            DrawParam::new().dest(pos).color(random_color()),
        )?;
        //let pos = pos - na::Vector2::new(0.0, TILE_SIZE.1);
        graphics::draw(_ctx, &self.text, (pos, graphics::WHITE));
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
