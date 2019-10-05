pub mod prelude {
    pub use deathframe::components::prelude::*;
    pub use deathframe::geo::prelude::*;
}

pub mod component_prelude {
    pub use deathframe::components::component_prelude::*;

    pub use super::prelude::*;
}
