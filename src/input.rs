pub mod prelude {
    pub use super::ActionBinding;
    pub use super::AxisBinding;
    pub use super::Bindings;
}

use amethyst::input::{BindingTypes, InputBundle};

#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct Bindings;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AxisBinding {
    None,
    PlayerX,
    PlayerY,
}

impl Default for AxisBinding {
    fn default() -> Self {
        AxisBinding::None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionBinding {
    None,
    PlayerJump,
}

impl Default for ActionBinding {
    fn default() -> Self {
        ActionBinding::None
    }
}

impl BindingTypes for Bindings {
    type Axis = AxisBinding;
    type Action = ActionBinding;
}

pub fn input_bundle() -> InputBundle<Bindings> {
    InputBundle::<Bindings>::new()
}
