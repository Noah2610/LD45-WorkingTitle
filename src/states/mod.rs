pub mod prelude {
    pub use super::Ingame;
    pub use super::Paused;
    pub use super::Startup;
    pub use super::Win;
}

pub mod state_prelude {
    pub use amethyst::ecs::{World, WorldExt};
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

mod ingame;
mod paused;
mod startup;
mod win;

pub use ingame::Ingame;
pub use paused::Paused;
pub use startup::Startup;
pub use win::Win;
