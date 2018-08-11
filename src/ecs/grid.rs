use super::*;
use std::cmp::{Ord, Ordering};
use std::collections::VecDeque;

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
    dimensions: (usize, usize, usize),
    grid: VecDeque<VecDeque<VecDeque<Option<Index>>>>,
}

impl Default for Grid {
    fn default() -> Grid {
        Grid::new(8, 8, 8)
    }
}

impl Grid {
    pub fn new(width: usize, height: usize, depth: usize) -> Grid {
        let mut grid = VecDeque::new();
        for _ in 0..width {
            grid.push_back({
                let mut row = VecDeque::new();
                for _ in 0..height {
                    row.push_back({
                        let mut stack = VecDeque::<Option<Index>>::new();
                        for _ in 0..depth {
                            stack.push_back(None);
                        }
                        stack
                    });
                }
                row
            });
        }
        Grid {
            dimensions: (width, height, depth),
            grid: VecDeque::new(),
        }
    }

    pub fn new_position(&mut self, entity: Entity, x: usize, y: usize, z: usize) -> Position {
        if let Some(mut row) = self.grid.get_mut(x) {
            if let Some(mut stack) = row.get_mut(y) {
                if let Some(mut tile) = stack.get_mut(z) {
                    *tile = Some(entity.id());
                }
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

    pub fn entity_at(&self, x: usize, y: usize, z: usize) -> Option<Index> {
        if let Some(row) = self.grid.get(x) {
            if let Some(stack) = row.get(y) {
                if let Some(tile) = stack.get(z) {
                    return *tile;
                }
            }
        }
        None
    }
}

pub struct GridGravity;

impl<'a> System<'a> for GridGravity {
    type SystemData = (Write<'a, Grid>, WriteStorage<'a, Position>);

    fn run(&mut self, (mut grid, positions): Self::SystemData) {}
}
