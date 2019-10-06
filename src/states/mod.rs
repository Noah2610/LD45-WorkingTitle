pub mod prelude {
    pub use super::Ingame;
    pub use super::Startup;
}

pub mod state_prelude {
    pub use amethyst::ecs::{World, WorldExt};
    pub use amethyst::{State, StateData, StateEvent, Trans};

    pub use deathframe::custom_game_data::CustomGameData;

    pub use super::prelude::*;
    pub use super::ResetLevel;
    pub use crate::audio::prelude::*;
    pub use crate::components::prelude::*;
    pub use crate::helpers::*;
    pub use crate::settings::prelude::*;
    pub use crate::CustomData;
}

mod ingame;
mod startup;

pub use ingame::Ingame;
pub use startup::Startup;

#[derive(Default)]
pub struct ResetLevel(pub bool);
