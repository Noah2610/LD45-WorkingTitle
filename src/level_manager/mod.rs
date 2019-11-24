pub mod level;
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

pub use level::Level;

use crate::components::prelude::*;
use crate::resources::prelude::*;
use crate::savefile_data::prelude::*;
use crate::settings::prelude::*;
use level_loader::{BuildType, LevelLoader, ToBuild};

pub struct LevelManager {
    pub level:          Level,
    pub level_loader:   LevelLoader,
    should_delete_save: bool,
}

impl LevelManager {
    pub fn new(level: Level) -> Self {
        Self {
            level:              level,
            level_loader:       Default::default(),
            should_delete_save: false,
        }
    }

    pub fn with_delete_save(level: Level) -> Self {
        Self {
            level:              level,
            level_loader:       Default::default(),
            should_delete_save: true,
        }
    }

    pub fn setup(&mut self, world: &mut World) {
        self.level_loader = LevelLoader::default();

        self.reset_level(world);
        self.load_from_savefile(world);

        if world.read_resource::<Music>().should_audio_stop() {
            world.write_resource::<StopAudio>().0 = true;
        }

        // Create timer if a timer should run
        if world.read_resource::<CheckpointRes>().0.is_none() {
            world.write_resource::<TimerRes>().add_timer();
        } else {
            world.write_resource::<TimerRes>().remove_timer();
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
        // Finish timer here, so the time is saved to the savefile.
        if let Some(timer) = world.write_resource::<TimerRes>().0.as_mut() {
            if timer.state.is_running() {
                timer.finish().unwrap();
                println!("---\nLEVEL TIME: {}\n---", timer.time_output());
            }
        }
        world.write_resource::<WinGame>().0 = true;
        // Clear these resources, so when the game saves,
        // it resets the relevant data for this level.
        // This way, after the level was beaten and the player
        // starts the same level again, they will start at the beginning.
        world.write_resource::<CheckpointRes>().0 = None;
        world.write_resource::<Music>().reset();
        world.write_resource::<PlayerDeaths>().0 = 0;
        self.save_to_savefile(world, true);
    }

    pub fn save_to_savefile(&mut self, world: &mut World, won: bool) {
        {
            let checkpoint_data =
                world.read_resource::<CheckpointRes>().0.clone();
            let music_data = MusicData::from(&*world.read_resource::<Music>());
            let player_deaths = world.read_resource::<PlayerDeaths>().0;

            let savefile_data_res =
                &mut world.write_resource::<SavefileDataRes>().0;
            let savefile_data =
                savefile_data_res.get_or_insert_with(Default::default);

            let existing_level_data = savefile_data.levels.get(&self.level);

            let time = world
                .read_resource::<TimerRes>()
                .0
                .as_ref()
                .filter(|timer| timer.state.is_finished())
                .map(|timer| timer.time_output());
            let level_data = LevelSaveData {
                checkpoint: checkpoint_data.clone(),
                music:      music_data,
                stats:      StatsData { player_deaths },
                best_time:  existing_level_data
                    .and_then(|p| p.best_time)
                    .map(|prev_time| {
                        if let Some(time) = time {
                            if time < prev_time {
                                time
                            } else {
                                prev_time
                            }
                        } else {
                            prev_time
                        }
                    })
                    .or(time),
                won:        won
                    || existing_level_data.map(|d| d.won).unwrap_or(false),
            };
            savefile_data.levels.insert(self.level.clone(), level_data);
        }

        save_savefile_data(world);
    }

    fn load_from_savefile(&mut self, world: &mut World) {
        let mut should_apply_checkpoint = false;

        if let Some(savefile_data) =
            world.read_resource::<SavefileDataRes>().0.as_ref()
        {
            if let Some(level_data) = savefile_data.level(&self.level) {
                // Set SHOULD_DISPLAY_TIMER
                world.write_resource::<ShouldDisplayTimer>().0 = level_data.won
                    && (level_data.checkpoint.is_none()
                        || self.should_delete_save);
                // Set BEST_TIME
                if let Some(best_time) = level_data.best_time.as_ref() {
                    world.write_resource::<BestTime>().0 =
                        Some(best_time.clone());
                }

                // Don't apply this level's save
                if !self.should_delete_save {
                    {
                        // Set CHECKPOINT
                        world.write_resource::<CheckpointRes>().0 =
                            level_data.checkpoint.clone();
                        // Set MUSIC
                        world.write_resource::<Music>().queue =
                            level_data.music.queue.clone();
                        // Set PLAYER_DEATHS
                        world.write_resource::<PlayerDeaths>().0 =
                            level_data.stats.player_deaths;
                        // Apply the set checkpoint data later (due to borrow checker)
                        should_apply_checkpoint = true;
                    }
                }
            } else {
                // No save for this level
            }
        } else {
            // No savefile
        }

        // Apply checkpoint
        if should_apply_checkpoint {
            self.apply_checkpoint(world);
        }
    }

    fn reset_level(&mut self, world: &mut World) {
        // Reset some resources
        // Reset CHECKPOINT
        world.write_resource::<CheckpointRes>().0 = None;
        // Reset MUSIC
        world.write_resource::<Music>().reset();
        // Reset PLAYER_DEATHS
        world.write_resource::<PlayerDeaths>().0 = 0;
        // Reset SHOULD_DISPLAY_TIMER
        world.write_resource::<ShouldDisplayTimer>().0 = false;
        // Reset BEST_TIME
        world.write_resource::<BestTime>().0 = None;

        // Load level
        self.load_level(world);
    }

    fn load_level(&mut self, world: &mut World) {
        let level_filename = world
            .read_resource::<Settings>()
            .level_manager
            .level(&self.level)
            .filename
            .clone();
        world.delete_all();
        world.write_resource::<CheckpointRes>().0 = None;
        self.level_loader.to_build = ToBuild::all();
        self.level_loader.load(level_filename);
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
