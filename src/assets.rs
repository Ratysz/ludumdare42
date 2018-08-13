use super::ecs::TILE_SIZE;
use ggez::audio::{SoundData, Source};
use ggez::graphics::{Color, DrawMode, Drawable, Image, Mesh};
use ggez::{Context, GameResult};
use nalgebra as na;
use rand;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum MeshHandle {
    Circle,
    Box,
    Tile,
    TileSelector,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SpriteHandle {
    TileSprite,
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
pub enum SoundHandle {
    Click,
    Construct,
    WaveCrash,
    Waves,
}

pub struct Assets {
    meshes: HashMap<MeshHandle, Mesh>,
    sprites: HashMap<SpriteHandle, Image>,
    sounds: HashMap<SoundHandle, Source>,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let mut meshes = HashMap::new();
        let mut sprites = HashMap::new();
        let mut sounds = HashMap::new();

        meshes.insert(
            MeshHandle::Circle,
            Mesh::new_circle(ctx, DrawMode::Fill, na::Point2::origin(), 0.2, 0.1)?,
        );

        meshes.insert(
            MeshHandle::Box,
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
            MeshHandle::Tile,
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
            MeshHandle::TileSelector,
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
            SpriteHandle::TileSprite,
            Image::from_bytes(
                ctx,
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/tile.png")),
            )?,
        );

        sprites.insert(
            SpriteHandle::Trees,
            Image::from_bytes(
                ctx,
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/trees.png")),
            )?,
        );

        sprites.insert(
            SpriteHandle::Terraform,
            Image::from_bytes(
                ctx,
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/terraform.png")),
            )?,
        );

        sprites.insert(
            SpriteHandle::Housing,
            Image::from_bytes(
                ctx,
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/house.png")),
            )?,
        );

        sprites.insert(
            SpriteHandle::Sanctuary,
            Image::from_bytes(
                ctx,
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sanctuary.png")),
            )?,
        );

        sprites.insert(
            SpriteHandle::Powerplant,
            Image::from_bytes(
                ctx,
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/assets/powerplant.png"
                )),
            )?,
        );

        sprites.insert(
            SpriteHandle::Renewables,
            Image::from_bytes(
                ctx,
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/assets/renewables.png"
                )),
            )?,
        );

        sprites.insert(
            SpriteHandle::Farm,
            Image::from_bytes(
                ctx,
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/farm.png")),
            )?,
        );

        sprites.insert(
            SpriteHandle::Fishery,
            Image::from_bytes(
                ctx,
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fishery.png")),
            )?,
        );

        let mut source = Source::from_data(
            ctx,
            SoundData::from_bytes(include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/Click.ogg"
            ))),
        )?;
        let volume = source.volume();
        source.set_volume(0.5 * volume);
        sounds.insert(SoundHandle::Click, source);

        let mut source = Source::from_data(
            ctx,
            SoundData::from_bytes(include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/Construct.ogg"
            ))),
        )?;
        let volume = source.volume();
        source.set_volume(0.5 * volume);
        sounds.insert(SoundHandle::Construct, source);

        let mut source = Source::from_data(
            ctx,
            SoundData::from_bytes(include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/WaveCrash.ogg"
            ))),
        )?;
        let volume = source.volume();
        source.set_volume(0.5 * volume);
        sounds.insert(SoundHandle::WaveCrash, source);

        let mut source = Source::from_data(
            ctx,
            SoundData::from_bytes(include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/Waves.ogg"
            ))),
        )?;
        let volume = source.volume();
        source.set_volume(0.25 * volume);
        sounds.insert(SoundHandle::Waves, source);

        Ok(Assets {
            meshes,
            sprites,
            sounds,
        })
    }

    pub fn fetch_mesh(&self, handle: MeshHandle) -> &impl Drawable {
        self.meshes.get(&handle).unwrap()
    }

    pub fn fetch_sprite(&self, handle: SpriteHandle) -> &impl Drawable {
        self.sprites.get(&handle).unwrap()
    }

    pub fn fetch_sound(&mut self, handle: SoundHandle) -> &mut Source {
        self.sounds.get_mut(&handle).unwrap()
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
