use super::state_prelude::*;

const UI_TIMER_DISPLAY_RON_PATH: &str = "ui/timer_display.ron";

#[derive(Default)]
pub struct Ingame {
    ui_data: UiData,
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        // Start timer
        if let Some(timer) = data.world.write_resource::<TimerRes>().0.as_mut()
        {
            if timer.state.is_stopped() || timer.state.is_finished() {
                timer.start().unwrap();
            }
        }
        // Display timer
        if data.world.read_resource::<ShouldDisplayTimer>().0 {
            // self.create_timer_display(data.world);
            let _progress =
                self.create_ui(&mut data, resource(UI_TIMER_DISPLAY_RON_PATH));
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
        // NOTE: Don't delete on stop, so the time is still displayed in `Win` state.
        // Delete timer display
        // if data.world.read_resource::<ShouldDisplayTimer>().0 {
        //     self.delete_ui(&mut data);
        // }
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

    fn fixed_update(
        &mut self,
        mut data: StateData<CustomGameData<'a, 'b, CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        if let Some(trans) = self.update_ui_events(&mut data) {
            trans
        } else {
            Trans::None
        }
    }
}

impl Ingame {
    fn handle_keys<'a, 'b>(
        &self,
        world: &World,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        let input = world.read_resource::<InputManager<IngameBindings>>();

        if input.is_down(IngameActionBinding::Quit) {
            Some(Trans::Quit)
        } else if input.is_down(IngameActionBinding::TogglePause) {
            Some(Trans::Push(Box::new(Paused::default())))
        } else {
            None
        }
    }
}

impl<'a, 'b> Menu<CustomGameData<'a, 'b, CustomData>, StateEvent> for Ingame {
    fn event_triggered(
        &mut self,
        _data: &mut StateData<CustomGameData<'a, 'b, CustomData>>,
        _event_name: String,
        _event: UiEvent,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        None
    }

    fn ui_data(&self) -> &UiData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UiData {
        &mut self.ui_data
    }
}
