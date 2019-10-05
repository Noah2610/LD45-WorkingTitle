pub mod prelude {
    pub use deathframe::systems::prelude::*;

    pub use super::camera::CameraSystem;
    pub use super::feature::FeatureSystem;
    pub use super::handle_solid_collisions::HandleSolidCollisionsSystem;
    pub use super::move_player::MovePlayerSystem;
}

mod system_prelude {
    pub use deathframe::systems::system_prelude::*;

    pub use super::helpers::*;
    pub use crate::components::prelude::*;
    pub use crate::input::prelude::*;
    pub use crate::solid_tag::SolidTag;
}

mod camera;
mod feature;
mod handle_solid_collisions;
mod move_player;

mod helpers;
