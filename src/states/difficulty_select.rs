use super::state_prelude::*;

const UI_RON_PATH: &str = "ui/difficulty_select.ron";

#[derive(Default)]
pub struct DifficultySelect {
    ui_data: UiData,
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent>
    for DifficultySelect
{
    fn on_start(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        let _progress = self.create_ui(&mut data, resource(UI_RON_PATH));
    }

    fn on_stop(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.delete_ui(&mut data);
    }

    fn on_resume(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        let _progress = self.create_ui(&mut data, resource(UI_RON_PATH));
    }

    fn on_pause(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.delete_ui(&mut data);
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(data.world, "difficulty_select").unwrap();
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

impl<'a, 'b> Menu<CustomGameData<'a, 'b, CustomData>, StateEvent>
    for DifficultySelect
{
    fn event_triggered(
        &mut self,
        _data: &mut StateData<CustomGameData<CustomData>>,
        event_name: String,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        match event_name.as_ref() {
            "button_start_easy" => {
                Some(Trans::Push(Box::new(LevelLoad::new("level_easy.json"))))
            }
            "button_start_normal" => {
                Some(Trans::Push(Box::new(LevelLoad::new("level_normal.json"))))
            }
            "button_start_hard" => {
                Some(Trans::Push(Box::new(LevelLoad::new("level_hard.json"))))
            }
            "button_start_absurd" => {
                Some(Trans::Push(Box::new(LevelLoad::new("level_absurd.json"))))
            }
            "button_quit" => Some(Trans::Quit),
            _ => None,
        }
    }

    fn ui_data(&self) -> &UiData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UiData {
        &mut self.ui_data
    }
}
