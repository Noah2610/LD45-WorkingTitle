use amethyst::input::BindingTypes;

#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct IngameBindings;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IngameAxisBinding {
    None,
    PlayerX,
    PlayerY,
}

impl Default for IngameAxisBinding {
    fn default() -> Self {
        IngameAxisBinding::None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IngameActionBinding {
    None,
    Quit,
    PlayerJump,
    PlayerRun,
    TogglePause,
    ToMainMenu,
}

impl Default for IngameActionBinding {
    fn default() -> Self {
        IngameActionBinding::None
    }
}

impl BindingTypes for IngameBindings {
    type Axis = IngameAxisBinding;
    type Action = IngameActionBinding;
}
