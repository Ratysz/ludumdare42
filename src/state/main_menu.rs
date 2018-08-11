use super::*;

pub struct MainMenu;

impl State for MainMenu {
    fn start(&mut self, _ctx: &mut Context, _world: &mut World) -> GameResult {
        info!("Main menu, go!");
        Ok(())
    }

    fn stop(&mut self, _ctx: &mut Context, _world: &mut World) -> GameResult {
        info!("Main menu, gone!");
        Ok(())
    }

    fn update(&mut self, _ctx: &mut Context, _world: &mut World) -> GameResult<Transition> {
        Ok(Transition::Push(Box::new(super::Game::new(_world))))
    }
}

impl Display for MainMenu {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Main Menu")
    }
}
