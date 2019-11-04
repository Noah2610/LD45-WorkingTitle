use super::state_prelude::*;
use crate::level_manager::LevelManager;

pub struct LevelLoad {
    level_manager: LevelManager,
}

impl LevelLoad {
    pub fn new<S>(level_name: S) -> Self
    where
        S: ToString,
    {
        Self {
            level_manager: LevelManager::new(level_name),
        }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent>
    for LevelLoad
{
    fn on_start(&mut self, data: StateData<CustomGameData<CustomData>>) {
        self.level_manager.setup(data.world);
    }

    fn on_resume(&mut self, data: StateData<CustomGameData<CustomData>>) {
        // TODO: This happens when the Ingame state pops-off,
        // so we should probably return to the DifficultySelect state now.
        self.level_manager.reset(data.world);
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(data.world, "level_load").unwrap();

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
