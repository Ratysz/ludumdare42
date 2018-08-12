use ggez::graphics::{self, DrawMode, Drawable, Mesh, MeshBuilder};
use ggez::{Context, GameResult};
use nalgebra as na;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum DrawableHandle {
    Circle,
    Box,
    FullTile,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum DrawableType {
    Mesh,
}

pub struct Assets {
    types: HashMap<DrawableHandle, DrawableType>,
    meshes: HashMap<DrawableHandle, Mesh>,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let mut types = HashMap::new();
        let mut meshes = HashMap::new();

        types.insert(DrawableHandle::Circle, DrawableType::Mesh);
        meshes.insert(
            DrawableHandle::Circle,
            Mesh::new_circle(ctx, DrawMode::Fill, na::Point2::origin(), 0.2, 0.1)?,
        );

        types.insert(DrawableHandle::Box, DrawableType::Mesh);
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

        types.insert(DrawableHandle::FullTile, DrawableType::Mesh);
        meshes.insert(
            DrawableHandle::FullTile,
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

        Ok(Assets { types, meshes })
    }

    pub fn fetch_drawable(&self, handle: DrawableHandle) -> &impl Drawable {
        match self.types.get(&handle).unwrap() {
            DrawableType::Mesh => self.meshes.get(&handle).unwrap(),
        }
    }
}
