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
        data.world.delete_all();
        stop_audio(data.world);
        self.level_manager.setup(data.world);
    }

    fn on_stop(&mut self, data: StateData<CustomGameData<CustomData>>) {
        data.world.delete_all();
        data.world.write_resource::<TimerRes>().remove_timer();
        data.world.write_resource::<Music>().reset();
        data.world.write_resource::<CheckpointRes>().0 = None;
        data.world.write_resource::<PlayerDeaths>().0 = 0;
        data.world.write_resource::<WinLevel>().0 = false;
        data.world.write_resource::<WinGame>().0 = false;
        // data.world.write_resource::<StopAudio>().0 = true;
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(data.world, "level_load").unwrap();

        if data.world.read_resource::<ToMainMenu>().0 {
            data.world.write_resource::<ToMainMenu>().0 = false;
            return Trans::Pop;
        }

        if self.level_manager.level_loader.is_finished() {
            return Trans::Push(Box::new(Ingame::default()));
        }

        Trans::None
    }

    fn shadow_update(&mut self, data: StateData<CustomGameData<CustomData>>) {
        // Reset level
        if data.world.read_resource::<ResetLevel>().0 {
            self.level_manager.reset(data.world);
            data.world.write_resource::<ResetLevel>().0 = false;
        }

        // Win level
        if data.world.read_resource::<WinLevel>().0 {
            self.level_manager.win_level(data.world);
            data.world.write_resource::<WinLevel>().0 = false;
        }

        // Should save to savefile
        if data.world.read_resource::<ShouldSave>().0 {
            self.level_manager.save_to_savefile(data.world);
            data.world.write_resource::<ShouldSave>().0 = false;
        }
    }
}
