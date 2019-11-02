use super::state_prelude::*;

const RON_PATH: &str = "ui/win.ron";

#[derive(Default)]
pub struct Win {
    ui_data: UiData,
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Win {
    fn on_start(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        let _progress = self.create_ui(&mut data, resource(RON_PATH));
    }

    fn on_stop(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.delete_ui(&mut data);
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(data.world, "win").unwrap();
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

impl<'a, 'b> Menu<CustomGameData<'a, 'b, CustomData>, StateEvent> for Win {
    fn ui_data(&self) -> &UiData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UiData {
        &mut self.ui_data
    }

    fn event_triggered(
        &mut self,
        _data: &mut StateData<CustomGameData<CustomData>>,
        _event_name: String,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        None
    }
}
