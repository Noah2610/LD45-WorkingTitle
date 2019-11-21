use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Level {
    Easy,
    Normal,
    Hard,
    Absurd,
}

impl Level {
    fn name(&self) -> &str {
        match self {
            Level::Easy => "Easy",
            Level::Normal => "Normal",
            Level::Hard => "Hard",
            Level::Absurd => "Absurd",
        }
    }
}

impl Default for Level {
    fn default() -> Self {
        Level::Easy
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
