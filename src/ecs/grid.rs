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
    heightmap: HashMap<(usize, usize), usize>,
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
            heightmap: HashMap::new(),
        }
    }

    pub fn new_position(&mut self, entity: Entity, x: usize, y: usize, z: usize) -> Position {
        {
            let height = self.heightmap.entry((x, y)).or_insert(z);
            if *height < z {
                *height = z;
            }
        }
        let (w, h, d) = self.dimensions();
        Position {
            x,
            y,
            z,
            ordering: (x as i32) - (y as i32) * (h as i32).pow(2) + (z as i32) * (d as i32).pow(3),
        }
    }

    pub fn dimensions(&self) -> (usize, usize, usize) {
        self.dimensions
    }

    pub fn is_top_tile(&self, pos: &Position) -> bool {
        if let Some(height) = self.heightmap.get(&(pos.x(), pos.y())) {
            return pos.z() == *height;
        }
        false
    }
}

pub struct GridGravity;

impl<'a> System<'a> for GridGravity {
    type SystemData = (Write<'a, Grid>, WriteStorage<'a, Position>);

    fn run(&mut self, (grid, positions): Self::SystemData) {
        debug!("hi");
    }
}
