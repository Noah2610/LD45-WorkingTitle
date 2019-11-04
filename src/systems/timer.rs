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
        if let Some(timer) = timer_res.0.as_mut() {
            let now = Instant::now();

            if timer.state.is_running()
                && now.duration_since(self.last_update)
                    >= self.update_timer_duration
            {
                timer.update().unwrap();
            }
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
