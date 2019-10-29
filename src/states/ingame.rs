use amethyst::audio::output::Output;
use amethyst::audio::AudioSink;

use super::state_prelude::*;
use crate::savefile_data::SavefileData;

#[derive(Default)]
pub struct Ingame;

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Ingame {
    fn update(
        &mut self,
        data: StateData<CustomGameData<CustomData>>,
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
        let mut stop_audio = data.world.write_resource::<StopAudio>();
        if stop_audio.0 {
            data.world.write_resource::<Music>().clear();

            let output = data.world.read_resource::<Output>();
            let mut sink = data.world.write_resource::<AudioSink>();
            sink.stop();
            *sink = AudioSink::new(&output);
            sink.set_volume(MUSIC_VOLUME);

            stop_audio.0 = false;
        }

        // Should save to savefile
        if data.world.read_resource::<ShouldSave>().0 {
            if let Some(checkpoint_data) =
                data.world.read_resource::<CheckpointRes>().0.clone()
            {
                let savefile_settings =
                    &data.world.read_resource::<Settings>().savefile;
                let savefile_path = file(&savefile_settings.filename);
                let savefile_data = SavefileData {
                    checkpoint: checkpoint_data.clone(),
                };

                match serde_json::to_string(&savefile_data) {
                    Ok(serialized) => {
                        dbg!("Writing to savefile");
                        dbg!(&serialized);
                        write_file(savefile_path, serialized).unwrap();
                    }
                    Err(err) => eprintln!(
                        "Couldn't save savefile data to file, an error \
                         occured while serializing save data:\n{:#?}",
                        err
                    ),
                }
            }

            data.world.write_resource::<ShouldSave>().0 = false;
        }

        Trans::None
    }
}
