use super::*;
use ggez::graphics::{self, Color, DrawParam, Text};
use ggez::input::mouse;
use ggez::{Context, GameResult};
use nalgebra as na;

use assets::{random_color, Assets, MeshHandle, SpriteHandle};

pub const TILE_SIZE: (f32, f32) = (32.0, 32.0);

#[derive(Component, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Tile {
    Water,
    Terrain,
    Trees,
    Structure(Structure),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Structure {
    Housing,
    Sanctuary,
    Powerplant,
    Renewables,
    Farm,
    Fishery,
}

impl Tile {
    pub fn draw(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        pos: &Position,
        sealevel: usize,
        depth: usize,
        is_top: bool,
    ) -> GameResult {
        match self {
            Tile::Water => graphics::draw(
                ctx,
                assets.fetch_mesh(MeshHandle::Tile),
                DrawParam::new()
                    .dest(map_pos_to_screen(pos))
                    .color(map_pos_to_water_color(pos.z(), sealevel, depth))
                    .scale(na::Vector2::new(TILE_SIZE.0, TILE_SIZE.1)),
            ),
            Tile::Terrain => graphics::draw(
                ctx,
                assets.fetch_sprite(SpriteHandle::TileSprite),
                DrawParam::new()
                    .dest(
                        map_pos_to_screen(pos) + na::Vector2::new(-TILE_SIZE.0, -0.5 * TILE_SIZE.1),
                    )
                    .color(map_pos_to_terrain_color(pos.z(), sealevel, depth)),
            ),
            Tile::Trees => graphics::draw(
                ctx,
                assets.fetch_sprite(SpriteHandle::Trees),
                DrawParam::new()
                    .dest(
                        map_pos_to_screen(pos) + na::Vector2::new(-TILE_SIZE.0, -0.5 * TILE_SIZE.1),
                    )
                    .color(Color::new(0.3, 0.8, 0.3, 1.0)),
            ),
            Tile::Structure(structure) => match structure {
                Structure::Housing => graphics::draw(
                    ctx,
                    assets.fetch_sprite(SpriteHandle::Housing),
                    DrawParam::new()
                        .dest(
                            map_pos_to_screen(pos)
                                + na::Vector2::new(-TILE_SIZE.0, -0.5 * TILE_SIZE.1),
                        )
                        .color(Color::new(0.7, 0.7, 0.9, 1.0)),
                ),
                Structure::Sanctuary => graphics::draw(
                    ctx,
                    assets.fetch_sprite(SpriteHandle::Sanctuary),
                    DrawParam::new().dest(
                        map_pos_to_screen(pos) + na::Vector2::new(-TILE_SIZE.0, -0.5 * TILE_SIZE.1),
                    ),
                ),
                Structure::Powerplant => graphics::draw(
                    ctx,
                    assets.fetch_sprite(SpriteHandle::Powerplant),
                    DrawParam::new()
                        .dest(
                            map_pos_to_screen(pos)
                                + na::Vector2::new(-TILE_SIZE.0, -0.5 * TILE_SIZE.1),
                        )
                        .color(Color::new(0.3, 0.3, 0.3, 1.0)),
                ),
                Structure::Renewables => graphics::draw(
                    ctx,
                    assets.fetch_sprite(SpriteHandle::Renewables),
                    DrawParam::new().dest(
                        map_pos_to_screen(pos) + na::Vector2::new(-TILE_SIZE.0, -0.5 * TILE_SIZE.1),
                    ),
                ),
                Structure::Farm => graphics::draw(
                    ctx,
                    assets.fetch_sprite(SpriteHandle::Farm),
                    DrawParam::new().dest(
                        map_pos_to_screen(pos) + na::Vector2::new(-TILE_SIZE.0, -0.4 * TILE_SIZE.1),
                    ),
                ),
                Structure::Fishery => graphics::draw(
                    ctx,
                    assets.fetch_sprite(SpriteHandle::Fishery),
                    DrawParam::new()
                        .dest(
                            map_pos_to_screen(pos)
                                + na::Vector2::new(-TILE_SIZE.0, -0.5 * TILE_SIZE.1),
                        )
                        .color(Color::new(0.5, 0.5, 1.0, 1.0)),
                ),
            },
        }
    }

    pub fn draw_tooltip(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        pos: &Position,
    ) -> GameResult<bool> {
        let z = pos.z();
        let pos = map_pos_to_screen(pos);
        if hit_test(ctx, pos) {
            graphics::draw(
                ctx,
                assets.fetch_mesh(MeshHandle::TileSelector),
                DrawParam::new().dest(pos).color(random_color()),
            )?;
            //let pos = pos - na::Vector2::new(0.0, TILE_SIZE.1);
            let text = Text::new(match self {
                Tile::Water => format!("Water ({})", z),
                Tile::Terrain => format!("Terrain ({})", z),
                Tile::Trees => format!("Trees ({})", z),
                Tile::Structure(s) => match s {
                    Structure::Housing => format!("Housing ({})", z),
                    Structure::Sanctuary => format!("Sanctuary ({})", z),
                    Structure::Powerplant => format!("Powerplant ({})", z),
                    Structure::Renewables => format!("Renewables ({})", z),
                    Structure::Farm => format!("Farm ({})", z),
                    Structure::Fishery => format!("Fishery ({})", z),
                },
            });
            tooltip::draw(ctx, pos, &text);
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
        100.0 - (320.0 / TILE_SIZE.0 as f32)
            + (pos.x() as f32 * TILE_SIZE.0)
            + (pos.y() as f32 * TILE_SIZE.1),
        240.0 + (pos.x() as f32 * TILE_SIZE.0 * 0.5)
            - (pos.y() as f32 * TILE_SIZE.1 * 0.5)
            - (pos.z() as f32 * TILE_SIZE.0 * 0.25),
    )
}

fn map_pos_to_water_color(z: usize, s: usize, d: usize) -> Color {
    Color::new(
        0.0,
        0.2 * ((0.5 * d as f32 + z as f32 - s as f32) / (0.5 * d as f32)).min(1.0),
        0.8 * ((0.5 * d as f32 + z as f32 - s as f32) / (0.5 * d as f32)).min(1.0),
        0.4,
    )
}

fn map_pos_to_terrain_color(z: usize, s: usize, d: usize) -> Color {
    let z = (z as f32 - s as f32) / d as f32;
    if z < -0.05 {
        Color::new(0.2, 0.1, 0.05, 1.0)
    } else if z < 0.05 {
        Color::new(0.8, 0.7, 0.1, 1.0)
    } else if z < 0.30 {
        Color::new(
            (1.0 * (0.4 - z)).min(0.4),
            (1.8 * (0.4 - z)).min(0.8),
            0.0,
            1.0,
        )
    } else if z < 0.40 {
        Color::new(
            (1.5 * z).min(1.0),
            (1.0 * z).min(1.0),
            (0.5 * z).min(1.0),
            1.0,
        )
    } else {
        Color::new(
            (2.0 * z).min(1.0),
            (2.0 * z).min(1.0),
            (2.0 * z).min(1.0),
            1.0,
        )
    }
}
