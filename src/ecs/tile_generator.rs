use specs::prelude::*;

use super::{Grid, TileColor, TileDrawable, TilePosition, TileTooltip, TileType};
use assets::{ColorGenerator, DrawableHandle};

pub type TileCreationSystemData<'a> = (
    ReadStorage<'a, TileType>,
    ReadStorage<'a, TileDrawable>,
    ReadStorage<'a, TileColor>,
    ReadStorage<'a, TileTooltip>,
    ReadStorage<'a, TilePosition>,
);

pub fn create(
    entities: &Entities,
    updater: &LazyUpdate,
    grid: &mut Grid,
    tile_type: TileType,
    x: usize,
    y: usize,
) -> Entity {
    let (drawable, color, tooltip) = match tile_type {
        TileType::Water => (DrawableHandle::Tile, ColorGenerator::Water, "Water"),
        TileType::Terrain => (
            DrawableHandle::TileSprite,
            ColorGenerator::Terrain,
            "Terrain",
        ),
        TileType::Trees => (
            DrawableHandle::Trees,
            ColorGenerator::Tint(0.3, 0.8, 0.3),
            "Trees",
        ),
        TileType::Housing => (
            DrawableHandle::Housing,
            ColorGenerator::Tint(0.7, 0.7, 0.9),
            "Housing",
        ),
        TileType::Sanctuary => (
            DrawableHandle::Sanctuary,
            ColorGenerator::Tint(0.9, 0.9, 0.9),
            "Polar Bear Sanctuary",
        ),
        TileType::Powerplant => (
            DrawableHandle::Powerplant,
            ColorGenerator::Tint(0.9, 0.9, 0.9),
            "Powerplant",
        ),
        TileType::Renewables => (
            DrawableHandle::Renewables,
            ColorGenerator::Tint(0.9, 0.9, 0.9),
            "Renewables",
        ),
        TileType::Farm => (
            DrawableHandle::Farm,
            ColorGenerator::Tint(0.9, 0.9, 0.9),
            "Farm",
        ),
        TileType::Fishery => (
            DrawableHandle::Fishery,
            ColorGenerator::Tint(0.9, 0.9, 0.9),
            "Fishing Pier",
        ),
    };

    grid.place(tile_type, x, y);
    let z = grid.height(x, y);
    let drawable = TileDrawable::new(drawable);
    let color = TileColor::new(color, z, grid.sea_level, grid.dimensions().2);
    let tooltip = TileTooltip::new(tooltip.to_owned());
    let position = TilePosition::new(x, y, z, grid.dimensions());

    debug!("Created {:?} at ({}, {}, {})", tile_type, x, y, z);

    updater
        .create_entity(entities)
        .with(tile_type)
        .with(drawable)
        .with(color)
        .with(tooltip)
        .with(position)
        .build()
}
