use std::convert::TryFrom;

use super::component_prelude::*;
use crate::level_manager::Level;

#[derive(Default, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct MenuSelection(pub Level);

impl MenuSelection {
    #[rustfmt::skip]
    pub fn next(&mut self) {
        self.0 = match self.0 {
            Level::VeryEasy => Level::Easy,
            Level::Easy     => Level::Normal,
            Level::Normal   => Level::Hard,
            Level::Hard     => Level::Absurd,
            Level::Absurd   => Level::VeryEasy,
        };
    }

    #[rustfmt::skip]
    pub fn prev(&mut self) {
        self.0 = match self.0 {
            Level::VeryEasy => Level::Absurd,
            Level::Easy     => Level::VeryEasy,
            Level::Normal   => Level::Easy,
            Level::Hard     => Level::Normal,
            Level::Absurd   => Level::Hard,
        };
    }

    pub fn level(&self) -> Level {
        self.0.clone()
    }
}

impl TryFrom<&str> for MenuSelection {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_string().as_str() {
            "button_start_very_easy" => Ok(Self(Level::VeryEasy)),
            "button_start_easy" => Ok(Self(Level::Easy)),
            "button_start_normal" => Ok(Self(Level::Normal)),
            "button_start_hard" => Ok(Self(Level::Hard)),
            "button_start_absurd" => Ok(Self(Level::Absurd)),
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
        self.selection.next();
    }

    pub fn prev(&mut self) {
        self.selection.prev();
    }

    pub fn set(&mut self, selection: MenuSelection) {
        self.selection = selection;
    }
}

impl Component for MenuSelector {
    type Storage = HashMapStorage<Self>;
}
