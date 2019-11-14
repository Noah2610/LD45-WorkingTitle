use climer::Timer;

#[derive(Default)]
pub struct TimerRes(pub Option<Timer>);

impl TimerRes {
    pub fn add_timer(&mut self) {
        self.0 = Some(Timer::new(
            None,
            // Some(Output::new::<char, char>(None, None, None)),
            None,
        ));
    }

    pub fn remove_timer(&mut self) {
        self.0 = None;
    }
}
