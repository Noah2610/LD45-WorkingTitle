use std::time::{Duration, Instant};

use amethyst::utils::fps_counter::FpsCounter;

use super::system_prelude::*;

pub struct DebugSystem {
    last_fps_print: Instant,
}

const PRINT_EVERY_MS: u64 = 1000;

impl<'a> System<'a> for DebugSystem {
    type SystemData = (Read<'a, FpsCounter>, Read<'a, PlayerDeaths>);

    fn run(&mut self, (fps_counter, player_deaths): Self::SystemData) {
        let now = Instant::now();
        if now - self.last_fps_print >= Duration::from_millis(PRINT_EVERY_MS) {
            let fps_frame = fps_counter.frame_fps();
            let fps_avg = fps_counter.sampled_fps();
            println!("fps: {:.02} (avg: {:.02})", fps_frame, fps_avg);
            println!("player deaths: {}", player_deaths.0);
            self.last_fps_print = now;
        }
    }
}

impl Default for DebugSystem {
    fn default() -> Self {
        Self {
            last_fps_print: Instant::now(),
        }
    }
}
