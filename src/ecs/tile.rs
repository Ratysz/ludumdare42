use super::*;
use assets::DrawableHandle;

#[derive(Component, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Tile {
    Free,
    Occupied(()),
    Water,
    Terrain,
}
