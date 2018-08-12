use super::*;
use ggez::graphics::{self, Color, DrawParam, Text};
use ggez::{Context, GameResult};
use nalgebra as na;

use assets::{Assets, DrawableHandle};

pub fn draw_tooltip(
    ctx: &mut Context,
    assets: &Assets,
    text: &Text,
    pos: na::Point2<f32>,
) -> GameResult {
    graphics::draw(ctx, text, (pos, graphics::WHITE))
}
