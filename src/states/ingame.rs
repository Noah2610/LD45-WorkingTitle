use amethyst::audio::output::Output;
use amethyst::audio::AudioSink;

use super::state_prelude::*;
use crate::savefile_data::prelude::*;

#[derive(Default)]
pub struct Ingame;

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Ingame {
    fn update(
        &mut self,
        mut data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(data.world, "ingame").unwrap();

        // Reset level
        if data.world.read_resource::<ResetLevel>().0 {
            return Trans::Pop;
        }

        // Win game
        if data.world.read_resource::<WinGame>().0 {
            return Trans::Push(Box::new(Win::default()));
        }

        // Stop audio
        if data.world.read_resource::<StopAudio>().0 {
            stop_audio(&mut data.world);
            data.world.write_resource::<StopAudio>().0 = false;
        }

        // Should save to savefile
        if data.world.read_resource::<ShouldSave>().0 {
            save_to_savefile(&mut data.world);
            data.world.write_resource::<ShouldSave>().0 = false;
        }

        Trans::None
    }
}

fn stop_audio(world: &mut World) {
    world.write_resource::<Music>().clear();

    let output = world.read_resource::<Output>();
    let mut sink = world.write_resource::<AudioSink>();
    sink.stop();
    *sink = AudioSink::new(&output);
    sink.set_volume(MUSIC_VOLUME);
}

fn save_to_savefile(world: &mut World) {
    if let Some(checkpoint_data) =
        world.read_resource::<CheckpointRes>().0.clone()
    {
        let music_data = MusicData::from(&*world.read_resource::<Music>());
        let savefile_settings = &world.read_resource::<Settings>().savefile;
        let savefile_path = file(&savefile_settings.filename);
        let savefile_data = SavefileData {
            checkpoint: checkpoint_data.clone(),
            music:      music_data,
        };

        match serde_json::to_string(&savefile_data) {
            Ok(serialized) => {
                write_file(savefile_path, serialized).unwrap();
            }
            Err(err) => eprintln!(
                "Couldn't save savefile data to file, an error occured while \
                 serializing save data:\n{:#?}",
                err
            ),
        }
    }
}
