use amethyst::assets::ProgressCounter;
use amethyst::core::Parent;
use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, WriteStorage};
use amethyst::prelude::Builder;
use amethyst::ui::{Anchor, UiImage, UiText, UiTransform};
use std::convert::TryFrom;

use super::state_prelude::*;

const UI_RON_PATH: &str = "ui/difficulty_select.ron";

#[derive(Default)]
pub struct DifficultySelect {
    ui_data:             UiData,
    ui_loading_progress: Option<ProgressCounter>,
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

        if let Some(progress) = self.ui_loading_progress.as_ref() {
            if progress.is_complete() {
                self.populate_ui(data.world);
                self.ui_loading_progress = None;
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
        self.ui_loading_progress =
            Some(self.create_ui(data, resource(UI_RON_PATH)));
        self.create_selector(data.world);
        // Completion text
        self.maybe_create_completion_text_ui(data);
    }

    fn maybe_create_completion_text_ui(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        if data
            .world
            .read_resource::<SavefileDataRes>()
            .0
            .as_ref()
            .map(SavefileData::has_completed_all_levels_except_very_easy)
            .unwrap_or(false)
        {
            // Has completed all levels, and maybe VeryEasy
            let _progress =
                self.create_ui(data, resource(COMPLETION_TEXT_UI_RON_PATH));
        }
    }

    fn populate_ui(&self, world: &mut World) {
        const VERSION_UI_TRANSFORM_ID: &str = "label_version";
        const PREFIX_BUTTON_UI_TRANSFORM_ID: &str = "button_start_";
        const PREFIX_BEST_TIME_UI_TRANSFORM_ID: &str = "label_best_time_";
        const SUFFIX_BUTTON_TEXT_UI_TRANSFORM_ID: &str = "_btn_txt";

        world.exec(
            |(settings, savefile_data_res, ui_transforms, mut ui_texts): (
                ReadExpect<Settings>,
                Read<SavefileDataRes>,
                ReadStorage<UiTransform>,
                WriteStorage<UiText>,
            )| {
                let level_manager_settings = &settings.level_manager;

                for (transform, text) in (&ui_transforms, &mut ui_texts).join()
                {
                    let transform_id = transform.id.as_str();

                    // Set version number
                    if transform_id == VERSION_UI_TRANSFORM_ID
                        && text.text.as_str() != crate::meta::VERSION
                    {
                        text.text = format!("v{}", crate::meta::VERSION);
                    }
                    // Set best time
                    if let Some(savefile_data) = savefile_data_res.0.as_ref() {
                        if transform_id
                            .starts_with(PREFIX_BEST_TIME_UI_TRANSFORM_ID)
                        {
                            if let Some(best_time) = Level::try_from(
                                transform_id
                                    .replace(
                                        PREFIX_BEST_TIME_UI_TRANSFORM_ID,
                                        "",
                                    )
                                    .as_str(),
                            )
                            .ok()
                            .and_then(|level| savefile_data.level(&level))
                            .and_then(|level_data| {
                                level_data.best_time.as_ref()
                            }) {
                                text.text = best_time.to_string();
                            }
                        }
                    }
                    // Set locked text color
                    if transform_id.starts_with(PREFIX_BUTTON_UI_TRANSFORM_ID)
                        && transform_id
                            .ends_with(SUFFIX_BUTTON_TEXT_UI_TRANSFORM_ID)
                    {
                        if let Some(locked_level) = Level::try_from(
                            transform_id
                                .replace(PREFIX_BUTTON_UI_TRANSFORM_ID, "")
                                .replace(SUFFIX_BUTTON_TEXT_UI_TRANSFORM_ID, "")
                                .as_str(),
                        )
                        .ok()
                        .filter(|level| {
                            is_level_locked(
                                level,
                                level_manager_settings,
                                &savefile_data_res.0,
                            )
                        }) {
                            let level_settings =
                                level_manager_settings.level(&locked_level);
                            text.color =
                                level_settings.locked_text_color.unwrap_or(
                                    level_manager_settings
                                        .default_locked_text_color,
                                );
                        }
                    }
                }
            },
        );
    }

    fn handle_keys<'a, 'b>(
        &mut self,
        world: &World,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        let input = world.read_resource::<InputManager<MenuBindings>>();

        if input.is_down(MenuActionBinding::MenuPrev) {
            (&mut world.write_storage::<MenuSelector>())
                .join()
                .next()
                .map(MenuSelector::prev);
        } else if input.is_down(MenuActionBinding::MenuNext) {
            (&mut world.write_storage::<MenuSelector>())
                .join()
                .next()
                .map(MenuSelector::next);
        } else if input.is_down(MenuActionBinding::MenuSelect) {
            if let Some(selector) =
                (&mut world.write_storage::<MenuSelector>()).join().next()
            {
                return Some(start_level(world, selector.selection.level()));
            }
        } else if input.is_down(MenuActionBinding::MenuDeleteSave) {
            if let Some(selector) =
                (&mut world.write_storage::<MenuSelector>()).join().next()
            {
                return Some(start_level_with_delete_save(
                    world,
                    selector.selection.level(),
                ));
            }
        }

        if input.is_down(MenuActionBinding::Quit) {
            Some(Trans::Quit)
        } else {
            None
        }
    }

    fn create_selector(&mut self, world: &mut World) {
        let parent_transform = UiTransform::new(
            "container_menu_selector".to_string(), // id
            Anchor::Middle,                        // anchor
            Anchor::Middle,                        // pivot
            0.05,                                  // x
            0.008,                                 // y
            0.0,                                   // z
            1.0,                                   // width
            1.0,                                   // height
        )
        .into_percent()
        .into_transparent();

        let selector_transform = UiTransform::new(
            "menu_selector".to_string(), // id
            Anchor::MiddleLeft,          // anchor
            Anchor::MiddleLeft,          // pivot
            0.0,                         // x
            0.0,                         // y
            1.1,                         // z
            0.015,                       // width
            0.015,                       // height
        )
        .into_percent()
        .into_transparent();
        let color = UiImage::SolidColor([1.0, 0.0, 0.0, 1.0]);

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
        data: &mut StateData<CustomGameData<CustomData>>,
        event_name: String,
        event: UiEvent,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        match (event_name.as_ref(), event.event_type) {
            ("button_start_very_easy", UiEventType::ClickStop) => {
                Some(start_level(data.world, Level::VeryEasy))
            }
            ("button_start_easy", UiEventType::ClickStop) => {
                Some(start_level(data.world, Level::Easy))
            }
            ("button_start_normal", UiEventType::ClickStop) => {
                Some(start_level(data.world, Level::Normal))
            }
            ("button_start_hard", UiEventType::ClickStop) => {
                Some(start_level(data.world, Level::Hard))
            }
            ("button_start_absurd", UiEventType::ClickStop) => {
                Some(start_level(data.world, Level::Absurd))
            }
            ("button_quit", UiEventType::ClickStop) => Some(Trans::Quit),

            (name, UiEventType::HoverStart) => {
                if let Ok(selection) = MenuSelection::try_from(name) {
                    (&mut data.world.write_storage::<MenuSelector>())
                        .join()
                        .next()
                        .map(|selector| selector.set(selection));
                }
                None
            }
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

fn start_level<'a, 'b>(
    world: &World,
    level: Level,
) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
    let settings = &world.read_resource::<Settings>().level_manager;
    let savefile_data = &world.read_resource::<SavefileDataRes>().0;
    if is_level_locked(&level, settings, savefile_data) {
        Trans::None
    } else {
        Trans::Push(Box::new(LevelLoad::new(level)))
    }
}

fn start_level_with_delete_save<'a, 'b>(
    world: &World,
    level: Level,
) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
    let settings = &world.read_resource::<Settings>().level_manager;
    let savefile_data = &world.read_resource::<SavefileDataRes>().0;
    if is_level_locked(&level, settings, savefile_data) {
        Trans::None
    } else {
        Trans::Push(Box::new(LevelLoad::with_delete_save(level)))
    }
}
