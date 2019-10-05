pub mod prelude {
    pub use deathframe::systems::prelude::*;

    // pub use super::camera::CameraSystem;
    pub use super::move_player::MovePlayerSystem;
}

mod system_prelude {
    pub use deathframe::systems::system_prelude::*;

    pub use crate::components::prelude::*;
    pub use crate::input::prelude::*;
}

mod camera;
mod move_player;
