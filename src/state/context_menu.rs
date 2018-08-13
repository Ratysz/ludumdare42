use super::*;
use nalgebra as na;
use specs::world::Index;
use std::f32::INFINITY;

pub struct ContextMenu {
    is_top: bool,
    target_entity: Entity,
    target_tile: Tile,
    target_pos: Position,
    near_city: bool,
    in_water: bool,
    on_shore: bool,
    options: Vec<(na::Vector2<f32>, SpriteHandle)>,
}

impl ContextMenu {
    pub fn new<'c>(ctx: &Context, world: &'c mut World) -> Option<ContextMenu> {
        let grid = world.read_resource::<Grid>();
        let entities = world.entities();
        let positions = world.read_storage::<Position>();
        let tiles = world.read_storage::<Tile>();
        let target = (&*entities, &positions, &tiles)
            .join()
            .find(|(entity, pos, tile)| {
                grid.is_top_tile(pos) && tile::hit_test(ctx, tile::map_pos_to_screen(pos))
            });
        if let Some((entity, pos, tile)) = target {
            let in_water = *tile == Tile::Water;
            let (w, h, d) = grid.dimensions();
            let mut on_shore = false;
            for (other_pos, other_tile) in (&positions, &tiles).join().filter(|(p, _)| {
                (p.y() == pos.y()
                    && (p.x() > 0 && p.x() - 1 <= pos.x())
                    && (p.x() < w && p.x() + 1 >= pos.x()))
                    || (p.x() == pos.x()
                        && (p.y() > 0 && p.y() - 1 <= pos.y())
                        && (p.y() < h && p.y() + 1 >= pos.y()))
            }) {
                if *other_tile == Tile::Water {
                    on_shore = true;
                    break;
                }
            }
            debug!("Target: {:?} at {:?} ({:?})", tile, pos, entity);
            let civ = grid.is_civilizable(pos.x(), pos.y());
            let options = vec![
                (
                    na::Vector2::new(-3.3 * TILE_SIZE.0, 0.5 * TILE_SIZE.0),
                    SpriteHandle::Housing,
                ),
                (
                    na::Vector2::new(-1.1 * TILE_SIZE.0, 0.5 * TILE_SIZE.0),
                    SpriteHandle::Powerplant,
                ),
                (
                    na::Vector2::new(1.1 * TILE_SIZE.0, 0.5 * TILE_SIZE.0),
                    SpriteHandle::Fishery,
                ),
                (
                    na::Vector2::new(3.3 * TILE_SIZE.0, 0.5 * TILE_SIZE.0),
                    SpriteHandle::Farm,
                ),
                (
                    na::Vector2::new(-2.2 * TILE_SIZE.0, 1.5 * TILE_SIZE.0),
                    SpriteHandle::Sanctuary,
                ),
                (
                    na::Vector2::new(0.0 * TILE_SIZE.0, 1.5 * TILE_SIZE.0),
                    SpriteHandle::Terraform,
                ),
                (
                    na::Vector2::new(2.2 * TILE_SIZE.0, 1.5 * TILE_SIZE.0),
                    SpriteHandle::Renewables,
                ),
            ];
            return Some(ContextMenu {
                is_top: false,
                target_entity: entity,
                target_tile: *tile,
                target_pos: *pos,
                near_city: civ,
                in_water,
                on_shore,
                options,
            });
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
        if _command == Command::Click {
            let mut i = 0;
            let mut new_tile = None;
            let mut pick_or_place = false;
            let mut grid = _world.write_resource::<Grid>();
            for (vec, sprite) in &self.options {
                if self.near_city
                    && !self.in_water
                    && tile::hit_test(_ctx, tile::map_pos_to_screen(&self.target_pos) + vec)
                {
                    new_tile = match i {
                        0 => Some(Tile::Structure(Structure::Housing)),
                        1 => Some(Tile::Structure(Structure::Powerplant)),
                        2 => if self.on_shore {
                            Some(Tile::Structure(Structure::Fishery))
                        } else {
                            None
                        },
                        3 => Some(Tile::Structure(Structure::Farm)),
                        4 => Some(Tile::Structure(Structure::Sanctuary)),
                        5 => {
                            pick_or_place = true;
                            grid.held_tile
                        }
                        6 => Some(Tile::Structure(Structure::Renewables)),
                        _ => panic!("that shouldn't happen"),
                    };
                    break;
                }
                i += 1;
            }
            if let Some(new_tile) = new_tile {
                let entities = _world.entities();
                let mut positions = _world.write_storage::<Position>();
                let mut tiles = _world.write_storage::<Tile>();
                let mut modify = false;
                let mut place = false;
                for (entity, pos, tile) in (&*entities, &mut positions, &mut tiles).join() {
                    if *pos == self.target_pos {
                        match tile {
                            Tile::Trees => modify = true,
                            Tile::Terrain => place = true,
                            _ => {}
                        }
                    }
                }
                if modify {
                    if pick_or_place {
                        grid.held_tile = None;
                    }
                    debug!(
                        "replacing {} {} {}",
                        self.target_pos.x(),
                        self.target_pos.y(),
                        self.target_pos.z()
                    );
                    *tiles.get_mut(self.target_entity).unwrap() = new_tile;
                    grid.new_position(
                        new_tile,
                        self.target_pos.x(),
                        self.target_pos.y(),
                        self.target_pos.z(),
                    );
                } else if place {
                    if pick_or_place {
                        grid.held_tile = None;
                    }
                    let entity = entities.create();
                    positions
                        .insert(
                            entity,
                            grid.new_position(
                                new_tile,
                                self.target_pos.x(),
                                self.target_pos.y(),
                                self.target_pos.z() + 1,
                            ),
                        )
                        .unwrap();
                    tiles.insert(entity, new_tile).unwrap();
                }
            } else if pick_or_place {
                grid.held_tile = Some(self.target_tile);
                debug!(
                    "removing {} {} {}",
                    self.target_pos.x(),
                    self.target_pos.y(),
                    self.target_pos.z()
                );
                _world.entities().delete(self.target_entity);
                grid.uncivilize(self.target_pos.x(), self.target_pos.y());
                grid.lower_heightmap(self.target_pos.x(), self.target_pos.y());
                grid.new_position(
                    Tile::Terrain,
                    self.target_pos.x(),
                    self.target_pos.y(),
                    self.target_pos.z() - 1,
                );
            }
        }
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
        for (vec, sprite) in &self.options {
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
                _assets.fetch_sprite(*sprite),
                DrawParam::new()
                    .dest(pos + vec + na::Vector2::new(-TILE_SIZE.0, -0.5 * TILE_SIZE.1)),
            )?;
            if self.near_city && !self.in_water && !tooltip_drawn && tile::hit_test(_ctx, pos + vec)
            {
                graphics::draw(
                    _ctx,
                    _assets.fetch_mesh(MeshHandle::TileSelector),
                    DrawParam::new().dest(pos + vec).color(random_color()),
                )?;
                tooltip_drawn = true;
                let mut grid = _world.write_resource::<Grid>();
                let mut text = Text::new(TextFragment::new(match i {
                    0 => "Build ",
                    1 => "Build a ",
                    2 => if self.on_shore {
                        "Build a "
                    } else {
                        "Needs to be on shore!"
                    },
                    3 => "Build a ",
                    4 => "Build a ",
                    5 => if grid.held_tile != None {
                        "Place here"
                    } else {
                        "Pick up"
                    },
                    6 => "Build ",
                    _ => panic!("that shouldn't happen"),
                }));
                text.add(
                    TextFragment::new(match i {
                        0 => "Housing",
                        1 => "Power Plant",
                        2 => if self.on_shore {
                            "Fishing Pier"
                        } else {
                            ""
                        },
                        3 => "Farm",
                        4 => "Polar Bear Sanctuary",
                        5 => "",
                        6 => "Eco Power Generators",
                        _ => panic!("that shouldn't happen"),
                    }).color(Color::new(0.1, 0.6, 0.6, 1.0)),
                );
                text.add(TextFragment::new(match i {
                    0 => " (+1 housing, -1 power)",
                    1 => " (+3 power, -1 nature)",
                    2 => if self.on_shore {
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
                tooltip::draw(_ctx, pos, &text);
            }
            i += 1;
        }
        if !self.near_city {
            let text = Text::new("Too far from city!");
            tooltip::draw(_ctx, pos, &text);
        } else if self.in_water {
            let text = Text::new("Can't build on water!");
            tooltip::draw(_ctx, pos, &text);
        }
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
