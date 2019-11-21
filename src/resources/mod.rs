pub mod prelude {
    pub use climer::Timer;
    pub use deathframe::handles::SpriteSheetHandles;

    pub use super::best_time::BestTime;
    pub use super::checkpoint::{CheckpointData, CheckpointRes};
    pub use super::insert_resources;
    pub use super::player_deaths::PlayerDeaths;
    pub use super::reset_level::ResetLevel;
    pub use super::savefile_data::SavefileDataRes;
    pub use super::should_display_timer::ShouldDisplayTimer;
    pub use super::should_save::ShouldSave;
    pub use super::stop_audio::StopAudio;
    pub use super::timer::TimerRes;
    pub use super::to_main_menu::ToMainMenu;
    pub use super::win_game::WinGame;
    pub use super::win_level::WinLevel;
    pub use crate::audio::Music;
    pub use crate::level_manager::LevelManager;
}

mod best_time;
mod checkpoint;
mod player_deaths;
mod reset_level;
mod savefile_data;
mod should_display_timer;
mod should_save;
mod stop_audio;
mod timer;
mod to_main_menu;
mod win_game;
mod win_level;

use amethyst::ecs::World;

use crate::helpers::resource;
use crate::settings::Settings;

pub fn insert_resources(world: &mut World) {
    use prelude::*;

    world.insert(load_settings());
    world.insert(SpriteSheetHandles::default());
    world.insert(ResetLevel::default());
    world.insert(CheckpointRes::default());
    world.insert(WinLevel::default());
    world.insert(WinGame::default());
    world.insert(StopAudio::default());
    world.insert(ShouldSave::default());
    world.insert(PlayerDeaths::default());
    world.insert(TimerRes::default());
    world.insert(ToMainMenu::default());
    world.insert(ShouldDisplayTimer::default());
    world.insert(BestTime::default());
    world.insert(SavefileDataRes::default());
}

fn load_settings() -> Settings {
    use std::fs::File;

    let file = File::open(resource("config/settings.ron"))
        .expect("Couldn't open settings.ron file");
    ron::de::from_reader(file).expect("Failed parsing settings.ron file")
}
