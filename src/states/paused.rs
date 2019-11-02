use super::state_prelude::*;

#[derive(Default)]
pub struct Paused {
    ui_data: UiData,
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Paused {
    fn on_start(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        // let _progress = self.create_ui(&mut data, resource(RON_PATH));
        data.world.write_resource::<Music>().pause();
        set_decreased_volume(&mut data.world);
    }

    fn on_stop(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        data.world.write_resource::<Music>().resume();
        set_normal_volume(&mut data.world);
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(data.world, "paused").unwrap();

        if let Some(trans) = self.handle_keys(&data.world) {
            return trans;
        }

        Trans::None
    }
}

impl Paused {
    fn handle_keys<'a, 'b>(
        &mut self,
        world: &World,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        let input = world.read_resource::<InputManager<Bindings>>();

        if input.is_down(ActionBinding::Quit) {
            Some(Trans::Quit)
        } else if input.is_down(ActionBinding::TogglePause) {
            Some(Trans::Pop)
        } else {
            None
        }
    }
}
