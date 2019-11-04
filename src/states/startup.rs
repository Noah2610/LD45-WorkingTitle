use super::state_prelude::*;
use crate::level_manager::LevelManager;

#[derive(Default)]
pub struct Startup;

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Startup {
    fn on_start(&mut self, data: StateData<CustomGameData<CustomData>>) {
        insert_resources(data.world);
        initialize_music(data.world);
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(data.world, "startup").unwrap();
        Trans::Push(Box::new(DifficultySelect::default()))
    }
}

fn insert_resources(world: &mut World) {
    use deathframe::handles::SpriteSheetHandles;

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
}

fn load_settings() -> Settings {
    use std::fs::File;

    let file = File::open(resource("config/settings.ron"))
        .expect("Couldn't open settings.ron file");
    ron::de::from_reader(file).expect("Failed parsing settings.ron file")
}
