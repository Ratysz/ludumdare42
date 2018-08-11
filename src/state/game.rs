use super::*;
use nalgebra as na;

pub const TILE_SIZE_PX: (f32, f32) = (30.0, 30.0);

pub struct Game<'a, 'b> {
    logic: Dispatcher<'a, 'b>,
    animation: Dispatcher<'a, 'b>,
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

        Game { logic, animation }
    }
}

impl<'a, 'b> State for Game<'a, 'b> {
    fn start(&mut self, _ctx: &mut Context, _world: &mut World) -> GameResult {
        Ok(())
    }

    fn stop(&mut self, _ctx: &mut Context, _world: &mut World) -> GameResult {
        Ok(())
    }

    fn update(&mut self, _ctx: &mut Context, _world: &mut World) -> GameResult<Transition> {
        self.logic.dispatch(&mut _world.res);
        Ok(Transition::None)
    }

    fn draw(
        &mut self,
        _ctx: &mut Context,
        _world: &mut World,
        _assets: &Assets,
    ) -> GameResult<bool> {
        self.animation.dispatch(&mut _world.res);
        let positions = _world.read_storage::<Position>();
        let tiles = _world.read_storage::<Tile>();
        let mut sorted = (&positions, &tiles).join().collect::<Vec<_>>();
        sorted.sort_by_key(|(&pos, _)| pos);
        for (pos, tile) in sorted.iter() {
            match tile {
                Tile::Water => {
                    graphics::draw(
                        _ctx,
                        _assets.fetch_drawable(DrawableHandle::FullTile),
                        DrawParam::new()
                            .dest(map_pos_to_screen(pos))
                            .color(map_pos_to_water_color(pos))
                            .scale(na::Vector2::new(TILE_SIZE_PX.0, TILE_SIZE_PX.1)),
                    )?;
                }
                Tile::Terrain => {
                    graphics::draw(
                        _ctx,
                        _assets.fetch_drawable(DrawableHandle::FullTile),
                        DrawParam::new()
                            .dest(map_pos_to_screen(pos))
                            .color(map_pos_to_terrain_color(pos))
                            .scale(na::Vector2::new(TILE_SIZE_PX.0, TILE_SIZE_PX.1)),
                    )?;
                }
                _ => (),
            }
        }
        Ok(false)
    }
}

impl<'a, 'b> Display for Game<'a, 'b> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Game")
    }
}

fn map_pos_to_screen(pos: &Position) -> na::Point2<f32> {
    na::Point2::new(
        110.0 + (pos.x() as f32 * TILE_SIZE_PX.0) + (pos.y() as f32 * TILE_SIZE_PX.1),
        240.0 + (pos.x() as f32 * TILE_SIZE_PX.0 * 0.5)
            - (pos.y() as f32 * TILE_SIZE_PX.1 * 0.5)
            - (pos.z() as f32 * TILE_SIZE_PX.0 * 0.25),
    )
}

fn map_pos_to_water_color(pos: &Position) -> Color {
    Color::new(
        0.0,
        0.2 * ((pos.z() as f32 + 1.0) / 4.0).min(1.0),
        1.0 * ((pos.z() as f32 + 1.0) / 4.0).min(1.0),
        0.5,
    )
}

fn map_pos_to_terrain_color(pos: &Position) -> Color {
    Color::new(
        0.4 * ((pos.z() as f32 + 1.0) / 12.0).min(1.0),
        0.8 * ((pos.z() as f32 + 1.0) / 12.0).min(1.0),
        0.0,
        1.0,
    )
}
