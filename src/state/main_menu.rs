use super::*;

pub struct MainMenu;

const MULTIPLIER: f32 = 32.0 / TILE_SIZE.0 as f32;

impl State for MainMenu {
    fn start(
        &mut self,
        _ctx: &mut Context,
        _assets: &mut Assets,
        _world: &mut World,
    ) -> GameResult {
        info!("Main menu, go!");
        _assets.sound(SoundHandle::Waves).set_repeat(true);
        _assets.sound(SoundHandle::Waves).play();
        Ok(())
    }

    fn stop(&mut self, _ctx: &mut Context, _assets: &mut Assets, _world: &mut World) -> GameResult {
        info!("Main menu, gone!");
        Ok(())
    }

    fn update(
        &mut self,
        _ctx: &mut Context,
        _assets: &mut Assets,
        _world: &mut World,
    ) -> GameResult<Transition> {
        _world.delete_all();
        _world.maintain();
        *_world.res.entry::<Grid>().or_insert_with(|| {
            Grid::new(
                (8.0 * MULTIPLIER).floor() as usize,
                (8.0 * MULTIPLIER).floor() as usize,
                (16.0 * MULTIPLIER).floor() as usize,
            )
        }) = Grid::new(
            (8.0 * MULTIPLIER).floor() as usize,
            (8.0 * MULTIPLIER).floor() as usize,
            (16.0 * MULTIPLIER).floor() as usize,
        );
        *_world.res.entry::<Time>().or_insert_with(Time::new) = Time::new();
        Ok(Transition::Push(Box::new(super::Game::new(_world))))
    }
}

impl Display for MainMenu {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Main Menu")
    }
}
