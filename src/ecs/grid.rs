use super::*;
use std::cmp::{Ord, Ordering};
use std::collections::HashMap;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    x: usize,
    y: usize,
    z: usize,
    ordering: i32,
}

impl Position {
    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn z(&self) -> usize {
        self.z
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Position) -> Ordering {
        self.ordering.cmp(&other.ordering)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

pub struct Grid {
    pub current_sealevel: usize,
    dimensions: (usize, usize, usize),
    map: HashMap<(usize, usize), (usize, bool)>,
    pub held_tile: Option<Tile>,
}

impl Default for Grid {
    fn default() -> Grid {
        Grid::new(8, 8, 16)
    }
}

impl Grid {
    pub fn new(width: usize, height: usize, depth: usize) -> Grid {
        Grid {
            current_sealevel: 0,
            dimensions: (width, height, depth),
            map: HashMap::new(),
            held_tile: None,
        }
    }

    pub fn new_position(&mut self, tile: Tile, x: usize, y: usize, z: usize) -> Position {
        let civilized = if let Tile::Structure(_) = tile {
            true
        } else {
            false
        };
        {
            let (height, civ) = self.map.entry((x, y)).or_insert((z, civilized));
            if *height < z {
                *height = z;
            }
            *civ = civilized;
        }
        let (w, h, d) = self.dimensions();
        Position {
            x,
            y,
            z,
            ordering: (x as i32) - (y as i32) * (h as i32).pow(2) + (z as i32) * (d as i32).pow(3),
        }
    }

    pub fn lower_heightmap(&mut self, x: usize, y: usize) {
        if let Some((height, _)) = self.map.get_mut(&(x, y)) {
            *height -= 1;
        }
    }

    pub fn dimensions(&self) -> (usize, usize, usize) {
        self.dimensions
    }

    pub fn is_top_tile(&self, pos: &Position) -> bool {
        if let Some((height, _)) = self.map.get(&(pos.x(), pos.y())) {
            return pos.z() == *height;
        }
        false
    }

    pub fn is_civilized(&self, x: usize, y: usize) -> bool {
        if let Some((_, civilized)) = self.map.get(&(x, y)) {
            return *civilized;
        }
        false
    }

    pub fn uncivilize(&mut self, x: usize, y: usize) {
        if let Some((_, civilized)) = self.map.get_mut(&(x, y)) {
            *civilized = false;
        }
    }

    pub fn is_civilizable(&self, x: usize, y: usize) -> bool {
        let (w, h, d) = self.dimensions();
        (y > 0 && if let Some((_, civilized)) = self.map.get(&(x, y - 1)) {
            *civilized
        } else {
            false
        }) || (y < h && if let Some((_, civilized)) = self.map.get(&(x, y + 1)) {
            *civilized
        } else {
            false
        }) || (x > 0 && if let Some((_, civilized)) = self.map.get(&(x - 1, y)) {
            *civilized
        } else {
            false
        }) || (x < w && if let Some((_, civilized)) = self.map.get(&(x + 1, y)) {
            *civilized
        } else {
            false
        })
    }
}

pub struct GridGravity;

impl<'a> System<'a> for GridGravity {
    type SystemData = (Write<'a, Grid>, WriteStorage<'a, Position>);

    fn run(&mut self, (grid, positions): Self::SystemData) {}
}
