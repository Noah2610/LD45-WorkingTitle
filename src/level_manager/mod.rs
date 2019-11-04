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

pub struct LevelManager {
    level_name:       String,
    pub level_loader: LevelLoader,
    savefile_data:    Option<SavefileData>,
}

impl LevelManager {
    pub fn new<S>(level_name: S) -> Self
    where
        S: ToString,
    {
        Self {
            level_name:    level_name.to_string(),
            level_loader:  Default::default(),
            savefile_data: None,
        }
    }

    pub fn setup(&mut self, world: &mut World) {
        self.level_loader = LevelLoader::default();

        self.load_from_savefile(world);

        if world.read_resource::<Music>().should_audio_stop() {
            world.write_resource::<StopAudio>().0 = true;
        }

        // Create timer if a timer should run
        if world.read_resource::<CheckpointRes>().0.is_none() {
            world.write_resource::<TimerRes>().add_timer();
        }
    }

    pub fn reset(&mut self, world: &mut World) {
        self.level_loader.to_build = ToBuild::none()
            .with(BuildType::Backgrounds)
            .with(BuildType::Camera)
            .with(BuildType::Enemies)
            .with(BuildType::Features)
            .with(BuildType::Indicators)
            .with(BuildType::Player);

        self.level_loader.build(world);
        self.apply_checkpoint(world);

        if world.read_resource::<Music>().should_audio_stop() {
            world.write_resource::<StopAudio>().0 = true;
        }
    }

    pub fn win_level(&mut self, world: &mut World) {
        world.write_resource::<WinGame>().0 = true;
    }

    pub fn save_to_savefile(&mut self, world: &mut World) {
        let checkpoint_data = world.read_resource::<CheckpointRes>().0.clone();
        let music_data = MusicData::from(&*world.read_resource::<Music>());
        let player_deaths = world.read_resource::<PlayerDeaths>().0;

        let level_data = LevelSaveData {
            level_manager: LevelManagerData {
                level_name: self.level_name.to_string(),
            },
            checkpoint:    checkpoint_data.clone(),
            music:         music_data,
            stats:         StatsData { player_deaths },
        };
        self.savefile_data
            .get_or_insert_with(Default::default)
            .levels
            .insert(self.level_name.to_string(), level_data);

        match serde_json::to_string(&self.savefile_data) {
            Ok(serialized) => {
                let savefile_settings =
                    &world.read_resource::<Settings>().savefile;
                let savefile_path = file(&savefile_settings.filename);
                write_file(savefile_path, serialized).unwrap();
            }
            Err(err) => eprintln!(
                "Couldn't save savefile data to file, an error occured while \
                 serializing save data:\n{:#?}",
                err
            ),
        }
    }

    fn load_from_savefile(&mut self, world: &mut World) {
        let savefile_settings =
            world.read_resource::<Settings>().savefile.clone();
        let savefile_path = file(&savefile_settings.filename);
        if let Some(savefile_data) = get_savefile_data(savefile_path) {
            if let Some(level_data) = savefile_data.level(&self.level_name) {
                self.load_level(world);
                world.write_resource::<CheckpointRes>().0 =
                    level_data.checkpoint.clone();
                {
                    let mut music = world.write_resource::<Music>();
                    music.queue = level_data.music.queue.clone();
                }
                world.write_resource::<PlayerDeaths>().0 =
                    level_data.stats.player_deaths;
                self.apply_checkpoint(world);
            } else {
                // No save for this level
                self.load_level(world);
            }

            self.savefile_data = Some(savefile_data);
        } else {
            // No savefile
            self.load_level(world);
        }
    }

    fn load_level(&mut self, world: &mut World) {
        world.delete_all();
        world.write_resource::<CheckpointRes>().0 = None;
        self.level_loader.to_build = ToBuild::all();
        self.level_loader.load(&self.level_name);
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

fn get_savefile_data<S>(filepath: S) -> Option<SavefileData>
where
    S: ToString,
{
    use std::fs::File;
    use std::path::PathBuf;

    let savefile_path = PathBuf::from(filepath.to_string());
    if savefile_path.is_file() {
        let savefile_file = File::open(savefile_path)
            .expect("Savefile should exist at this point");
        let savefile_data: Option<SavefileData> =
            match serde_json::de::from_reader(savefile_file) {
                Ok(data) => Some(data),
                Err(e) => {
                    eprintln!("Couldn't deserialize savefile data: {:#?}", e);
                    None
                }
            };
        savefile_data
    } else {
        None
    }
}
