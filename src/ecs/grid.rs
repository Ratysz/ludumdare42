use super::*;
use std::collections::{HashMap, VecDeque};
use std::fmt::{self, Display, Formatter};

pub enum PlaceRejectionReason {
    Invalid,
    Flooded,
    TooFar,
    NotShore,
    Occupied,
    TooHigh,
}

impl Display for PlaceRejectionReason {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let string = match self {
            PlaceRejectionReason::Invalid => "something is horribly wrong",
            PlaceRejectionReason::Flooded => "flooded",
            PlaceRejectionReason::TooFar => "too far from city",
            PlaceRejectionReason::NotShore => "not a shore",
            PlaceRejectionReason::Occupied => "occupied",
            PlaceRejectionReason::TooHigh => "practically in space",
        };
        write!(f, "{}", string)
    }
}

pub enum PickRejectionReason {
    Invalid,
    Flooded,
    TooFar,
    TooDeep,
}

impl Display for PickRejectionReason {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let string = match self {
            PickRejectionReason::Invalid => "something is horribly wrong",
            PickRejectionReason::Flooded => "flooded",
            PickRejectionReason::TooFar => "too far from city",
            PickRejectionReason::TooDeep => "can't dig deeper",
        };
        write!(f, "{}", string)
    }
}

struct TileStack {
    tiles: VecDeque<Tile>,
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
    pub current_sealevel: usize,
    dimensions: (usize, usize, usize),
    map: VecDeque<VecDeque<TileStack>>,
    pub held_tile: Option<Tile>,
}

impl Default for Grid {
    fn default() -> Grid {
        Grid::new(8, 8, 16)
    }
}

impl Grid {
    pub fn new(width: usize, height: usize, depth: usize) -> Grid {
        let mut map = VecDeque::with_capacity(height);
        for i in 0..height {
            let mut column = VecDeque::with_capacity(width);
            for j in 0..width {
                column.push_back(TileStack::new(depth));
            }
            map.push_back(column);
        }
        Grid {
            current_sealevel: 0,
            dimensions: (width, height, depth),
            map,
            held_tile: None,
        }
    }

    pub fn dimensions(&self) -> (usize, usize, usize) {
        self.dimensions
    }

    pub fn tile(&self, x: usize, y: usize, z: usize) -> Option<Tile> {
        if let Some(stack) = self.stack(x, y) {
            return stack.tiles.get(z).map(|tile| *tile);
        }
        None
    }

    fn stack(&self, x: usize, y: usize) -> Option<&TileStack> {
        if let Some(row) = self.map.get(y) {
            return row.get(x);
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

    pub fn can_place(&self, x: usize, y: usize, tile: Tile) -> Result<(), PlaceRejectionReason> {
        if let Some(stack) = self.stack(x, y) {
            if stack.height() >= self.dimensions.2 {
                return Err(PlaceRejectionReason::TooHigh);
            }
            if stack.flooded {
                if tile == Tile::Water {
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

    pub fn can_pick(&self, x: usize, y: usize) -> Result<Tile, PickRejectionReason> {
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

    pub fn place(&self, x: usize, y: usize, tile: Tile) {}

    pub fn remove(&self, x: usize, y: usize, z: usize) {}
    //ordering: (x as i32) - (y as i32) * (h as i32).pow(2) + (z as i32) * (d as i32).pow(3),
}
