use amethyst::audio::output::Output;
use amethyst::audio::AudioSink;

use super::state_prelude::*;

#[derive(Default)]
pub struct Ingame;

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Ingame {
    fn update(
        &mut self,
        mut data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(data.world, "ingame").unwrap();

        // Handle input
        if let Some(trans) = self.handle_keys(&data.world) {
            return trans;
        }

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

        Trans::None
    }
}

impl Ingame {
    fn handle_keys<'a, 'b>(
        &mut self,
        world: &World,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        let input = world.read_resource::<InputManager<Bindings>>();

        if input.is_down(ActionBinding::Quit) {
            Some(Trans::Quit)
        } else if input.is_down(ActionBinding::TogglePause) {
            Some(Trans::Push(Box::new(Paused::default())))
        } else {
            None
        }
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
