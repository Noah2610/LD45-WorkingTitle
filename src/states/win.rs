use amethyst::assets::ProgressCounter;

use super::state_prelude::*;

const RON_PATH: &str = "ui/win.ron";

#[derive(Default)]
pub struct Win {
    level:               Level,
    ui_data:             UiData,
    ui_loading_progress: Option<ProgressCounter>,
}

impl Win {
    pub fn new(level: Level) -> Self {
        Self {
            level,
            ui_data: Default::default(),
            ui_loading_progress: Default::default(),
        }
    }

    fn set_label(&self, world: &mut World) {
        use amethyst::ecs::{Join, ReadStorage, WriteStorage};
        use amethyst::ui::{UiText, UiTransform};

        const WIN_LABEL_UI_TRANSFORM_ID: &str = "label_win";

        let new_text = world
            .read_resource::<Settings>()
            .level_manager
            .level(&self.level)
            .win_text
            .clone();

        world.exec(
            |(ui_transforms, mut ui_texts): (
                ReadStorage<UiTransform>,
                WriteStorage<UiText>,
            )| {
                if let Some(text) = (&ui_transforms, &mut ui_texts)
                    .join()
                    .filter_map(|(transform, text)| {
                        if transform.id.as_str() == WIN_LABEL_UI_TRANSFORM_ID {
                            Some(text)
                        } else {
                            None
                        }
                    })
                    .next()
                {
                    if text.text.as_str() != new_text.as_str() {
                        text.text = new_text.to_string();
                    }
                }
            },
        );
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Win {
    fn on_start(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        let _progress = self.create_ui(&mut data, resource(QUIT_UI_RON_PATH));
        self.ui_loading_progress =
            Some(self.create_ui(&mut data, resource(RON_PATH)));
    }

    fn on_stop(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.delete_ui(&mut data);
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(data.world, "win").unwrap();

        if let Some(trans) = self.handle_keys(data.world) {
            return trans;
        }

        if let Some(progress) = self.ui_loading_progress.as_ref() {
            if progress.is_complete() {
                self.set_label(data.world);
            }
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

impl Win {
    fn handle_keys<'a, 'b>(
        &self,
        world: &World,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        let input = world.read_resource::<InputManager<IngameBindings>>();

        if input.is_down(IngameActionBinding::Quit) {
            Some(Trans::Quit)
        } else if input.is_down(IngameActionBinding::ToMainMenu) {
            world.write_resource::<ToMainMenu>().0 = true;
            Some(Trans::Pop)
        } else {
            None
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
        data: &mut StateData<CustomGameData<CustomData>>,
        event_name: String,
        event: UiEvent,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        if let UiEventType::ClickStop = event.event_type {
            match event_name.as_ref() {
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
}
