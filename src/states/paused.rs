use super::state_prelude::*;

const UI_RON_PATH: &str = "ui/paused.ron";

#[derive(Default)]
pub struct Paused {
    ui_data: UiData,
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Paused {
    fn on_start(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        let _progress = self.create_ui(&mut data, resource(QUIT_UI_RON_PATH));
        // let _progress = self.create_ui(&mut data, resource(BACK_UI_RON_PATH));
        let _progress = self.create_ui(&mut data, resource(UI_RON_PATH));
        data.world.write_resource::<Music>().pause();
        set_decreased_volume(&mut data.world);
    }

    fn on_stop(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.delete_ui(&mut data);
        data.world.write_resource::<Music>().resume();
        set_normal_volume(&mut data.world);
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData<'a, 'b, CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(data.world, "paused").unwrap();

        if let Some(trans) = self.handle_keys(&data.world) {
            return trans;
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

impl Paused {
    fn handle_keys<'a, 'b>(
        &self,
        world: &World,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        let input = world.read_resource::<InputManager<IngameBindings>>();

        if input.is_down(IngameActionBinding::Quit) {
            Some(Trans::Quit)
        } else if input.is_down(IngameActionBinding::TogglePause) {
            Some(Trans::Pop)
        } else if input.is_down(IngameActionBinding::ToMainMenu) {
            world.write_resource::<ToMainMenu>().0 = true;
            Some(Trans::Pop)
        } else {
            None
        }
    }
}

impl<'a, 'b> Menu<CustomGameData<'a, 'b, CustomData>, StateEvent> for Paused {
    fn event_triggered(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
        event_name: String,
        event: UiEvent,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        if let UiEventType::ClickStop = event.event_type {
            match event_name.as_ref() {
                "button_back" => Some(Trans::Pop),
                "button_quit" => {
                    data.world.write_resource::<ToMainMenu>().0 = true;
                    Some(Trans::Pop)
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn ui_data(&self) -> &UiData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UiData {
        &mut self.ui_data
    }
}
