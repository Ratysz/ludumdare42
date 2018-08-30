use noise::{NoiseFn, Perlin, Seedable};
use rand;
use std::collections::{HashMap, VecDeque};
use std::fmt::{self, Display, Formatter};

use super::TileType;

pub enum PlaceRejectionReason {
    Invalid,
    Flooded,
    TooFar,
    TooHigh,
    NotShore,
    Occupied,
}

impl Display for PlaceRejectionReason {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let string = match self {
            PlaceRejectionReason::Invalid => "something is horribly wrong",
            PlaceRejectionReason::Flooded => "flooded",
            PlaceRejectionReason::TooFar => "too far from city",
            PlaceRejectionReason::TooHigh => "practically in space",
            PlaceRejectionReason::NotShore => "not a shore",
            PlaceRejectionReason::Occupied => "occupied",
        };
        write!(f, "{}", string)
    }
}

pub enum PickRejectionReason {
    Invalid,
    Flooded,
    TooFar,
    TooDeep,
    Occupied,
}

impl Display for PickRejectionReason {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let string = match self {
            PickRejectionReason::Invalid => "something is horribly wrong",
            PickRejectionReason::Flooded => "flooded",
            PickRejectionReason::TooFar => "too far from city",
            PickRejectionReason::TooDeep => "can't dig deeper",
            PickRejectionReason::Occupied => "can't pick up more",
        };
        write!(f, "{}", string)
    }
}

struct TileStack {
    tiles: VecDeque<TileType>,
    civilized: bool,
    flooded: bool,
}

impl TileStack {
    fn new(depth: usize) -> TileStack {
        TileStack {
            tiles: VecDeque::with_capacity(depth),
            civilized: false,
            flooded: false,
        }
    }

    fn height(&self) -> usize {
        self.tiles.len()
    }
}

pub struct Grid {
    pub sea_level: usize,
    dimensions: (usize, usize, usize),
    map: VecDeque<VecDeque<TileStack>>,
    pub held_tile: Option<TileType>,
    noise: Perlin,
}

impl Default for Grid {
    fn default() -> Grid {
        Grid::new(8, 8, 16, rand::random())
    }
}

impl Grid {
    pub fn new(width: usize, height: usize, depth: usize, seed: u32) -> Grid {
        let mut map = VecDeque::with_capacity(height);
        for i in 0..height {
            let mut column = VecDeque::with_capacity(width);
            for j in 0..width {
                column.push_back(TileStack::new(depth));
            }
            map.push_back(column);
        }
        Grid {
            sea_level: 0,
            dimensions: (width, height, depth),
            map,
            held_tile: None,
            noise: Perlin::new().set_seed(seed),
        }
    }

    pub fn dimensions(&self) -> (usize, usize, usize) {
        self.dimensions
    }

    pub fn tile(&self, x: usize, y: usize, z: usize) -> Option<TileType> {
        if let Some(stack) = self.stack(x, y) {
            return stack.tiles.get(z).map(|tile| *tile);
        }
        None
    }

    pub fn height(&self, x: usize, y: usize) -> usize {
        if let Some(stack) = self.stack(x, y) {
            return stack.height();
        }
        usize::max_value()
    }

    pub fn noise(&self) -> Perlin {
        self.noise
    }

    fn stack(&self, x: usize, y: usize) -> Option<&TileStack> {
        if let Some(row) = self.map.get(y) {
            return row.get(x);
        }
        None
    }

    fn stack_mut(&mut self, x: usize, y: usize) -> Option<&mut TileStack> {
        if let Some(row) = self.map.get_mut(y) {
            return row.get_mut(x);
        }
        None
    }

    fn adjacent_stacks(&self, x: usize, y: usize) -> [Option<&TileStack>; 4] {
        if let Some(row) = self.map.get(y) {
            [
                x.checked_add(1).and_then(|x| row.get(x)),
                x.checked_sub(1).and_then(|x| row.get(x)),
                y.checked_add(1)
                    .and_then(|y| self.map.get(y))
                    .and_then(|row| row.get(x)),
                y.checked_sub(1)
                    .and_then(|y| self.map.get(y))
                    .and_then(|row| row.get(x)),
            ]
        } else {
            [None, None, None, None]
        }
    }

    pub fn can_place(
        &self,
        x: usize,
        y: usize,
        tile: TileType,
    ) -> Result<(), PlaceRejectionReason> {
        if let Some(stack) = self.stack(x, y) {
            if stack.height() >= self.dimensions.2 {
                return Err(PlaceRejectionReason::TooHigh);
            }
            if stack.flooded {
                if tile == TileType::Water {
                    return Ok(());
                }
                return Err(PlaceRejectionReason::Flooded);
            }
            if stack.civilized {
                return Err(PlaceRejectionReason::Occupied);
            }
            for adjacent in &self.adjacent_stacks(x, y) {
                if adjacent.map(|adjacent| adjacent.civilized) == Some(true) {
                    return Ok(());
                }
            }
            return Err(PlaceRejectionReason::TooFar);
        }
        Err(PlaceRejectionReason::Invalid)
    }

    pub fn can_pick(&self, x: usize, y: usize) -> Result<TileType, PickRejectionReason> {
        if self.held_tile.is_some() {
            return Err(PickRejectionReason::Occupied);
        }
        if let Some(stack) = self.stack(x, y) {
            if stack.flooded {
                return Err(PickRejectionReason::Flooded);
            }
            for adjacent in &self.adjacent_stacks(x, y) {
                if adjacent.map(|adjacent| adjacent.civilized) == Some(true) {
                    return stack
                        .tiles
                        .back()
                        .map(|tile| *tile)
                        .ok_or(PickRejectionReason::TooDeep);
                }
            }
            return Err(PickRejectionReason::TooFar);
        }
        Err(PickRejectionReason::Invalid)
    }

    pub fn place(&mut self, tile: TileType, x: usize, y: usize) {
        let ceiling = self.dimensions.2;
        if let Some(stack) = self.stack_mut(x, y) {
            if stack.height() < ceiling {
                stack.tiles.push_back(tile);
            }
        }
    }

    pub fn remove(&mut self, x: usize, y: usize, z: usize) -> Option<TileType> {
        if let Some(stack) = self.stack_mut(x, y) {
            return stack.tiles.remove(z);
        }
        None
    }

    //ordering: (x as i32) - (y as i32) * (h as i32).pow(2) + (z as i32) * (d as i32).pow(3),
}
