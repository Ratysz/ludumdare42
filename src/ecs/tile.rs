use super::*;
use ggez::graphics::{self, Color, DrawParam, Text};
use ggez::input::mouse;
use ggez::{Context, GameResult};
use nalgebra as na;

use assets::{random_color, Assets, DrawableHandle};
use gui;

pub const TILE_SIZE: (f32, f32) = (30.0, 30.0);

#[derive(Component, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Tile {
    Water,
    Terrain,
    Trees,
}

impl Tile {
    pub fn draw(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        pos: &Position,
        is_top: bool,
    ) -> GameResult {
        match self {
            Tile::Water => graphics::draw(
                ctx,
                assets.fetch_drawable(DrawableHandle::Tile),
                DrawParam::new()
                    .dest(map_pos_to_screen(pos))
                    .color(map_pos_to_water_color(pos))
                    .scale(na::Vector2::new(TILE_SIZE.0, TILE_SIZE.1)),
            ),
            Tile::Terrain => graphics::draw(
                ctx,
                assets.fetch_drawable(DrawableHandle::Tile),
                DrawParam::new()
                    .dest(map_pos_to_screen(pos))
                    .color(map_pos_to_terrain_color(pos))
                    .scale(na::Vector2::new(TILE_SIZE.0, TILE_SIZE.1)),
            ),
            Tile::Trees => graphics::draw(
                ctx,
                assets.fetch_drawable(DrawableHandle::Box),
                DrawParam::new()
                    .dest(map_pos_to_screen(pos))
                    .color(Color::new(0.1, 0.6, 0.2, 1.0))
                    .scale(na::Vector2::new(TILE_SIZE.0, TILE_SIZE.1)),
            ),
        }
    }

    pub fn draw_tooltip(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        pos: &Position,
    ) -> GameResult<bool> {
        let pos = map_pos_to_screen(pos);
        if hit_test(ctx, pos) {
            graphics::draw(
                ctx,
                assets.fetch_drawable(DrawableHandle::TileSelector),
                DrawParam::new().dest(pos).color(random_color()),
            )?;
            let pos = pos - na::Vector2::new(0.0, TILE_SIZE.1);
            match self {
                Tile::Water => {
                    gui::draw_tooltip(ctx, assets, &Text::new("Water"), pos)?;
                }
                Tile::Terrain => {
                    gui::draw_tooltip(ctx, assets, &Text::new("Terrain"), pos)?;
                }
                Tile::Trees => {
                    gui::draw_tooltip(ctx, assets, &Text::new("Trees"), pos)?;
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

pub fn hit_test(ctx: &Context, pos: na::Point2<f32>) -> bool {
    let mouse = mouse::get_position(ctx);
    let (x, y) = ((mouse.x - pos.x).abs(), (mouse.y - pos.y).abs());
    x < TILE_SIZE.0 && y < 0.5 * TILE_SIZE.1 && x / TILE_SIZE.0 + 0.5 * y / TILE_SIZE.1 < 1.0
}

pub fn map_pos_to_screen(pos: &Position) -> na::Point2<f32> {
    na::Point2::new(
        110.0 + (pos.x() as f32 * TILE_SIZE.0) + (pos.y() as f32 * TILE_SIZE.1),
        240.0 + (pos.x() as f32 * TILE_SIZE.0 * 0.5)
            - (pos.y() as f32 * TILE_SIZE.1 * 0.5)
            - (pos.z() as f32 * TILE_SIZE.0 * 0.25),
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
