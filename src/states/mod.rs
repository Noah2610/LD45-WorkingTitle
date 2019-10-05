pub mod prelude {
    pub use super::Startup;
}

pub mod state_prelude {
    pub use amethyst::ecs::{World, WorldExt};
    pub use amethyst::{State, StateData, StateEvent, Trans};

    pub use deathframe::custom_game_data::CustomGameData;

    pub use super::prelude::*;
    pub use crate::components::prelude::*;
    pub use crate::helpers::*;
    pub use crate::settings::prelude::*;
    pub use crate::CustomData;
}

mod startup;

pub use startup::Startup;
