pub mod prelude {
    pub use deathframe::systems::prelude::*;

    pub use super::background::BackgroundSystem;
    pub use super::control_player::ControlPlayerSystem;
    pub use super::death_floor::DeathFloorSystem;
    pub use super::enemy_ai::EnemyAiSystem;
    pub use super::feature::FeatureSystem;
    pub use super::follow::FollowSystem;
    pub use super::handle_solid_collisions::HandleSolidCollisionsSystem;
    pub use super::kill_enemy::KillEnemySystem;
    pub use super::player_run::PlayerRunSystem;
    pub use super::spike::SpikeSystem;
}

mod system_prelude {
    pub use amethyst::ecs::{World, WorldExt};
    pub use deathframe::systems::system_prelude::*;

    pub use super::helpers::*;
    pub use crate::audio::prelude::*;
    pub use crate::components::prelude::*;
    pub use crate::input::prelude::*;
    pub use crate::settings::prelude::*;
    pub use crate::solid_tag::SolidTag;
    pub use crate::states::ResetLevel;
}

mod background;
mod control_player;
mod death_floor;
mod enemy_ai;
mod feature;
mod follow;
mod handle_solid_collisions;
mod kill_enemy;
mod player_run;
mod spike;

mod helpers;
