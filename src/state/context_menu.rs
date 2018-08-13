use super::*;
use nalgebra as na;
use specs::world::Index;
use std::f32::INFINITY;

pub struct ContextMenu {
    is_top: bool,
    target_id: Index,
    target_tile: Tile,
    target_pos: Position,
    enabled: bool,
    options: Vec<(na::Vector2<f32>, (SpriteHandle, bool))>,
}

impl ContextMenu {
    pub fn new<'c>(ctx: &Context, world: &'c mut World) -> Option<ContextMenu> {
        let grid = world.read_resource::<Grid>();
        let entities = world.entities();
        let positions = world.read_storage::<Position>();
        let tiles = world.read_storage::<Tile>();
        for (entity, pos, tile) in (&*entities, &positions, &tiles).join() {
            if grid.is_top_tile(pos) && tile::hit_test(ctx, tile::map_pos_to_screen(pos)) {
                debug!("Target: {:?} at {:?} ({:?})", tile, pos, entity);
                let civ = grid.is_civilizable(pos.x(), pos.y());
                let options = vec![
                    (
                        na::Vector2::new(-3.3 * TILE_SIZE.0, 0.5 * TILE_SIZE.0),
                        (SpriteHandle::Housing, civ),
                    ),
                    (
                        na::Vector2::new(-1.1 * TILE_SIZE.0, 0.5 * TILE_SIZE.0),
                        (SpriteHandle::Powerplant, civ),
                    ),
                    (
                        na::Vector2::new(1.1 * TILE_SIZE.0, 0.5 * TILE_SIZE.0),
                        (SpriteHandle::Fishery, civ),
                    ),
                    (
                        na::Vector2::new(3.3 * TILE_SIZE.0, 0.5 * TILE_SIZE.0),
                        (SpriteHandle::Farm, civ),
                    ),
                    (
                        na::Vector2::new(-2.2 * TILE_SIZE.0, 1.5 * TILE_SIZE.0),
                        (SpriteHandle::Sanctuary, civ),
                    ),
                    (
                        na::Vector2::new(0.0 * TILE_SIZE.0, 1.5 * TILE_SIZE.0),
                        (SpriteHandle::Terraform, civ),
                    ),
                    (
                        na::Vector2::new(2.2 * TILE_SIZE.0, 1.5 * TILE_SIZE.0),
                        (SpriteHandle::Renewables, civ),
                    ),
                ];
                return Some(ContextMenu {
                    is_top: false,
                    target_id: entity.id(),
                    target_tile: *tile,
                    target_pos: *pos,
                    enabled: civ,
                    options,
                });
            }
        }
        None
    }
}

impl State for ContextMenu {
    fn start(
        &mut self,
        _ctx: &mut Context,
        _assets: &mut Assets,
        _world: &mut World,
    ) -> GameResult {
        self.is_top = true;
        Ok(())
    }

    fn stop(&mut self, _ctx: &mut Context, _assets: &mut Assets, _world: &mut World) -> GameResult {
        self.is_top = false;
        Ok(())
    }

    fn pause(
        &mut self,
        _ctx: &mut Context,
        _assets: &mut Assets,
        _world: &mut World,
    ) -> GameResult {
        self.is_top = false;
        Ok(())
    }

    fn resume(
        &mut self,
        _ctx: &mut Context,
        _assets: &mut Assets,
        _world: &mut World,
    ) -> GameResult {
        self.is_top = true;
        Ok(())
    }

    fn input(
        &mut self,
        _ctx: &mut Context,
        _assets: &mut Assets,
        _world: &mut World,
        _command: Command,
        _extra: InputExtra,
    ) -> GameResult<Transition> {
        Ok(Transition::Pop)
    }

