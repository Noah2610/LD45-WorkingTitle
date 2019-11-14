use std::time::{Duration, Instant};

use amethyst::utils::fps_counter::FpsCounter;

use super::system_prelude::*;

pub struct DebugSystem {
    to_print:       Vec<String>,
    last_fps_print: Instant,
}

const PRINT_EVERY_MS: u64 = 1000;

impl<'a> System<'a> for DebugSystem {
    type SystemData = (
        ReadExpect<'a, Settings>,
        Read<'a, FpsCounter>,
        Read<'a, PlayerDeaths>,
        Read<'a, TimerRes>,
    );

    fn run(
        &mut self,
        (settings, fps_counter, player_deaths, timer): Self::SystemData,
    ) {
        let now = Instant::now();
        let debug_settings = &settings.debug;

        if now - self.last_fps_print >= Duration::from_millis(PRINT_EVERY_MS) {
            if debug_settings.print_fps {
                self.print_fps(&fps_counter);
            }
            if debug_settings.print_deaths {
                self.print_deaths(&player_deaths);
            }
            if debug_settings.print_time {
                self.print_time(&timer);
            }
            self.last_fps_print = now;
        }

        self.print();
    }
}

impl DebugSystem {
    fn print(&mut self) {
        if !self.to_print.is_empty() {
            println!("{}", self.to_print.join("\n"));
        }
        self.to_print.clear();
    }

    fn print_fps(&mut self, fps_counter: &FpsCounter) {
        let fps_frame = fps_counter.frame_fps();
        let fps_avg = fps_counter.sampled_fps();
        self.push_text(format!(
            "fps: {:.02} (avg: {:.02})",
            fps_frame, fps_avg
        ));
    }

    fn print_deaths(&mut self, player_deaths: &PlayerDeaths) {
        self.push_text(format!("player deaths: {}", player_deaths.0));
    }

    fn print_time(&mut self, timer_res: &TimerRes) {
        if let Some(timer) = timer_res.0.as_ref() {
            self.push_text(format!("time: {}", timer.time_output()));
        }
    }

    fn push_text<S>(&mut self, text: S)
    where
        S: ToString,
    {
        self.to_print.push(text.to_string());
    }
}

impl Default for DebugSystem {
    fn default() -> Self {
        Self {
            to_print:       Vec::new(),
            last_fps_print: Instant::now(),
        }
    }
}
