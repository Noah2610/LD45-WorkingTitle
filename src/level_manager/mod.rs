pub mod level_loader;

use amethyst::ecs::{
    Entities,
    Entity,
    Join,
    ReadStorage,
    World,
    WorldExt,
    WriteStorage,
};

use crate::components::prelude::*;
use crate::helpers::*;
use crate::resources::prelude::*;
use crate::savefile_data::prelude::*;
use crate::settings::prelude::*;
use level_loader::{BuildType, LevelLoader, ToBuild};

#[derive(Default)]
pub struct LevelManager {
    pub level_loader: LevelLoader,
    pub level_names:  Vec<String>,
    pub level_index:  usize,
}

impl LevelManager {
    pub fn setup(&mut self, world: &mut World) {
        self.init_start(world);

        let settings = world.read_resource::<Settings>().level_manager.clone();
        self.level_names = settings.level_names;
        self.level_loader = LevelLoader::default();

        self.load_from_savefile(world);

        self.init_end(world);
    }

    pub fn reset(&mut self, world: &mut World) {
        self.init_start(world);

        self.level_loader.to_build = ToBuild::none()
            .with(BuildType::Backgrounds)
            .with(BuildType::Camera)
            .with(BuildType::Enemies)
            .with(BuildType::Features)
            .with(BuildType::Indicators)
            .with(BuildType::Player);

        self.level_loader.build(world);
        self.apply_checkpoint(world);

        self.init_end(world);
    }

    pub fn next_level(&mut self, world: &mut World) {
        {
            let timer = &mut world.write_resource::<TimerRes>().0;
            if timer.state.is_running() {
                timer.finish().unwrap();
                println!("---\nLEVEL TIME: {}\n---", timer.time_output());
            }
        }

        let next_index = self.level_index + 1;
        if next_index < self.level_names.len() {
            world.write_resource::<Music>().reset();
            world.write_resource::<StopAudio>().0 = true;
            self.level_index = next_index;
            self.load_current_level(world);
            self.save_to_savefile(world);

            // Start timer again
            let timer = &mut world.write_resource::<TimerRes>().0;
            if timer.state.is_finished() || timer.state.is_stopped() {
                timer.start().unwrap();
            }
        } else {
            world.write_resource::<WinGame>().0 = true;
        }
    }

    pub fn save_to_savefile(&self, world: &mut World) {
        let checkpoint_data = world.read_resource::<CheckpointRes>().0.clone();
        let music_data = MusicData::from(&*world.read_resource::<Music>());
        let player_deaths = world.read_resource::<PlayerDeaths>().0;

        let savefile_settings = &world.read_resource::<Settings>().savefile;
        let savefile_path = file(&savefile_settings.filename);
        let savefile_data = SavefileData {
            level_manager: LevelManagerData {
                level_name: self.level_name().to_string(),
            },
            checkpoint:    checkpoint_data.clone(),
            music:         music_data,
            stats:         StatsData { player_deaths },
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

    fn init_start(&self, world: &mut World) {
        world.write_resource::<ResetLevel>().0 = false;
        world.write_resource::<WinGame>().0 = false;
    }

    fn init_end(&mut self, world: &mut World) {
        if world.read_resource::<Music>().should_audio_stop() {
            world.write_resource::<StopAudio>().0 = true;
        }

        // Start timer
        if world.read_resource::<CheckpointRes>().0.is_none() {
            let timer = &mut world.write_resource::<TimerRes>().0;
            if timer.state.is_stopped() {
                timer.start().unwrap();
            }
        }
    }

    fn load_from_savefile(&mut self, world: &mut World) {
        use std::fs::File;
        use std::path::PathBuf;

        let savefile_settings =
            world.read_resource::<Settings>().savefile.clone();
        let savefile_path = PathBuf::from(file(&savefile_settings.filename));
        if savefile_path.is_file() {
            let savefile_file = File::open(savefile_path)
                .expect("Savefile should exist at this point");
            let savefile_data: Option<SavefileData> =
                match serde_json::de::from_reader(savefile_file) {
                    Ok(data) => Some(data),
                    Err(e) => {
                        eprintln!(
                            "Couldn't deserialize savefile data: {:#?}",
                            e
                        );
                        None
                    }
                };
            if let Some(savefile_data) = savefile_data {
                self.level_index = self
                    .level_names
                    .iter()
                    .position(|n| n == &savefile_data.level_manager.level_name)
                    .expect(&format!(
                        "Level name '{}' from savefile doesn't exist",
                        &savefile_data.level_manager.level_name,
                    ));
                self.load_current_level(world);
                world.write_resource::<CheckpointRes>().0 =
                    savefile_data.checkpoint.clone();
                {
                    let mut music = world.write_resource::<Music>();
                    music.queue = savefile_data.music.queue;
                }
                world.write_resource::<PlayerDeaths>().0 =
                    savefile_data.stats.player_deaths;
                self.apply_checkpoint(world);
            } else {
                self.load_current_level(world);
            }
        } else {
            // No savefile
            self.load_current_level(world);
        }
    }

    fn level_name(&self) -> &str {
        self.level_names
            .get(self.level_index)
            .expect(&format!(
                "Level at index {} doesn't exist",
                self.level_index
            ))
            .as_ref()
    }

    fn load_current_level(&mut self, world: &mut World) {
        world.delete_all();
        world.write_resource::<CheckpointRes>().0 = None;
        self.level_loader.to_build = ToBuild::all();
        self.level_loader.load(self.level_name().to_string());
        self.level_loader.build(world);
    }

    fn apply_checkpoint(&self, world: &mut World) {
        world.maintain();

        let checkpoint_data = world.read_resource::<CheckpointRes>().0.clone();
        if let Some(checkpoint) = checkpoint_data {
            world.exec(
                |(
                    entities,
                    players,
                    features,
                    mut transforms,
                    mut force_apply_features,
                ): (
                    Entities,
                    ReadStorage<Player>,
                    ReadStorage<Feature>,
                    WriteStorage<Transform>,
                    WriteStorage<ForceApplyFeature>,
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

                    // Set features to force apply
                    let mut song_feature: Option<(Entity, FeatureType)> = None;

                    for (feature_entity, feature) in
                        (&entities, &features).join()
                    {
                        if checkpoint.features.contains(&feature.feature_type) {
                            match feature.feature_type {
                                FeatureType::SetSong(n) => {
                                    if let Some((
                                        _,
                                        FeatureType::SetSong(last_n),
                                    )) = song_feature.as_ref()
                                    {
                                        if n > *last_n {
                                            song_feature = Some((
                                                feature_entity,
                                                feature.feature_type.clone(),
                                            ));
                                        }
                                    } else {
                                        song_feature = Some((
                                            feature_entity,
                                            feature.feature_type.clone(),
                                        ));
                                    }
                                }
                                _ => {
                                    force_apply_features
                                        .insert(
                                            feature_entity,
                                            ForceApplyFeature::default(),
                                        )
                                        .expect(
                                            "Should add ForceApplyFeature to \
                                             Feature",
                                        );
                                }
                            }
                        }
                    }

                    if let Some((feature_entity, _)) = song_feature {
                        force_apply_features
                            .insert(
                                feature_entity,
                                ForceApplyFeature::default(),
                            )
                            .expect(
                                "Should add ForceApplyFeature to Feature \
                                 (song)",
                            );
                    }
                },
            );
        } else {
            // If no checkpoint was set, then stop audio
            world.write_resource::<StopAudio>().0 = true;
        }
    }
}