    fn draw(&mut self, _ctx: &mut Context, _assets: &mut Assets, _world: &mut World) -> GameResult {
        let pos = tile::map_pos_to_screen(&self.target_pos);
        graphics::draw(
            _ctx,
            _assets.fetch_mesh(MeshHandle::TileSelector),
            DrawParam::new().dest(pos).color(graphics::BLACK),
        )?;
        let mut i = 0;
        let mut tooltip_drawn = false;
        for (vec, (drawable, enabled)) in &self.options {
            graphics::draw(
                _ctx,
                _assets.fetch_mesh(MeshHandle::Tile),
                DrawParam::new()
                    .dest(pos + vec + na::Vector2::new(0.0, 0.25 * TILE_SIZE.1))
                    .color(Color::new(0.0, 0.0, 0.0, 0.95))
                    .scale(na::Vector2::new(TILE_SIZE.0, TILE_SIZE.1)),
            )?;
            graphics::draw(
                _ctx,
                _assets.fetch_sprite(*drawable),
                DrawParam::new()
                    .dest(pos + vec + na::Vector2::new(-TILE_SIZE.0, -0.5 * TILE_SIZE.1)),
            )?;
            if self.enabled && !tooltip_drawn && tile::hit_test(_ctx, pos + vec) {
                graphics::draw(
                    _ctx,
                    _assets.fetch_mesh(MeshHandle::TileSelector),
                    DrawParam::new().dest(pos + vec).color(random_color()),
                )?;
                tooltip_drawn = true;
                let mut text = Text::new(TextFragment::new(match i {
                    0 => "Build a ",
                    1 => "Build a ",
                    2 => if *enabled {
                        "Build a "
                    } else {
                        "Needs to be on shore!"
                    },
                    3 => "Build a ",
                    4 => "Build a ",
                    5 => if *enabled {
                        "Place stored tile"
                    } else {
                        "Can't place here!"
                    },
                    6 => "Build ",
                    _ => panic!("that shouldn't happen"),
                }));
                text.add(
                    TextFragment::new(match i {
                        0 => "house",
                        1 => "power plant",
                        2 => if *enabled {
                            "fishing pier"
                        } else {
                            ""
                        },
                        3 => "farm",
                        4 => "polar bear freezer",
                        5 => "",
                        6 => "eco power generators",
                        _ => panic!("that shouldn't happen"),
                    }).color(Color::new(0.1, 0.5, 0.5, 1.0)),
                );
                text.add(TextFragment::new(match i {
                    0 => " (+1 housing, -1 power)",
                    1 => " (+3 power, -1 nature)",
                    2 => if *enabled {
                        " (+3 food)"
                    } else {
                        ""
                    },
                    3 => " (+2 food)",
                    4 => " (+1 nature, -1 power)",
                    5 => "",
                    6 => " (+2 power)",
                    _ => panic!("that shouldn't happen"),
                }));
                let vec = na::Vector2::new(text.width(_ctx) as f32 * 0.5, 0.5 * TILE_SIZE.1);
                let dim = text.dimensions(_ctx);
                let rect = graphics::Mesh::new_rectangle(
                    _ctx,
                    graphics::DrawMode::Fill,
                    graphics::Rect::new(0.0, 0.0, dim.0 as f32, dim.1 as f32),
                )?;
                graphics::draw(
                    _ctx,
                    &rect,
                    DrawParam::new()
                        .dest(pos - vec)
                        .color(Color::new(0.0, 0.0, 0.0, 0.6)),
                )?;
                graphics::draw(_ctx, &text, DrawParam::new().dest(pos - vec))?;
            }
            i += 1;
        }
        if !self.enabled {
            let text = Text::new("Too far from city!");
            let vec = na::Vector2::new(text.width(_ctx) as f32 * 0.5, 0.5 * TILE_SIZE.1);
            let dim = text.dimensions(_ctx);
            let rect = graphics::Mesh::new_rectangle(
                _ctx,
                graphics::DrawMode::Fill,
                graphics::Rect::new(0.0, 0.0, dim.0 as f32, dim.1 as f32),
            )?;
            graphics::draw(
                _ctx,
                &rect,
                DrawParam::new()
                    .dest(pos - vec)
                    .color(Color::new(0.0, 0.0, 0.0, 0.6)),
            )?;
            graphics::draw(_ctx, &text, DrawParam::new().dest(pos - vec))?;
        }
        /*let mut text = Text::new(format!("This is {:?}\n", tile));
        text.add(format!(
            "Civilized: {}\n",
            grid.is_civilized(pos.x(), pos.y())
        )).add(format!(
            "Civilizable: {}\n",
            grid.is_civilizable(pos.x(), pos.y())
        ))
            .add(format!("Height is {}\n", pos.z()))
            .add(format!("X {} Y {}\n", pos.x(), pos.y()));
        graphics::draw(_ctx, &text, (pos, graphics::WHITE));*/
        Ok(())
    }

    fn draw_underlying(&self) -> bool {
        true
    }
}

impl Display for ContextMenu {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ContextMenu")
    }
}
