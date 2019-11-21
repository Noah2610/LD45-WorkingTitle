use super::system_prelude::*;

const RESPAWN_POS_PADDING: (f32, f32) = (4.0, 4.0);

#[derive(Default)]
pub struct CheckpointSystem;

impl<'a> System<'a> for CheckpointSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, CheckpointRes>,
        Write<'a, ShouldSave>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Collision>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Feature>,
        WriteStorage<'a, Checkpoint>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut checkpoint_res,
            mut should_save,
            transforms,
            sizes,
            collisions,
            players,
            features,
            mut checkpoints,
        ): Self::SystemData,
    ) {
        if let Some((_, player_collision, player_size)) =
            (&players, &collisions, &sizes).join().next()
        {
            let player_size_padding = (
                player_size.w * 0.5 + RESPAWN_POS_PADDING.0,
                player_size.h * 0.5 + RESPAWN_POS_PADDING.1,
            )
                .into();

            for (
                checkpoint_entity,
                checkpoint,
                checkpoint_transform,
                checkpoint_size,
            ) in (&entities, &mut checkpoints, &transforms, &sizes).join()
            {
                let checkpoint_pos = checkpoint_transform.into();

                let prev_checkpoint_ids =
                    checkpoint_res.0.as_ref().map(|c| c.checkpoints.clone());

                if !checkpoint.applied
                    && prev_checkpoint_ids
                        .as_ref()
                        .map(|ids| !ids.contains(&checkpoint.id))
                        .unwrap_or(true)
                {
                    if player_collision
                        .in_collision_with(checkpoint_entity.id())
                    {
                        let mut checkpoints_ids = Vec::new();
                        if let Some(mut prev_ids) = prev_checkpoint_ids.clone()
                        {
                            checkpoints_ids.append(&mut prev_ids);
                        }
                        checkpoints_ids.push(checkpoint.id);

                        let checkpoint_data = CheckpointData {
                            position:    checkpoint.respawn_pos(
                                &checkpoint_pos,
                                &checkpoint_size,
                                &player_size_padding,
                            ),
                            features:    features
                                .join()
                                .filter_map(|feature| {
                                    if feature.applied {
                                        Some(feature.feature_type.clone())
                                    } else {
                                        None
                                    }
                                })
                                .collect(),
                            checkpoints: checkpoints_ids,
                        };
                        checkpoint_res.0 = Some(checkpoint_data);
                        checkpoint.applied = true;
                        should_save.0 = true;
                    }
                }
            }
        }
    }
}
