use super::state_prelude::*;
use crate::level_loader::LevelLoader;

use amethyst::ecs::{Join, Read, ReadStorage, Write, WriteStorage};

const LEVEL_NAME: &str = "level.json";

#[derive(Default)]
pub struct Startup {
    level_loader: Option<LevelLoader>,
}

impl Startup {
    fn setup(&mut self, world: &mut World) {
        world.delete_all();

        world.write_resource::<ResetLevel>().0 = false;
        // Reset audio
        world.write_resource::<Music>().reset();
        {
            use amethyst::audio::output::Output;
            use amethyst::audio::AudioSink;

            let output = world.read_resource::<Output>();
            let mut sink = world.write_resource::<AudioSink>();
            sink.stop();
            *sink = AudioSink::new(&output);
            sink.set_volume(MUSIC_VOLUME);
        }

        let mut level_loader = LevelLoader::default();
        level_loader.load(LEVEL_NAME);
        level_loader.build(world);
        self.level_loader = Some(level_loader);

        self.apply_checkpoint(world);
    }

    fn apply_checkpoint(&self, world: &mut World) {
        let checkpoint_data = world.read_resource::<CheckpointRes>().0.clone();
        if let Some(checkpoint) = checkpoint_data {
            world.exec(
                |(players, mut transforms): (
                    ReadStorage<Player>,
                    WriteStorage<Transform>,
                )| {
                    // Set player position
                    if let Some((_, player_transform)) =
                        (&players, &mut transforms).join().next()
                    {
                        player_transform
                            .set_translation_x(checkpoint.position.0);
                        player_transform
                            .set_translation_y(checkpoint.position.1);
                    }
                },
            );
        }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Startup {
    fn on_start(&mut self, data: StateData<CustomGameData<CustomData>>) {
        insert_resources(data.world);
        initialize_music(data.world);

        self.setup(data.world);
    }

    fn on_resume(&mut self, data: StateData<CustomGameData<CustomData>>) {
        self.setup(data.world);
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(data.world, "startup").unwrap();

        if let Some(level_loader) = self.level_loader.as_ref() {
            if level_loader.is_finished() {
                Trans::Push(Box::new(Ingame::default()))
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}

fn insert_resources(world: &mut World) {
    use deathframe::handles::SpriteSheetHandles;

    world.insert(load_settings());
    world.insert(SpriteSheetHandles::default());
    world.insert(ResetLevel(false));
    world.insert(CheckpointRes::default());
}

fn load_settings() -> Settings {
    use std::fs::File;

    let file = File::open(resource("config/settings.ron"))
        .expect("Couldn't open settings.ron file");
    ron::de::from_reader(file).expect("Failed parsing settings.ron file")
}
