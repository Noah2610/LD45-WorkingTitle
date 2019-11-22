use crate::level_manager::Level;

#[derive(Clone, Deserialize)]
pub struct LevelManagerSettings {
    pub levels:                        Vec<LevelSettings>,
    pub description_text_color:        [f32; 4],
    pub locked_description_text_color: [f32; 4],
    pub default_locked_description:    String,
    pub default_locked_text_color:     [f32; 4],
}

#[derive(Clone, Deserialize)]
pub struct LevelSettings {
    pub level:              Level,
    pub filename:           String,
    pub win_text:           String,
    pub description:        String,
    pub locked_description: Option<String>,
    pub initially_locked:   bool,
    pub unlocked_by_any:    Option<Vec<Level>>,
    pub locked_text_color:  Option<[f32; 4]>,
}

impl LevelManagerSettings {
    pub fn level(&self, target: &Level) -> &LevelSettings {
        self.levels
            .iter()
            .find(|level| &level.level == target)
            .expect(&format!("Level {} should exist in settings", target))
    }
}
