use ggez::graphics::Color;
use specs::prelude::*;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::ops::Deref;

use super::Grid;
use assets::{ColorGenerator, DrawableHandle};

pub const TILE_SIZE: (f32, f32) = (32.0, 32.0);

#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TileDrawable(DrawableHandle);

impl TileDrawable {
    pub fn new(handle: DrawableHandle) -> TileDrawable {
        TileDrawable(handle)
    }
}

impl Deref for TileDrawable {
    type Target = DrawableHandle;

    fn deref(&self) -> &DrawableHandle {
        &self.0
    }
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct TileColor(Color);

impl TileColor {
    pub fn new(generator: ColorGenerator, z: usize, sea_level: usize, depth: usize) -> TileColor {
        TileColor(generator.generate(z, sea_level, depth))
    }

    pub fn from_color(color: Color) -> TileColor {
        TileColor(color)
    }
}

impl Deref for TileColor {
    type Target = Color;

    fn deref(&self) -> &Color {
        &self.0
    }
}

mod color_serde {
    use super::TileColor as RealTileColor;
    use ggez::graphics::Color;
    use serde::de::{Deserialize, Deserializer};
    use serde::ser::{Serialize, Serializer};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct TileColor(u32);

    impl Serialize for RealTileColor {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            TileColor(self.to_rgba_u32()).serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for RealTileColor {
        fn deserialize<D>(deserializer: D) -> Result<RealTileColor, D::Error>
        where
            D: Deserializer<'de>,
        {
            TileColor::deserialize(deserializer)
                .map(|x| RealTileColor::from_color(Color::from_rgba_u32(x.0)))
        }
    }
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TileTooltip(String);

impl TileTooltip {
    pub fn new(tooltip: String) -> TileTooltip {
        TileTooltip(tooltip)
    }
}

impl Deref for TileTooltip {
    type Target = String;

    fn deref(&self) -> &String {
        &self.0
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TilePosition {
    x: usize,
    y: usize,
    z: usize,
    ordering: i32,
}

impl TilePosition {
    pub fn new(
        x: usize,
        y: usize,
        z: usize,
        grid_dimensions: (usize, usize, usize),
    ) -> TilePosition {
        TilePosition {
            x,
            y,
            z,
            ordering: {
                let (x, y, z) = (x as i32, y as i32, z as i32);
                let (w, h, d) = grid_dimensions;
                let (w, h, d) = (w as i32, h as i32, d as i32);
                x - y * h * h + z * d * d * d
            },
        }
    }

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

impl Ord for TilePosition {
    fn cmp(&self, other: &TilePosition) -> Ordering {
        self.ordering.cmp(&other.ordering)
    }
}

impl PartialOrd for TilePosition {
    fn partial_cmp(&self, other: &TilePosition) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileType {
    Water,
    Terrain,
    Trees,
    Housing,
    Sanctuary,
    Powerplant,
    Renewables,
    Farm,
    Fishery,
}
/*TileType::Water => (
                DrawableHandle::Tile,
                ColorGenerator::Water,
                param.scale(na::Vector2::new(TILE_SIZE.0, TILE_SIZE.1)),
            ),
            TileType::Terrain => (
                DrawableHandle::TileSprite,
                ColorGenerator::Terrain,
                param.offset(na::Vector2::new(-TILE_SIZE.0, -0.5 * TILE_SIZE.1)),
            ),
            TileType::Trees => (
                DrawableHandle::Trees,
                ColorGenerator::Tint(0.3, 0.8, 0.3),
                param.offset(na::Vector2::new(-TILE_SIZE.0, -0.5 * TILE_SIZE.1)),
            ),*/

/*pub fn hit_test(ctx: &Context, pos: na::Point2<f32>) -> bool {
    let mouse = mouse::get_position(ctx);
    let (x, y) = ((mouse.x - pos.x).abs(), (mouse.y - pos.y).abs());
    x < TILE_SIZE.0 && y < 0.5 * TILE_SIZE.1 && x / TILE_SIZE.0 + 0.5 * y / TILE_SIZE.1 < 1.0
}

pub fn map_pos_to_screen(pos: &Position) -> na::Point2<f32> {
    na::Point2::new(
        100.0 - (320.0 / TILE_SIZE.0 as f32)
            + (pos.x() as f32 * TILE_SIZE.0)
            + (pos.y() as f32 * TILE_SIZE.1),
        240.0 + (pos.x() as f32 * TILE_SIZE.0 * 0.5)
            - (pos.y() as f32 * TILE_SIZE.1 * 0.5)
            - (pos.z() as f32 * TILE_SIZE.0 * 0.25),
    )
}*/
