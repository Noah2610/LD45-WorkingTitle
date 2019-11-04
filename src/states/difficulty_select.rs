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
        self.create_uis(&mut data);
    }

    fn on_stop(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.delete_ui(&mut data);
    }

    fn on_resume(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.create_uis(&mut data);
    }

    fn on_pause(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.delete_ui(&mut data);
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(data.world, "menu").unwrap();

        if let Some(trans) = self.handle_keys(data.world) {
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

    fn shadow_update(&mut self, data: StateData<CustomGameData<CustomData>>) {
        // Stop audio
        if data.world.read_resource::<StopAudio>().0 {
            stop_audio(data.world);
            data.world.write_resource::<StopAudio>().0 = false;
        }
    }
}

impl DifficultySelect {
    fn create_uis(&mut self, data: &mut StateData<CustomGameData<CustomData>>) {
        let _progress = self.create_ui(data, resource(QUIT_UI_RON_PATH));
        let _progress = self.create_ui(data, resource(UI_RON_PATH));
        self.create_selector(data.world);
    }

    fn handle_keys<'a, 'b>(
        &mut self,
        world: &World,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        use amethyst::ecs::Join;

        let input = world.read_resource::<InputManager<MenuBindings>>();

        if input.is_down(MenuActionBinding::MenuNext) {
            (&mut world.write_storage::<MenuSelector>())
                .join()
                .next()
                .map(MenuSelector::next);
        } else if input.is_down(MenuActionBinding::MenuPrev) {
            (&mut world.write_storage::<MenuSelector>())
                .join()
                .next()
                .map(MenuSelector::prev);
        }

        if input.is_down(MenuActionBinding::Quit) {
            Some(Trans::Quit)
        } else if input.is_down(MenuActionBinding::MenuSelect) {
            if let Some(selector) =
                (&world.read_storage::<MenuSelector>()).join().next()
            {
                Some(Trans::Push(Box::new(LevelLoad::new(
                    selector.selection.level_name(),
                ))))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn create_selector(&mut self, world: &mut World) {
        use amethyst::core::Parent;
        use amethyst::prelude::Builder;
        use amethyst::ui::{Anchor, UiImage, UiTransform};

        // world.register::<MenuSelector>();

        let parent_transform = UiTransform::new(
            "container_menu_selector".to_string(), // id
            Anchor::Middle,                        // anchor
            Anchor::Middle,                        // pivot
            0.0,                                   // x
            0.0,                                   // y
            0.0,                                   // z
            1.0,                                   // width
            1.0,                                   // height
        )
        .into_percent();

        let selector_transform = UiTransform::new(
            "menu_selector".to_string(), // id
            Anchor::MiddleLeft,          // anchor
            Anchor::MiddleLeft,          // pivot
            0.0,                         // x
            0.1,                         // y
            1.0,                         // z
            0.3,                         // width
            0.1,                         // height
        )
        .into_percent();
        let color = UiImage::SolidColor([1.0, 1.0, 1.0, 1.0]);

        let parent = world.create_entity().with(parent_transform).build();

        let selector = world
            .create_entity()
            .with(Parent { entity: parent })
            .with(selector_transform)
            .with(color)
            .with(MenuSelector::default())
            .build();

        self.push_ui_entity(parent);
        self.push_ui_entity(selector);
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
