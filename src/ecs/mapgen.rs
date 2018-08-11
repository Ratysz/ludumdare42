use super::*;
use noise::{NoiseFn, Perlin, Seedable};
use rand;
use std::collections::HashMap;

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
        let noise = Perlin::new().set_seed(rand::random());
        let mut map = HashMap::new();
        for x in 0..w {
            for y in 0..h {
                let bound = (1.0 + y as f64
                    + d as f64
                        * noise
                            .get([(1.0 + x as f64 / w as f64), (1.0 + y as f64 / h as f64)])
                            .abs())
                    .max(1.0);
                for z in 0..(bound as usize) {
                    create_tile(&mut data, x, y, z, Tile::Terrain);
                    map.insert((x, y, z), Tile::Terrain);
                }
            }
        }
        for x in 0..w {
            for y in 0..h {
                for z in 0..4 {
                    if !map.contains_key(&(x, y, z)) {
                        create_tile(&mut data, x, y, z, Tile::Water);
                        map.insert((x, y, z), Tile::Water);
                    }
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
    positions
        .insert(entity, grid.new_position(entity, x, y, z))
        .unwrap();
    tiles.insert(entity, tile).unwrap();
}
