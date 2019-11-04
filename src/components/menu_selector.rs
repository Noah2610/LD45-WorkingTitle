use std::convert::TryFrom;

use super::component_prelude::*;

#[derive(PartialEq, Eq, Hash)]
pub enum MenuSelection {
    Easy,
    Normal,
    Hard,
    Absurd,
}

impl MenuSelection {
    #[rustfmt::skip]
    pub fn next(&self) -> Self {
        match self {
            MenuSelection::Easy   => MenuSelection::Normal,
            MenuSelection::Normal => MenuSelection::Hard,
            MenuSelection::Hard   => MenuSelection::Absurd,
            MenuSelection::Absurd => MenuSelection::Easy,
        }
    }

    #[rustfmt::skip]
    pub fn prev(&self) -> Self {
        match self {
            MenuSelection::Easy   => MenuSelection::Absurd,
            MenuSelection::Normal => MenuSelection::Easy,
            MenuSelection::Hard   => MenuSelection::Normal,
            MenuSelection::Absurd => MenuSelection::Hard,
        }
    }

    #[rustfmt::skip]
    pub fn level_name(&self) -> &str {
        match self {
            MenuSelection::Easy   => "level_easy.json",
            MenuSelection::Normal => "level_normal.json",
            MenuSelection::Hard   => "level_hard.json",
            MenuSelection::Absurd => "level_absurd.json",
        }
    }
}

impl Default for MenuSelection {
    fn default() -> Self {
        MenuSelection::Easy
    }
}

impl TryFrom<&str> for MenuSelection {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_string().as_str() {
            "button_start_easy" => Ok(MenuSelection::Easy),
            "button_start_normal" => Ok(MenuSelection::Normal),
            "button_start_hard" => Ok(MenuSelection::Hard),
            "button_start_absurd" => Ok(MenuSelection::Absurd),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
pub struct MenuSelector {
    pub selection: MenuSelection,
}

impl MenuSelector {
    pub fn next(&mut self) {
        self.selection = self.selection.next();
    }

    pub fn prev(&mut self) {
        self.selection = self.selection.prev();
    }
}

impl Component for MenuSelector {
    type Storage = HashMapStorage<Self>;
}
