use std::time::{Duration, Instant};

use super::system_prelude::*;

const UPDATE_TIMER_MS: u64 = 250;

pub struct TimerSystem {
    last_update:           Instant,
    update_timer_duration: Duration,
}

impl<'a> System<'a> for TimerSystem {
    type SystemData = Write<'a, TimerRes>;

    fn run(&mut self, mut timer_res: Self::SystemData) {
        let now = Instant::now();

        if timer_res.0.state.is_running()
            && now.duration_since(self.last_update)
                >= self.update_timer_duration
        {
            timer_res.0.update().unwrap();
        }
    }
}

impl Default for TimerSystem {
    fn default() -> Self {
        Self {
            last_update:           Instant::now(),
            update_timer_duration: Duration::from_millis(UPDATE_TIMER_MS),
        }
    }
}
