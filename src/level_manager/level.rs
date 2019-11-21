use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Level {
    VeryEasy,
    Easy,
    Normal,
    Hard,
    Absurd,
}

impl Level {
    fn name(&self) -> &str {
        match self {
            Level::VeryEasy => "VeryEasy",
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

impl TryFrom<&str> for Level {
    type Error = String;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        match name.to_lowercase().as_str() {
            "veryeasy" | "very_easy" | "level_very_easy.json" => {
                Ok(Self::VeryEasy)
            }
            "easy" | "level_easy.json" => Ok(Self::Easy),
            "normal" | "level_normal.json" => Ok(Self::Normal),
            "hard" | "level_hard.json" => Ok(Self::Hard),
            "absurd" | "level_absurd.json" => Ok(Self::Absurd),
            n => Err(format!("Level cannot be created from String '{}'", n)),
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
