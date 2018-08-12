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

    fn run(&mut self, (entities, mut grid, mut positions, mut tiles): Self::SystemData) {
        let (w, h, d) = grid.dimensions();
        let noise = Perlin::new().set_seed(rand::random());
        let mut map = HashMap::new();
        for x in 0..w {
            for y in 0..h {
                let bound = (y as f64
                    + 0.5 * d as f64
                        * noise
                            .get([(1.0 + x as f64 / w as f64), (1.0 + y as f64 / h as f64)])
                            .abs())
                    .max(1.0)
                    .min(d as f64 - 1.0);
                for z in 0..(bound as usize) {
                    let entity = entities.create();
                    positions
                        .insert(entity, grid.new_position(entity, x, y, z))
                        .unwrap();
                    tiles.insert(entity, Tile::Terrain).unwrap();
                    map.insert((x, y, z), Tile::Terrain);
                }
            }
        }
        for x in 0..w {
            for y in 0..h {
                for z in 0..4 {
                    if !map.contains_key(&(x, y, z)) {
                        let entity = entities.create();
                        positions
                            .insert(entity, grid.new_position(entity, x, y, z))
                            .unwrap();
                        tiles.insert(entity, Tile::Water).unwrap();
                        map.insert((x, y, z), Tile::Water);
                    }
                }
            }
        }
        for x in 0..w {
            for y in 0..h {
                for z in 4..d {
                    if !map.contains_key(&(x, y, z)) {
                        if map.get(&(x, y, z - 1)) == Some(&Tile::Terrain) {
                            if noise.get([
                                (1.0 + 0.5 * x as f64 / w as f64),
                                (1.0 + 2.0 * y as f64 / h as f64),
                            ]) > 0.0
                            {
                                let entity = entities.create();
                                positions
                                    .insert(entity, grid.new_position(entity, x, y, z))
                                    .unwrap();
                                tiles.insert(entity, Tile::Trees).unwrap();
                                map.insert((x, y, z), Tile::Trees);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}
