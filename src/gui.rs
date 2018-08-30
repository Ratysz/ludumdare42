use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect, Text, TextFragment, WHITE};
use ggez::{Context, GameResult};
use nalgebra as na;

use ecs::{Time, TILE_SIZE};

pub fn draw_tooltip(ctx: &mut Context, pos: na::Point2<f32>, text: &Text) -> GameResult {
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

pub const RED: Color = Color {
    r: 1.0,
    g: 0.2,
    b: 0.2,
    a: 1.0,
};

pub fn draw_score(ctx: &mut Context, time: Time) -> GameResult {
    let pos = na::Point2::new(0.0, 0.0);
    let mut text = Text::new(TextFragment::new(format!(
        "Turn: {}  Score: {}\n",
        time.turn, time.score,
    )));
    text.add(if time.flood_timer < 1 {
        TextFragment::new(format!("Sea level rises on next turn!\n",)).color(RED)
    } else {
        TextFragment::new(format!("Sea level rises in {} turns\n", time.flood_timer))
    }).add(TextFragment::new(format!(
            "Population: {}\n",
            time.population
        )))
        .add(
            TextFragment::new(format!("   {} are homeless\n", time.homeless.max(0))).color(if time
                .homeless
                <= 0
            {
                WHITE
            } else {
                RED
            }),
        )
        .add(TextFragment::new(format!(
            "   growth in {} turns\n",
            time.population_timer
        )))
        .add(
            TextFragment::new(format!("Food: {}\n", time.food)).color(if time.food >= 0 {
                WHITE
            } else {
                RED
            }),
        )
        .add(
            TextFragment::new(format!("Power: {}\n", time.power)).color(if time.power >= 0 {
                WHITE
            } else {
                RED
            }),
        )
        .add(TextFragment::new(format!("Nature: {}\n", time.nature)));
    graphics::draw(ctx, &text, DrawParam::new().dest(pos))
}
