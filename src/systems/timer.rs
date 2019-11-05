use std::time::{Duration, Instant};

use super::system_prelude::*;

const UPDATE_TIMER_MS: u64 = 250;
const TIMER_UI_TRANSFORM_ID: &str = "timer";
const BEST_TIME_UI_TRANSFORM_ID: &str = "best_time";

pub struct TimerSystem {
    last_update:           Instant,
    update_timer_duration: Duration,
    last_time_string:      String,
    has_set_best_time:     bool,
}

impl<'a> System<'a> for TimerSystem {
    type SystemData = (
        ReadExpect<'a, Settings>,
        Read<'a, ShouldDisplayTimer>,
        Read<'a, BestTime>,
        Write<'a, TimerRes>,
        ReadStorage<'a, UiTransform>,
        WriteStorage<'a, UiText>,
    );

    fn run(
        &mut self,
        (
            settings,
            should_display_timer,
            best_time_res,
            mut timer_res,
            ui_transforms,
            mut ui_texts,
        ): Self::SystemData,
    ) {
        if let Some(timer) = timer_res.0.as_mut() {
            let timer_settings = &settings.timer;
            let now = Instant::now();

            // Print to stdout
            if timer.state.is_running()
                && now.duration_since(self.last_update)
                    >= self.update_timer_duration
            {
                timer.update().unwrap();

                // Display timer and best time
                if should_display_timer.0 {
                    let new_text = format!(
                        "{}{}",
                        &timer_settings.time_prefix,
                        timer.time_output()
                    );
                    if new_text.as_str() != self.last_time_string.as_str() {
                        // Display running timer
                        if let Some(text) = get_text_with_id(
                            TIMER_UI_TRANSFORM_ID,
                            &ui_transforms,
                            &mut ui_texts,
                        ) {
                            self.last_time_string = new_text.clone();
                            text.text = new_text;
                        }
                    }

                    // Display best time
                    if let Some(best_time) = best_time_res.0.as_ref() {
                        if !self.has_set_best_time {
                            if let Some(text) = get_text_with_id(
                                BEST_TIME_UI_TRANSFORM_ID,
                                &ui_transforms,
                                &mut ui_texts,
                            ) {
                                text.text = format!(
                                    "{}{}",
                                    &timer_settings.best_time_prefix,
                                    &best_time
                                );
                                self.has_set_best_time = true;
                            }
                        }
                    }
                }

                self.last_update = now;
            }
        }
    }
}

fn get_text_with_id<'a, 'b>(
    target_id: &'a str,
    ui_transforms: &'a ReadStorage<'b, UiTransform>,
    ui_texts: &'a mut WriteStorage<'b, UiText>,
) -> Option<&'a mut UiText> {
    (ui_transforms, ui_texts)
        .join()
        .find_map(|(transform, text)| {
            if transform.id.as_str() == target_id {
                Some(text)
            } else {
                None
            }
        })
}

impl Default for TimerSystem {
    fn default() -> Self {
        Self {
            last_update:           Instant::now(),
            update_timer_duration: Duration::from_millis(UPDATE_TIMER_MS),
            last_time_string:      String::new(),
            has_set_best_time:     false,
        }
    }
}
