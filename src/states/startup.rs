use super::state_prelude::*;
use crate::level_manager::LevelManager;

#[derive(Default)]
pub struct Startup {
    level_manager: LevelManager,
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Startup {
    fn on_start(&mut self, data: StateData<CustomGameData<CustomData>>) {
        insert_resources(data.world);
        initialize_music(data.world);

        self.level_manager.setup(data.world);
    }

    fn on_resume(&mut self, data: StateData<CustomGameData<CustomData>>) {
        self.level_manager.reset(data.world);
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(data.world, "startup").unwrap();

        if self.level_manager.level_loader.is_finished() {
            Trans::Push(Box::new(Ingame::default()))
        } else {
            Trans::None
        }
    }
}

fn insert_resources(world: &mut World) {
    use deathframe::handles::SpriteSheetHandles;

    world.insert(load_settings());
    world.insert(SpriteSheetHandles::default());
    world.insert(ResetLevel::default());
    world.insert(CheckpointRes::default());
    world.insert(WinGame::default());
    world.insert(StopAudio::default());
}

fn load_settings() -> Settings {
    use std::fs::File;

    let file = File::open(resource("config/settings.ron"))
        .expect("Couldn't open settings.ron file");
    ron::de::from_reader(file).expect("Failed parsing settings.ron file")
}
