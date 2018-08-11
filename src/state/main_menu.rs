use super::*;

use time::*;

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
        let time = _world.read_resource::<Time>();
        if time.now() > Duration::from_secs(5) {
            Ok(Transition::Pop)
        } else {
            Ok(Transition::None)
        }
    }
}

impl Display for MainMenu {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Main Menu")
    }
}
