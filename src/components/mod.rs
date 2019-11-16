pub mod prelude {
    pub use amethyst::core::Hidden;
    pub use amethyst::renderer::{Camera as AmethystCamera, SpriteRender};
    pub use deathframe::components::prelude::*;
    pub use deathframe::geo::prelude::*;

    pub use super::background::Background;
    pub use super::can_dash::CanDash;
    pub use super::can_hover::CanHover;
    pub use super::can_jump::CanJump;
    pub use super::can_run::CanRun;
    pub use super::can_wall_jump::CanWallJump;
    pub use super::checkpoint::{Checkpoint, CheckpointId};
    pub use super::dynamic_animation::{
        DynamicAnimation,
        DynamicAnimationTrigger,
    };
    pub use super::enemy::{Enemy, EnemyType};
    pub use super::enemy_ai::{enemy_ai_data, EnemyAi};
    pub use super::feature::{Feature, FeatureType, ForceApplyFeature};
    pub use super::follow::{FollowTag, Followed, Follower};
    pub use super::goal::Goal;
    pub use super::has_animated_sprite::HasAnimatedSprite;
    pub use super::has_single_sprite::HasSingleSprite;
    pub use super::indicator::Indicator;
    pub use super::loader::Loader;
    pub use super::menu_selector::{Level, MenuSelection, MenuSelector};
    pub use super::player::Player;
    pub use super::spike::Spike;
    pub use super::tile::Tile;
}

pub mod component_prelude {
    pub use amethyst::ui::Anchor as AmethystAnchor;
    pub use deathframe::components::component_prelude::*;

    pub use super::prelude::*;
    pub use crate::resources::prelude::*;
    pub use crate::settings::prelude::*;
}

mod background;
mod can_dash;
mod can_hover;
mod can_jump;
mod can_run;
mod can_wall_jump;
mod checkpoint;
mod dynamic_animation;
mod enemy;
mod enemy_ai;
mod feature;
mod follow;
mod goal;
mod has_animated_sprite;
mod has_single_sprite;
mod indicator;
mod loader;
mod menu_selector;
mod player;
mod spike;
mod tile;
