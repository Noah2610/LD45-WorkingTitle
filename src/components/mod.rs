pub mod prelude {
    pub use amethyst::renderer::{Camera as AmethystCamera, SpriteRender};
    pub use deathframe::components::prelude::*;
    pub use deathframe::geo::prelude::*;

    pub use super::can_jump::CanJump;
    pub use super::enemy::{Enemy, EnemyType};
    pub use super::feature::{Feature, FeatureType};
    pub use super::has_animated_sprite::HasAnimatedSprite;
    pub use super::has_single_sprite::HasSingleSprite;
    pub use super::player::{Player, PlayerJumpData};
    pub use super::spike::Spike;
}

pub mod component_prelude {
    pub use deathframe::components::component_prelude::*;

    pub use super::prelude::*;
    pub use crate::settings::prelude::*;
    pub use crate::states::ResetLevel;
}

mod can_jump;
mod enemy;
mod feature;
mod has_animated_sprite;
mod has_single_sprite;
mod player;
mod spike;
