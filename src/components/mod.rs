pub mod prelude {
    pub use amethyst::renderer::{Camera as AmethystCamera, SpriteRender};
    pub use deathframe::components::prelude::*;
    pub use deathframe::geo::prelude::*;

    pub use super::can_jump::CanJump;
    pub use super::feature::{Feature, FeatureType};
    pub use super::player::Player;
}

pub mod component_prelude {
    pub use deathframe::components::component_prelude::*;

    pub use super::prelude::*;
    pub use crate::settings::prelude::*;
}

mod can_jump;
mod feature;
mod player;
