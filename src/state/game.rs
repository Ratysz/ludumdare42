use super::*;
use nalgebra as na;

pub struct Game<'a, 'b> {
    logic: Dispatcher<'a, 'b>,
    animation: Dispatcher<'a, 'b>,
    is_top: bool,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new<'c>(world: &'c mut World) -> Game<'a, 'b> {
        world.res.entry::<Grid>().or_insert_with(|| Grid::default());

        let mut logic = DispatcherBuilder::new()
            .with(grid::GridGravity, "grid_gravity", &[])
            .build();
        logic.setup(&mut world.res);

        let mut animation = DispatcherBuilder::new().build();
        animation.setup(&mut world.res);

        let mut grid_populator = mapgen::PopulateGrid;
        <mapgen::PopulateGrid as System>::setup(&mut grid_populator, &mut world.res);
        grid_populator.run_now(&mut world.res);
        world.maintain();

        Game {
            logic,
            animation,
            is_top: false,
        }
    }
}

impl<'a, 'b> State for Game<'a, 'b> {
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
        match _command {
            Command::ContextMenu => if let InputExtra::XY(x, y) = _extra {
                return Ok(Transition::Push(Box::new(ContextMenu::new(_world, x, y))));
            },
            _ => self.logic.dispatch(&mut _world.res),
        }
        Ok(Transition::None)
    }

    fn draw(&mut self, _ctx: &mut Context, _world: &mut World, _assets: &Assets) -> GameResult {
        self.animation.dispatch(&mut _world.res);
        let positions = _world.read_storage::<Position>();
        let tiles = _world.read_storage::<Tile>();
        let mut sorted = (&positions, &tiles).join().collect::<Vec<_>>();
        sorted.sort_by_key(|(&pos, _)| pos);
        for (pos, tile) in sorted.iter() {
            tile.draw(_ctx, _assets, pos)?;
        }
        if self.is_top {
            let grid = _world.read_resource::<Grid>();
            for (pos, tile) in (&positions, &tiles).join() {
                if grid.is_top_tile(pos) && tile.draw_tooltip(_ctx, _assets, pos)? {
                    break;
                }
            }
        }
        Ok(())
    }

    fn draw_underlying(&self) -> bool {
        false
    }
}

impl<'a, 'b> Display for Game<'a, 'b> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Game")
    }
}
