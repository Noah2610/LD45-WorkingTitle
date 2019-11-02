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

    fn shadow_update(
        &mut self,
        mut data: StateData<CustomGameData<CustomData>>,
    ) {
        // Stop audio
        if data.world.read_resource::<StopAudio>().0 {
            stop_audio(&mut data.world);
            data.world.write_resource::<StopAudio>().0 = false;
        }

        // Next level
        if data.world.read_resource::<WinLevel>().0 {
            self.level_manager.next_level(&mut data.world);
            data.world.write_resource::<WinLevel>().0 = false;
        }

        // Should save to savefile
        if data.world.read_resource::<ShouldSave>().0 {
            self.level_manager.save_to_savefile(&mut data.world);
            data.world.write_resource::<ShouldSave>().0 = false;
        }
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
