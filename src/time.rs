use specs::prelude::*;
pub use std::time::Duration;

pub struct Time {
    delta: Duration,
    turn: u32,
}

impl Default for Time {
    fn default() -> Time {
        Time::new()
    }
}

impl Time {
    pub fn new() -> Time {
        Time {
            delta: Duration::from_secs(0),
            turn: 0,
        }
    }

    pub fn update_delta(&mut self, delta: Duration) {
        self.delta = delta;
    }

    pub fn increment_turn(&mut self) {
        self.turn += 1;
    }

    pub fn turn(&self) -> u32 {
        self.turn
    }

    pub fn delta(&self) -> Duration {
        self.delta
    }
}
