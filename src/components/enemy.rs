use super::component_prelude::*;
use crate::helpers::resource;

pub enum EnemyType {
    Ground,
    Flying,
}

impl EnemyType {
    pub fn settings<'a>(
        &self,
        enemies_settings: &'a EnemiesSettings,
    ) -> &'a EnemySettings {
        match self {
            EnemyType::Ground => &enemies_settings.ground,
            EnemyType::Flying => &enemies_settings.flying,
        }
    }

    pub fn spritesheet_path(&self) -> String {
        resource(format!("spritesheets/{}.png", self.file_basename()))
    }

    pub fn animations_config_path(&self) -> String {
        resource(format!("animations/{}.ron", self.file_basename()))
    }

    fn file_basename(&self) -> &str {
        match self {
            EnemyType::Ground => "enemy_ground",
            EnemyType::Flying => "enemy_flying",
        }
    }
}

impl From<&str> for EnemyType {
    fn from(s: &str) -> Self {
        match s {
            "Ground" => EnemyType::Ground,
            "Flying" => EnemyType::Flying,
            s => panic!(format!("Unknown enemy_type {}", s)),
        }
    }
}

pub struct Enemy {
    pub enemy_type: EnemyType,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType) -> Self {
        Self { enemy_type }
    }
}

impl Component for Enemy {
    type Storage = VecStorage<Self>;
}
