use std::collections::HashMap;
use std::convert::TryFrom;

use amethyst::ui::UiTransform;

use super::system_prelude::*;

#[derive(Default)]
pub struct MenuSelectionSystem;

impl<'a> System<'a> for MenuSelectionSystem {
    type SystemData =
        (ReadStorage<'a, MenuSelector>, WriteStorage<'a, UiTransform>);

    fn run(&mut self, (menu_selectors, mut transforms): Self::SystemData) {
        let selections_positions: HashMap<MenuSelection, (f32, f32)> =
            (&transforms)
                .join()
                .filter_map(|transform| {
                    if let Ok(selection) =
                        MenuSelection::try_from(transform.id.as_str())
                    {
                        Some((
                            selection,
                            (transform.local_x, transform.local_y),
                        ))
                    } else {
                        None
                    }
                })
                .collect();

        for (selector, selector_transform) in
            (&menu_selectors, &mut transforms).join()
        {
            if let Some(selection_pos) =
                selections_positions.get(&selector.selection)
            {
                selector_transform.local_x = selection_pos.0;
                selector_transform.local_y = selection_pos.1;
            }
        }
    }
}
