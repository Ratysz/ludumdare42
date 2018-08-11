use super::*;

pub struct PopulateGrid;

impl<'a> System<'a> for PopulateGrid {
    type SystemData = (
        Entities<'a>,
        Write<'a, Grid>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Tile>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let (w, h, d) = data.1.dimensions();
        for x in 0..w {
            for y in 0..h {
                for z in 0..d {
                    create_tile(&mut data, x, y, z, Tile::Terrain);
                }
            }
        }
    }
}

fn create_tile<'a>(
    data: &mut <PopulateGrid as System>::SystemData,
    x: usize,
    y: usize,
    z: usize,
    tile: Tile,
) {
    let (entities, grid, positions, tiles) = data;
    let entity = entities.create();
    positions.insert(entity, grid.new_position(entity, x, y, z));
    tiles.insert(entity, tile);
}
