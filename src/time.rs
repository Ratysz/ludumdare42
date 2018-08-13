use ecs::*;
use specs::prelude::*;

pub struct Time {
    pub game_over: bool,
    pub score: i32,
    pub turn_passed: bool,
    pub turn: i32,
    pub flood_timer: i32,
    pub population: i32,
    pub population_timer: i32,
    pub nature: i32,
    pub power: i32,
    pub homeless: i32,
    pub food: i32,
}

impl Default for Time {
    fn default() -> Time {
        Time::new()
    }
}

impl Time {
    pub fn new() -> Time {
        Time {
            game_over: false,
            score: 0,
            turn_passed: false,
            turn: 0,
            flood_timer: 8,
            population: 3,
            population_timer: 3,
            nature: -1,
            power: 0,
            homeless: 0,
            food: 0,
        }
    }
}

pub struct AllThingsDoer;

impl<'a> System<'a> for AllThingsDoer {
    type SystemData = (
        Entities<'a>,
        Write<'a, Time>,
        Write<'a, Grid>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Tile>,
    );

    fn run(&mut self, (entities, mut time, mut grid, mut positions, mut tiles): Self::SystemData) {
        if time.population_timer < 1 {
            time.population += 1;
            time.population_timer = 3;
        }
        if time.food < 0 && time.homeless > 0 {
            let delta = (-time.food).min(time.homeless);
            time.population -= delta;
        }
        time.nature = 0;
        time.power = 0;
        time.homeless = time.population;
        time.food = -time.population;
        let mut sanctuary_bonus = 0;
        for tile in tiles.join() {
            match tile {
                Tile::Structure(structure) => match structure {
                    Structure::Housing => {
                        time.homeless -= 1;
                        time.power -= 1;
                    }
                    Structure::Sanctuary => {
                        sanctuary_bonus += 1;
                        time.power -= 1;
                    }
                    Structure::Powerplant => {
                        time.nature -= 1;
                        time.power += 3;
                    }
                    Structure::Renewables => {
                        time.power += 2;
                    }
                    Structure::Farm => {
                        time.food += 2;
                    }
                    Structure::Fishery => {
                        time.food += 3;
                    }
                },
                _ => (),
            }
        }
        if time.power >= 0 {
            time.nature += sanctuary_bonus;
        }
        if time.food >= 0 {
            time.population_timer -= 1;
        }
        time.flood_timer -= 1 - time.nature.min(0);
        time.score +=
            time.population - time.homeless + time.turn + time.food.min(0) + time.nature.min(0);
        time.turn += 1;
        if time.population == 0 {
            time.game_over = true;
        }
    }
}
