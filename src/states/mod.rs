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

    pub use super::helpers::*;
    pub use super::prelude::*;
    pub use crate::audio::prelude::*;
    pub use crate::components::prelude::*;
    pub use crate::helpers::*;
    pub use crate::input::prelude::*;
    pub use crate::level_manager::Level;
    pub use crate::resources::prelude::*;
    pub use crate::savefile_data::prelude::*;
    pub use crate::settings::prelude::*;
    pub use crate::CustomData;
}

mod difficulty_select;
mod ingame;
mod level_load;
mod paused;
mod startup;
mod win;

mod helpers {
    use amethyst::ecs::{World, WorldExt};

    use crate::level_manager::Level;
    use crate::resources::prelude::SavefileDataRes;
    use crate::settings::Settings;

    pub fn is_level_locked(world: &World, level: &Level) -> bool {
        let mut level_locked = false;
        let level_manager_settings =
            &world.read_resource::<Settings>().level_manager;
        let level_settings = level_manager_settings.level(level);
        if level_settings.initially_locked {
            level_locked = true;
            if let Some(savefile_data) =
                &world.read_resource::<SavefileDataRes>().0
            {
                if let Some(unlocked_by_any) =
                    level_settings.unlocked_by_any.as_ref()
                {
                    level_locked =
                        !unlocked_by_any.iter().any(|unlocked_by_level| {
                            if let Some(level_save) =
                                savefile_data.level(unlocked_by_level)
                            {
                                level_save.won
                            } else {
                                false
                            }
                        });
                }
            }
        }
        level_locked
    }
}
