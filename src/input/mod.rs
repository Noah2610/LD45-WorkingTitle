mod ingame_bindings;
mod menu_bindings;

pub mod prelude {
    pub use super::ingame_bindings::{
        IngameActionBinding,
        IngameAxisBinding,
        IngameBindings,
    };
    pub use super::menu_bindings::{
        MenuActionBinding,
        MenuAxisBinding,
        MenuBindings,
    };
}
pub use ingame_bindings::IngameBindings;
pub use menu_bindings::MenuBindings;

use amethyst::input::InputBundle;

use crate::helpers::resource;

pub fn ingame_input_bundle() -> InputBundle<ingame_bindings::IngameBindings> {
    InputBundle::<ingame_bindings::IngameBindings>::new()
        .with_bindings_from_file(resource("config/ingame_bindings.ron"))
        .unwrap()
}

pub fn menu_input_bundle() -> InputBundle<menu_bindings::MenuBindings> {
    InputBundle::<menu_bindings::MenuBindings>::new()
        .with_bindings_from_file(resource("config/menu_bindings.ron"))
        .unwrap()
}
