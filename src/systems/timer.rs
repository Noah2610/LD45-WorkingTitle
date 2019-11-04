use std::time::{Duration, Instant};

use super::system_prelude::*;

const UPDATE_TIMER_MS: u64 = 250;

pub struct TimerSystem {
    last_update:           Instant,
    update_timer_duration: Duration,
}

impl<'a> System<'a> for TimerSystem {
    type SystemData = (
        Read<'a, ShouldDisplayTimer>,
        Write<'a, TimerRes>,
        ReadStorage<'a, UiTransform>,
        WriteStorage<'a, UiText>,
    );

    fn run(
        &mut self,
        (
            should_display_timer,
            mut timer_res,
            ui_transforms,
            mut ui_texts,
        ): Self::SystemData,
    ) {
        if let Some(timer) = timer_res.0.as_mut() {
            let now = Instant::now();

            // Print to stdout
            if timer.state.is_running()
                && now.duration_since(self.last_update)
                    >= self.update_timer_duration
            {
                timer.update().unwrap();

                // Display timer
                if should_display_timer.0 {
                    if let Some(text) = (&ui_transforms, &mut ui_texts)
                        .join()
                        .find_map(|(transform, text)| {
                            if &transform.id == "timer_display" {
                                Some(text)
                            } else {
                                None
                            }
                        })
                    {
                        text.text = timer.time_output().to_string();
                    }
                }

                self.last_update = now;
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
