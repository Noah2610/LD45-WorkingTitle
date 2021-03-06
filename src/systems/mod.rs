pub mod prelude {
    pub use amethyst::utils::ortho_camera::CameraOrthoSystem;
    pub use deathframe::systems::prelude::*;

    pub use super::background::BackgroundSystem;
    pub use super::checkpoint::CheckpointSystem;
    pub use super::control_player::ControlPlayerSystem;
    pub use super::death_floor::DeathFloorSystem;
    pub use super::debug::DebugSystem;
    pub use super::dynamic_animation::DynamicAnimationSystem;
    pub use super::enemy_ai::EnemyAiSystem;
    pub use super::feature::FeatureSystem;
    pub use super::follow::FollowSystem;
    pub use super::goal::GoalSystem;
    pub use super::handle_solid_collisions::HandleSolidCollisionsSystem;
    pub use super::kill_enemy::KillEnemySystem;
    pub use super::loading::LoadingSystem;
    pub use super::menu_selection::MenuSelectionSystem;
    pub use super::player_run::PlayerRunSystem;
    pub use super::spike::SpikeSystem;
    pub use super::timer::TimerSystem;
}

mod system_prelude {
    pub use amethyst::ecs::{World, WorldExt};
    pub use amethyst::ui::{UiText, UiTransform};
    pub use deathframe::systems::system_prelude::*;

    pub use super::helpers::*;
    pub use crate::audio::prelude::*;
    pub use crate::components::prelude::*;
    pub use crate::input::prelude::*;
    pub use crate::resources::prelude::*;
    pub use crate::settings::prelude::*;
    pub use crate::solid_tag::SolidTag;
}

pub mod helpers;

mod background;
mod checkpoint;
mod control_player;
mod death_floor;
mod debug;
mod dynamic_animation;
mod enemy_ai;
mod feature;
mod follow;
mod goal;
mod handle_solid_collisions;
mod kill_enemy;
mod loading;
mod menu_selection;
mod player_run;
mod spike;
mod timer;
