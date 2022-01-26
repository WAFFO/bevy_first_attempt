use std::time::{Duration, SystemTime, UNIX_EPOCH};

use oorandom::Rand32;

pub struct RandStruct {
    god: Rand32,
    map: Rand32,
    map_seed: u64,
}

impl RandStruct {
    pub fn new() -> Self {
        let mut timer = SystemTime::now();
        timer += Duration::from_secs(86400);
        let mut god = Rand32::new(timer.duration_since(UNIX_EPOCH).unwrap().as_secs());
        let map_seed = god.rand_u32() as u64 + god.rand_u32() as u64;
        let map = Rand32::new(map_seed);
        RandStruct { god, map, map_seed }
    }

    pub fn get_map_float(&mut self) -> f32 {
        self.map.rand_float()
    }

    pub fn randomize_map(&mut self) {
        self.map_seed = self.god.rand_u32() as u64 + self.god.rand_u32() as u64;
        self.map = Rand32::new(self.map_seed);
    }

    pub fn map_seed(&self) -> u64 {
        self.map_seed
    }
}
