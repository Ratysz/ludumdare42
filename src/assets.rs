use super::ecs::TILE_SIZE;
use ggez::audio::{SoundData, Source};
use ggez::graphics::{Color, DrawMode, Drawable, Image, Mesh};
use ggez::{Context, GameResult};
use nalgebra as na;
use rand;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum DrawableHandle {
    Circle,
    Box,
    Tile,
    TileSelector,
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

pub enum ColorGenerator {
    Terrain,
    Water,
    Tint(f32, f32, f32),
    Random,
}

pub struct Assets {
    drawables: HashMap<DrawableHandle, Box<dyn Drawable>>,
    sounds: HashMap<SoundHandle, Source>,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let mut drawables = HashMap::<DrawableHandle, Box<dyn Drawable>>::new();
        let mut sounds = HashMap::new();

        drawables.insert(
            DrawableHandle::Circle,
            Box::new(Mesh::new_circle(
                ctx,
                DrawMode::Fill,
                na::Point2::origin(),
                0.2,
                0.1,
            )?),
        );

        drawables.insert(
            DrawableHandle::Box,
            Box::new(Mesh::new_polygon(
                ctx,
                DrawMode::Fill,
                &[
                    na::Point2::new(-0.2, -0.2),
                    na::Point2::new(0.2, -0.2),
                    na::Point2::new(0.2, 0.2),
                    na::Point2::new(-0.2, 0.2),
                ],
            )?),
        );

        drawables.insert(
            DrawableHandle::Tile,
            Box::new(Mesh::new_polygon(
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
            )?),
        );

        drawables.insert(
            DrawableHandle::TileSelector,
            Box::new(Mesh::new_polyline(
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
            )?),
        );

        drawables.insert(
            DrawableHandle::TileSprite,
            Box::new(Image::from_bytes(
                ctx,
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/tile.png")),
            )?),
        );

        drawables.insert(
            DrawableHandle::Trees,
            Box::new(Image::from_bytes(
                ctx,
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/trees.png")),
            )?),
        );

        drawables.insert(
            DrawableHandle::Terraform,
            Box::new(Image::from_bytes(
                ctx,
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/terraform.png")),
            )?),
        );

        drawables.insert(
            DrawableHandle::Housing,
            Box::new(Image::from_bytes(
                ctx,
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/house.png")),
            )?),
        );

        drawables.insert(
            DrawableHandle::Sanctuary,
            Box::new(Image::from_bytes(
                ctx,
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/sanctuary.png")),
            )?),
        );

        drawables.insert(
            DrawableHandle::Powerplant,
            Box::new(Image::from_bytes(
                ctx,
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/assets/powerplant.png"
                )),
            )?),
        );

        drawables.insert(
            DrawableHandle::Renewables,
            Box::new(Image::from_bytes(
                ctx,
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/assets/renewables.png"
                )),
            )?),
        );

        drawables.insert(
            DrawableHandle::Farm,
            Box::new(Image::from_bytes(
                ctx,
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/farm.png")),
            )?),
        );

        drawables.insert(
            DrawableHandle::Fishery,
            Box::new(Image::from_bytes(
                ctx,
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fishery.png")),
            )?),
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

        Ok(Assets { drawables, sounds })
    }

    pub fn drawable(&self, handle: DrawableHandle) -> &Drawable {
        self.drawables.get(&handle).unwrap().as_ref()
    }

    pub fn sound(&mut self, handle: SoundHandle) -> &mut Source {
        self.sounds.get_mut(&handle).unwrap()
    }
}

impl ColorGenerator {
    pub fn generate(self, z: usize, sea_level: usize, depth: usize) -> Color {
        match self {
            ColorGenerator::Terrain => {
                let z = (z as f32 - sea_level as f32) / depth as f32;
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
            ColorGenerator::Water => Color::new(
                0.0,
                0.2 * ((0.5 * depth as f32 + z as f32 - sea_level as f32) / (0.5 * depth as f32))
                    .min(1.0),
                0.8 * ((0.5 * depth as f32 + z as f32 - sea_level as f32) / (0.5 * depth as f32))
                    .min(1.0),
                0.4,
            ),
            ColorGenerator::Tint(r, g, b) => Color::new(r, g, b, 1.0),
            ColorGenerator::Random => Color::new(
                rand::random::<f32>(),
                rand::random::<f32>(),
                rand::random::<f32>(),
                1.0,
            ),
        }
    }
}
