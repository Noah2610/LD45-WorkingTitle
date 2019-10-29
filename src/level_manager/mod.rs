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
use crate::resources::prelude::*;
use level_loader::{BuildType, LevelLoader, ToBuild};

const LEVEL_NAME: &str = "level.json";

#[derive(Default)]
pub struct LevelManager {
    pub level_loader: LevelLoader,
}

impl LevelManager {
    pub fn setup(&mut self, world: &mut World) {
        self.init(world);

        world.delete_all();

        self.level_loader = LevelLoader::default();
        self.level_loader.load(LEVEL_NAME);
        self.level_loader.build(world);
    }

    pub fn reset(&mut self, world: &mut World) {
        self.init(world);

        self.level_loader.to_build = ToBuild::none()
            .with(BuildType::Backgrounds)
            .with(BuildType::Camera)
            .with(BuildType::Enemies)
            .with(BuildType::Features)
            .with(BuildType::Indicators)
            .with(BuildType::Player);

        self.level_loader.build(world);
        self.apply_checkpoint(world);
    }

    fn init(&self, world: &mut World) {
        world.write_resource::<ResetLevel>().0 = false;
        world.write_resource::<WinGame>().0 = false;
        if world.read_resource::<Music>().should_audio_stop() {
            world.write_resource::<StopAudio>().0 = true;
        }
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
                    let mut gravity_feature: Option<(Entity, FeatureType)> =
                        None;

                    for (feature_entity, feature) in
                        (&entities, &features).join()
                    {
                        if checkpoint.features.contains(&feature.feature_type) {
                            match feature.feature_type {
                                FeatureType::SetSong(n) => {
                                    if let Some((
                                        feature_entity,
                                        FeatureType::SetSong(last_n),
                                    )) = song_feature.as_ref()
                                    {
                                        if n > *last_n {
                                            song_feature = Some((
                                                feature_entity.clone(),
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
                                FeatureType::AddGravity1 => {
                                    if gravity_feature.is_none() {
                                        gravity_feature = Some((
                                            feature_entity,
                                            FeatureType::AddGravity1,
                                        ));
                                    }
                                }
                                FeatureType::AddGravity2 => {
                                    gravity_feature = Some((
                                        feature_entity,
                                        FeatureType::AddGravity2,
                                    ));
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

                    if let Some((feature_entity, _)) = gravity_feature {
                        force_apply_features
                            .insert(
                                feature_entity,
                                ForceApplyFeature::default(),
                            )
                            .expect(
                                "Should add ForceApplyFeature to Feature \
                                 (gravity)",
                            );
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