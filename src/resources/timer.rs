use climer::{Output, Timer};

pub struct TimerRes(pub Timer);

impl Default for TimerRes {
    fn default() -> Self {
        Self(Timer::new(
            None,
            Some(Output::new::<char, char>(None, None, None)),
        ))
    }
}
