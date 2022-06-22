use bracket_random::prelude::RandomNumberGenerator;
use std::sync::Mutex;

// Note: it's Mutex locked so it doesn't have to be ResMut

pub struct Rng {
    rng: Mutex<RandomNumberGenerator>
}

impl Rng {
    pub fn new() -> Self {
        Self { rng: Mutex::new(RandomNumberGenerator::new()) }
    }

    pub fn range(&self, start: i32, end: i32) -> i32 {
        let mut lock = self.rng.lock().unwrap();
        lock.range(start, end)
    }

    pub fn random_slice_entry<'a, T>(&self, slice: &'a [T]) -> Option<&'a T> {
        let mut lock = self.rng.lock().unwrap();
        lock.random_slice_entry(slice)
    }
}