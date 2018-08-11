use super::*;
use ggez::graphics::{self, Color, DrawParam};
use ggez::input::mouse;
use ggez::{Context, GameResult};
use nalgebra as na;

use assets::{Assets, DrawableHandle};
use gui;

pub const TILE_SIZE_PX: (f32, f32) = (30.0, 30.0);

#[derive(Component, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Tile {
    Free,
    Occupied(()),
    Water,
    Terrain,
}

impl Tile {
    pub fn draw(&self, ctx: &mut Context, assets: &Assets, pos: &Position) -> GameResult {
        match self {
            Tile::Water => {
                graphics::draw(
                    ctx,
                    assets.fetch_drawable(DrawableHandle::FullTile),
                    DrawParam::new()
                        .dest(map_pos_to_screen(pos))
                        .color(map_pos_to_water_color(pos))
                        .scale(na::Vector2::new(TILE_SIZE_PX.0, TILE_SIZE_PX.1)),
                )?;
            }
            Tile::Terrain => {
                graphics::draw(
                    ctx,
                    assets.fetch_drawable(DrawableHandle::FullTile),
                    DrawParam::new()
                        .dest(map_pos_to_screen(pos))
                        .color(map_pos_to_terrain_color(pos))
                        .scale(na::Vector2::new(TILE_SIZE_PX.0, TILE_SIZE_PX.1)),
                )?;
            }
            _ => (),
        }
        Ok(())
    }

    pub fn draw_tooltip(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        pos: &Position,
    ) -> GameResult<bool> {
        let mouse = mouse::get_position(ctx);
        let pos = map_pos_to_screen(pos);
        if (mouse.x - pos.x).abs() < 1.0 * TILE_SIZE_PX.0
            && (mouse.y - pos.y).abs() < 0.5 * TILE_SIZE_PX.1
        {
            match self {
                Tile::Water => {
                    gui::draw_tooltip(ctx, assets, "Water", pos)?;
                    Ok(true)
                }
                Tile::Terrain => {
                    gui::draw_tooltip(ctx, assets, "Terrain", pos)?;
                    Ok(true)
                }
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }
}

fn map_pos_to_screen(pos: &Position) -> na::Point2<f32> {
    na::Point2::new(
        110.0 + (pos.x() as f32 * TILE_SIZE_PX.0) + (pos.y() as f32 * TILE_SIZE_PX.1),
        240.0 + (pos.x() as f32 * TILE_SIZE_PX.0 * 0.5)
            - (pos.y() as f32 * TILE_SIZE_PX.1 * 0.5)
            - (pos.z() as f32 * TILE_SIZE_PX.0 * 0.25),
    )
}

fn map_pos_to_water_color(pos: &Position) -> Color {
    Color::new(
        0.0,
        0.2 * ((pos.z() as f32 + 1.0) / 4.0).min(1.0),
        1.0 * ((pos.z() as f32 + 1.0) / 4.0).min(1.0),
        0.5,
    )
}

fn map_pos_to_terrain_color(pos: &Position) -> Color {
    Color::new(
        0.4 * ((pos.z() as f32 + 1.0) / 12.0).min(1.0),
        0.8 * ((pos.z() as f32 + 1.0) / 12.0).min(1.0),
        0.0,
        1.0,
    )
}
