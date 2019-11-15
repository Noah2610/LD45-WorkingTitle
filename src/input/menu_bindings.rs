use amethyst::input::BindingTypes;

#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct MenuBindings;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MenuAxisBinding {
    None,
}

impl Default for MenuAxisBinding {
    fn default() -> Self {
        MenuAxisBinding::None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MenuActionBinding {
    None,
    Quit,
    MenuNext,
    MenuPrev,
    MenuSelect,
    MenuDeleteSave,
}

impl Default for MenuActionBinding {
    fn default() -> Self {
        MenuActionBinding::None
    }
}

impl BindingTypes for MenuBindings {
    type Axis = MenuAxisBinding;
    type Action = MenuActionBinding;
}
