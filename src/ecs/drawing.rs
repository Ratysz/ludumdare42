use ggez::graphics::{self, DrawParam};
use ggez::{Context, GameResult};
use nalgebra as na;
use specs::prelude::*;
use specs::world::EntitiesRes;

use super::{TileColor, TileDrawable, TilePosition, TILE_SIZE};
use assets::Assets;

pub type DrawTilesSystemData<'a> = (
    Entities<'a>,
    ReadStorage<'a, TileDrawable>,
    ReadStorage<'a, TileColor>,
    ReadStorage<'a, TilePosition>,
);

pub fn draw_tiles(ctx: &mut Context, assets: &mut Assets, world: &mut World) -> GameResult {
    let (entities, drawables, colors, positions) = world.system_data::<DrawTilesSystemData>();
    let mut sorted = (&*entities, &positions, &drawables)
        .join()
        .collect::<Vec<_>>();
    sorted.sort_by_key(|(_, &pos, _)| pos);
    for (entity, position, drawable) in sorted.iter() {
        let mut param = DrawParam::new().dest(map_pos_to_screen(position));
        if let Some(color) = colors.get(*entity) {
            param = param.color(**color);
        }
        graphics::draw(ctx, assets.drawable(***drawable), param)?;
    }
    Ok(())
}

pub fn map_pos_to_screen(pos: &TilePosition) -> na::Point2<f32> {
    na::Point2::new(
        100.0 - (320.0 / TILE_SIZE.0 as f32)
            + (pos.x() as f32 * TILE_SIZE.0)
            + (pos.y() as f32 * TILE_SIZE.1),
        240.0 + (pos.x() as f32 * TILE_SIZE.0 * 0.5)
            - (pos.y() as f32 * TILE_SIZE.1 * 0.5)
            - (pos.z() as f32 * TILE_SIZE.0 * 0.25),
    )
}
