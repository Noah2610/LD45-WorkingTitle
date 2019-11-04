use std::cmp;
use std::convert::TryFrom;

use super::state_prelude::*;

const UI_RON_PATH: &str = "ui/difficulty_select.ron";

#[derive(PartialEq, Eq, Hash)]
enum MenuSelection {
    Easy,
    Normal,
    Hard,
    Absurd,
}

impl MenuSelection {
    fn next(&self) -> Self {
        match self {
            MenuSelection::Easy => MenuSelection::Normal,
            MenuSelection::Normal => MenuSelection::Hard,
            MenuSelection::Hard => MenuSelection::Absurd,
            MenuSelection::Absurd => MenuSelection::Easy,
        }
    }

    fn prev(&self) -> Self {
        match self {
            MenuSelection::Easy => MenuSelection::Absurd,
            MenuSelection::Normal => MenuSelection::Easy,
            MenuSelection::Hard => MenuSelection::Normal,
            MenuSelection::Absurd => MenuSelection::Hard,
        }
    }

    fn index(&self) -> u8 {
        match self {
            MenuSelection::Easy => 0,
            MenuSelection::Normal => 1,
            MenuSelection::Hard => 2,
            MenuSelection::Absurd => 3,
        }
    }

    fn ui_transform_id(&self) -> &str {
        match self {
            MenuSelection::Easy => "button_start_easy",
            MenuSelection::Normal => "button_start_normal",
            MenuSelection::Hard => "button_start_hard",
            MenuSelection::Absurd => "button_start_absurd",
        }
    }
}

impl Default for MenuSelection {
    fn default() -> Self {
        MenuSelection::Easy
    }
}

impl cmp::PartialOrd for MenuSelection {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.index().partial_cmp(&other.index())
    }
}

impl TryFrom<&str> for MenuSelection {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_string().as_str() {
            "button_start_easy" => Ok(MenuSelection::Easy),
            "button_start_normal" => Ok(MenuSelection::Normal),
            "button_start_hard" => Ok(MenuSelection::Hard),
            "button_start_absurd" => Ok(MenuSelection::Absurd),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
pub struct DifficultySelect {
    ui_data:  UiData,
    selected: MenuSelection,
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
        data.data.update(data.world, "difficulty_select").unwrap();

        self.update_selector(data.world);

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
        create_selector(data.world);
    }

    fn handle_keys<'a, 'b>(
        &mut self,
        world: &World,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        let input = world.read_resource::<InputManager<Bindings>>();

        if input.is_down(ActionBinding::Quit) {
            Some(Trans::Quit)
        } else if input.is_down(ActionBinding::MenuNext) {
            self.selected = self.selected.next();
            None
        } else if input.is_down(ActionBinding::MenuPrev) {
            self.selected = self.selected.prev();
            None
        } else if input.is_down(ActionBinding::MenuSelect) {
            match self.selected {
                MenuSelection::Easy => Some(Trans::Push(Box::new(
                    LevelLoad::new("level_easy.json"),
                ))),
                MenuSelection::Normal => Some(Trans::Push(Box::new(
                    LevelLoad::new("level_normal.json"),
                ))),
                MenuSelection::Hard => Some(Trans::Push(Box::new(
                    LevelLoad::new("level_hard.json"),
                ))),
                MenuSelection::Absurd => Some(Trans::Push(Box::new(
                    LevelLoad::new("level_absurd.json"),
                ))),
            }
        } else {
            None
        }
    }

    fn update_selector(&mut self, world: &mut World) {
        use amethyst::ecs::{Join, ReadStorage, WriteStorage};
        use amethyst::ui::UiTransform;
        use std::collections::HashMap;

        let current_selection = &self.selected;

        world.exec(
            |(menu_selectors, mut transforms): (
                ReadStorage<MenuSelector>,
                WriteStorage<UiTransform>,
            )| {
                let selections_positions: HashMap<MenuSelection, (f32, f32)> =
                    (&transforms)
                        .join()
                        .filter_map(|transform| {
                            if let Ok(selection) =
                                MenuSelection::try_from(transform.id.as_str())
                            {
                                Some((
                                    selection,
                                    (transform.pixel_x(), transform.pixel_y()),
                                ))
                            } else {
                                None
                            }
                        })
                        .collect();

                for (_, selector_transform) in
                    (&menu_selectors, &mut transforms).join()
                {
                    if let Some(selection_pos) =
                        selections_positions.get(current_selection)
                    {
                        selector_transform.local_x = selection_pos.0;
                        selector_transform.local_y = selection_pos.1;
                    }
                }
            },
        );
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

fn create_selector(world: &mut World) {
    use amethyst::prelude::Builder;
    use amethyst::ui::{Anchor, UiImage, UiTransform};

    world.register::<MenuSelector>();

    let transform = UiTransform::new(
        "menu_selector".to_string(), // id
        Anchor::BottomMiddle,        // anchor
        Anchor::Middle,              // pivot
        0.0,                         // x
        0.0,                         // y
        1.0,                         // z
        128.0,                       // width
        16.0,                        // height
    );
    let color = UiImage::SolidColor([1.0, 1.0, 1.0, 1.0]);

    world
        .create_entity()
        .with(transform)
        .with(color)
        .with(MenuSelector::default())
        .build();
}

use menu_selector::MenuSelector;

mod menu_selector {
    use amethyst::ecs::{Component, NullStorage};

    #[derive(Default)]
    pub(super) struct MenuSelector;

    impl Component for MenuSelector {
        type Storage = NullStorage<Self>;
    }
}
