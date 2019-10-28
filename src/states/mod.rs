pub mod prelude {
    pub use super::Ingame;
    pub use super::Startup;
    pub use super::Win;
}

pub mod state_prelude {
    pub use amethyst::ecs::{World, WorldExt};
    pub use amethyst::{State, StateData, StateEvent, Trans};

    pub use deathframe::custom_game_data::CustomGameData;
    pub use deathframe::menu::prelude::*;

    pub use super::prelude::*;
    pub use crate::audio::prelude::*;
    pub use crate::components::prelude::*;
    pub use crate::helpers::*;
    pub use crate::resources::prelude::*;
    pub use crate::settings::prelude::*;
    pub use crate::CustomData;
}

mod ingame;
mod startup;
mod win;

pub use ingame::Ingame;
pub use startup::Startup;
pub use win::Win;
