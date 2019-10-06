pub mod prelude {
    pub use deathframe::systems::prelude::*;

    pub use super::camera::CameraSystem;
    pub use super::control_player::ControlPlayerSystem;
    pub use super::enemy_ai::EnemyAiSystem;
    pub use super::feature::FeatureSystem;
    pub use super::handle_solid_collisions::HandleSolidCollisionsSystem;
    pub use super::kill_enemy::KillEnemySystem;
    pub use super::spike::SpikeSystem;
}

mod system_prelude {
    pub use amethyst::ecs::{World, WorldExt};
    pub use deathframe::systems::system_prelude::*;

    pub use super::helpers::*;
    pub use crate::audio::prelude::*;
    pub use crate::components::prelude::*;
    pub use crate::input::prelude::*;
    pub use crate::solid_tag::SolidTag;
    pub use crate::states::ResetLevel;
}

mod camera;
mod control_player;
mod enemy_ai;
mod feature;
mod handle_solid_collisions;
mod kill_enemy;
mod spike;

mod helpers;
