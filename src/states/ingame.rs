use super::state_prelude::*;

#[derive(Default)]
pub struct Ingame;

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Ingame {
    fn on_start(&mut self, data: StateData<CustomGameData<CustomData>>) {
        // Start timer
        if let Some(timer) = data.world.write_resource::<TimerRes>().0.as_mut()
        {
            if timer.state.is_stopped() || timer.state.is_finished() {
                timer.start().unwrap();
            }
        }
    }

    fn on_stop(&mut self, data: StateData<CustomGameData<CustomData>>) {
        // Stop timer
        if let Some(timer) = data.world.write_resource::<TimerRes>().0.as_mut()
        {
            if timer.state.is_running() || timer.state.is_paused() {
                // Quit to main menu from pause menu
                if data.world.read_resource::<ToMainMenu>().0 {
                    timer.stop().unwrap();
                // Beat the level
                } else {
                    timer.finish().unwrap();
                    println!("---\nLEVEL TIME: {}\n---", timer.time_output());
                }
            }
        }
    }

    fn on_resume(&mut self, data: StateData<CustomGameData<CustomData>>) {
        // Resume timer
        if let Some(timer) = data.world.write_resource::<TimerRes>().0.as_mut()
        {
            if timer.state.is_paused() {
                timer.resume().unwrap();
            }
        }
    }

    fn on_pause(&mut self, data: StateData<CustomGameData<CustomData>>) {
        // Pause timer
        if let Some(timer) = data.world.write_resource::<TimerRes>().0.as_mut()
        {
            if timer.state.is_running() {
                timer.pause().unwrap();
            }
        }
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(data.world, "ingame").unwrap();

        // Handle input
        if let Some(trans) = self.handle_keys(&data.world) {
            return trans;
        }

        // Win game
        if data.world.read_resource::<WinGame>().0 {
            data.world.write_resource::<WinGame>().0 = false;
            return Trans::Switch(Box::new(Win::default()));
        }

        // To main menu (DifficultySelect)
        if data.world.read_resource::<ToMainMenu>().0 {
            return Trans::Pop;
        }

        Trans::None
    }
}

impl Ingame {
    fn handle_keys<'a, 'b>(
        &mut self,
        world: &World,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        let input = world.read_resource::<InputManager<Bindings>>();

        if input.is_down(ActionBinding::Quit) {
            Some(Trans::Quit)
        } else if input.is_down(ActionBinding::TogglePause) {
            Some(Trans::Push(Box::new(Paused::default())))
        } else {
            None
        }
    }
}
