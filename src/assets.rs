use super::ecs::TILE_SIZE;
use ggez::graphics::{self, Color, DrawMode, Drawable, Image, Mesh, MeshBuilder};
use ggez::{Context, GameResult};
use nalgebra as na;
use rand;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum DrawableHandle {
    Circle,
    Box,
    Tile,
    TileSelector,
    Terraform,
    Trees,
    Housing,
    Sanctuary,
    Powerplant,
    Renewables,
    Farm,
    Fishery,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum DrawableType {
    Mesh,
    Sprite,
}

pub struct Assets {
    meshes: HashMap<DrawableHandle, Mesh>,
    sprites: HashMap<DrawableHandle, Image>,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let mut meshes = HashMap::new();
        let mut sprites = HashMap::new();

        meshes.insert(
            DrawableHandle::Circle,
            Mesh::new_circle(ctx, DrawMode::Fill, na::Point2::origin(), 0.2, 0.1)?,
        );

        meshes.insert(
            DrawableHandle::Box,
            Mesh::new_polygon(
                ctx,
                DrawMode::Fill,
                &[
                    na::Point2::new(-0.2, -0.2),
                    na::Point2::new(0.2, -0.2),
                    na::Point2::new(0.2, 0.2),
                    na::Point2::new(-0.2, 0.2),
                ],
            )?,
        );

        meshes.insert(
            DrawableHandle::Tile,
            Mesh::new_polygon(
                ctx,
                DrawMode::Fill,
                &[
                    na::Point2::new(0.0, -0.5),
                    na::Point2::new(1.0, 0.0),
                    na::Point2::new(1.0, 0.25),
                    na::Point2::new(0.0, 0.75),
                    na::Point2::new(-1.0, 0.25),
                    na::Point2::new(-1.0, 0.0),
                ],
            )?,
        );

        meshes.insert(
            DrawableHandle::TileSelector,
            Mesh::new_polyline(
                ctx,
                DrawMode::Line(1.0),
                &[
                    na::Point2::new(-1.0 * TILE_SIZE.0, 0.0 * TILE_SIZE.1),
                    na::Point2::new(0.0 * TILE_SIZE.0, -0.5 * TILE_SIZE.1),
                    na::Point2::new(1.0 * TILE_SIZE.0, 0.0 * TILE_SIZE.1),
                    na::Point2::new(0.0 * TILE_SIZE.0, 0.5 * TILE_SIZE.1),
                    na::Point2::new(-1.0 * TILE_SIZE.0, 0.0 * TILE_SIZE.1),
                    na::Point2::new(-1.0 * TILE_SIZE.0, 0.25 * TILE_SIZE.1),
                    na::Point2::new(0.0 * TILE_SIZE.0, 0.75 * TILE_SIZE.1),
                    na::Point2::new(0.0 * TILE_SIZE.0, 0.5 * TILE_SIZE.1),
                    na::Point2::new(0.0 * TILE_SIZE.0, 0.75 * TILE_SIZE.1),
                    na::Point2::new(1.0 * TILE_SIZE.0, 0.25 * TILE_SIZE.1),
                    na::Point2::new(1.0 * TILE_SIZE.0, 0.0 * TILE_SIZE.1),
                ],
            )?,
        );

        sprites.insert(
            DrawableHandle::Trees,
            Image::from_bytes(
                ctx,
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/trees.png")),
            )?,
        );

        meshes.insert(
            DrawableHandle::Terraform,
            Mesh::new_circle(ctx, DrawMode::Fill, na::Point2::origin(), 0.2, 0.1)?,
        );

        meshes.insert(
            DrawableHandle::Housing,
            Mesh::new_circle(ctx, DrawMode::Fill, na::Point2::origin(), 0.2, 0.1)?,
        );

        meshes.insert(
            DrawableHandle::Sanctuary,
            Mesh::new_circle(ctx, DrawMode::Fill, na::Point2::origin(), 0.2, 0.1)?,
        );

        meshes.insert(
            DrawableHandle::Powerplant,
            Mesh::new_circle(ctx, DrawMode::Fill, na::Point2::origin(), 0.2, 0.1)?,
        );

        meshes.insert(
            DrawableHandle::Renewables,
            Mesh::new_circle(ctx, DrawMode::Fill, na::Point2::origin(), 0.2, 0.1)?,
        );

        meshes.insert(
            DrawableHandle::Farm,
            Mesh::new_circle(ctx, DrawMode::Fill, na::Point2::origin(), 0.2, 0.1)?,
        );

        meshes.insert(
            DrawableHandle::Fishery,
            Mesh::new_circle(ctx, DrawMode::Fill, na::Point2::origin(), 0.2, 0.1)?,
        );

        Ok(Assets { meshes, sprites })
    }

    pub fn fetch_mesh(&self, handle: DrawableHandle) -> &impl Drawable {
        self.meshes.get(&handle).unwrap()
    }

    pub fn fetch_sprite(&self, handle: DrawableHandle) -> &impl Drawable {
        self.sprites.get(&handle).unwrap()
    }
}

pub fn random_color() -> Color {
    Color::new(
        rand::random::<f32>(),
        rand::random::<f32>(),
        rand::random::<f32>(),
        1.0,
    )
}
