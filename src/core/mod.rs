use crate::core::clock::Clock;

pub mod clock;

struct Runtime {
    clock: Clock,
}

impl Runtime {
    pub fn init() -> Self {
        Runtime {
            clock: Clock::default(),
        }
    }

    pub fn run() {}

    pub fn shutdown() {}
}
