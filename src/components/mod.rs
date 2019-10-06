pub mod prelude {
    pub use amethyst::renderer::{Camera as AmethystCamera, SpriteRender};
    pub use deathframe::components::prelude::*;
    pub use deathframe::geo::prelude::*;

    pub use super::can_jump::CanJump;
    pub use super::can_run::CanRun;
    pub use super::can_wall_jump::CanWallJump;
    pub use super::enemy::{Enemy, EnemyType};
    pub use super::enemy_ai::{enemy_ai_data, EnemyAi};
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
mod can_run;
mod can_wall_jump;
mod enemy;
mod enemy_ai;
mod feature;
mod has_animated_sprite;
mod has_single_sprite;
mod player;
mod spike;
