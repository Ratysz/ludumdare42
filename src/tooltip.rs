use ecs::TILE_SIZE;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect, Text};
use ggez::{Context, GameResult};
use nalgebra as na;

pub fn draw(ctx: &mut Context, pos: na::Point2<f32>, text: &Text) -> GameResult {
    let dim = text.dimensions(ctx);
    let vec = na::Vector2::new(text.width(ctx) as f32 * 0.5, 0.5 * TILE_SIZE.1);
    let rect = Mesh::new_rectangle(
        ctx,
        DrawMode::Fill,
        Rect::new(0.0, 0.0, dim.0 as f32, dim.1 as f32),
    )?;
    graphics::draw(
        ctx,
        &rect,
        DrawParam::new()
            .dest(pos - vec)
            .color(Color::new(0.0, 0.0, 0.0, 0.7)),
    )?;
    graphics::draw(ctx, text, DrawParam::new().dest(pos - vec))
}
