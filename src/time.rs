use specs::prelude::*;
pub use std::time::Duration;

pub struct Time {
    elapsed: Duration,
    delta: Duration,
}

impl Default for Time {
    fn default() -> Time {
        Time::new()
    }
}

impl Time {
    pub fn new() -> Time {
        Time {
            elapsed: Duration::from_secs(0),
            delta: Duration::from_secs(0),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.delta = delta;
        self.elapsed += delta;
    }

    pub fn now(&self) -> Duration {
        self.elapsed
    }

    pub fn delta(&self) -> Duration {
        self.delta
    }
}
