use std::collections::HashMap;
use std::convert::TryFrom;

use amethyst::ui::{UiText, UiTransform};

use super::system_prelude::*;

const DIFFICULTY_DESCRIPTION_TRANSFORM_ID: &str =
    "label_difficulty_description";
const PREFIX_SELECTION_TRANSFORM_ID: &str = "selection_";

#[derive(Default)]
pub struct MenuSelectionSystem;

impl<'a> System<'a> for MenuSelectionSystem {
    type SystemData = (
        Read<'a, Time>,
        ReadExpect<'a, Settings>,
        Read<'a, SavefileDataRes>,
        ReadStorage<'a, MenuSelector>,
        WriteStorage<'a, UiTransform>,
        WriteStorage<'a, UiText>,
    );

    fn run(
        &mut self,
        (
            time,
            settings,
            savefile_data,
            menu_selectors,
            mut transforms,
            mut texts,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();
        let (selector_animation_speed, selector_animation_deadzone) = {
            let anim_speed = &settings.misc.menu_selector_animation_speed;
            (
                (anim_speed.0 * dt, anim_speed.1 * dt),
                settings.misc.menu_selector_animation_deadzone.into(),
            )
        };
        let level_manager_settings = &settings.level_manager;

        let selections_positions: HashMap<MenuSelection, (f32, f32)> =
            (&transforms)
                .join()
                .filter_map(|transform| {
                    let transform_id = transform.id.as_str();
                    if transform_id.starts_with(PREFIX_SELECTION_TRANSFORM_ID) {
                        if let Ok(selection) =
                            MenuSelection::try_from(transform_id)
                        {
                            Some((
                                selection,
                                (transform.local_x, transform.local_y),
                            ))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

        let mut selected_level_opt = None;

        for (selector, selector_transform) in
            (&menu_selectors, &mut transforms).join()
        {
            selected_level_opt = Some(selector.selection.0.clone());

            if let Some(selection_pos) =
                selections_positions.get(&selector.selection)
            {
                move_selector_towards(
                    selector_transform,
                    selection_pos,
                    selector_animation_speed,
                    selector_animation_deadzone,
                );
            }
        }

        if let Some(selected_level) = selected_level_opt {
            let level_settings = level_manager_settings.level(&selected_level);

            let (level_description, level_description_color) =
                if is_level_locked(
                    &selected_level,
                    level_manager_settings,
                    &savefile_data.0,
                ) {
                    (
                        level_settings
                            .locked_description
                            .as_ref()
                            .map(String::as_str)
                            .unwrap_or(
                                level_manager_settings
                                    .default_locked_description
                                    .as_str(),
                            ),
                        level_manager_settings.locked_description_text_color,
                    )
                } else {
                    (
                        level_settings.description.as_str(),
                        level_manager_settings.description_text_color,
                    )
                };

            // Update level description
            (&transforms, &mut texts)
                .join()
                .find(|(transform, _)| {
                    transform.id == DIFFICULTY_DESCRIPTION_TRANSFORM_ID
                })
                .map(|(_, description)| {
                    if description.text.as_str() != level_description {
                        description.text = level_description.to_string();
                        description.color = level_description_color;
                    }
                });
        }
    }
}

fn move_selector_towards(
    selector_transform: &mut UiTransform,
    target_pos: &(f32, f32),
    speed: (f32, f32),
    deadzone: (f32, f32),
) {
    let dist_x = target_pos.0 - selector_transform.local_x;
    let dist_y = target_pos.1 - selector_transform.local_y;
    if dist_x.abs() <= deadzone.0 {
        selector_transform.local_x = target_pos.0;
    } else {
        selector_transform.local_x += speed.0 * dist_x;
    }
    if dist_y.abs() <= deadzone.1 {
        selector_transform.local_y = target_pos.1;
    } else {
        selector_transform.local_y += speed.1 * dist_y;
    }
}
