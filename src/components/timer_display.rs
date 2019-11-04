use super::component_prelude::*;

#[derive(Default)]
pub struct TimerDisplay;

impl Component for TimerDisplay {
    type Storage = NullStorage<Self>;
}
