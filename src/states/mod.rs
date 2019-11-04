pub mod prelude {
    pub use super::difficulty_select::DifficultySelect;
    pub use super::ingame::Ingame;
    pub use super::level_load::LevelLoad;
    pub use super::paused::Paused;
    pub use super::startup::Startup;
    pub use super::win::Win;
}

pub mod state_prelude {
    pub const QUIT_UI_RON_PATH: &str = "ui/_quit.ron";
    pub const BACK_UI_RON_PATH: &str = "ui/_back.ron";

    pub use amethyst::ecs::{World, WorldExt};
    pub use amethyst::ui::{UiEvent, UiEventType};
    pub use amethyst::{State, StateData, StateEvent, Trans};

    pub use deathframe::custom_game_data::CustomGameData;
    pub use deathframe::input_manager::InputManager;
    pub use deathframe::menu::prelude::*;

    pub use super::prelude::*;
    pub use crate::audio::prelude::*;
    pub use crate::components::prelude::*;
    pub use crate::helpers::*;
    pub use crate::input::prelude::*;
    pub use crate::resources::prelude::*;
    pub use crate::settings::prelude::*;
    pub use crate::CustomData;
}

mod difficulty_select;
mod ingame;
mod level_load;
mod paused;
mod startup;
mod win;
