use super::system_prelude::*;

#[derive(Default)]
pub struct CheckpointSystem;

impl<'a> System<'a> for CheckpointSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, CheckpointRes>,
        Write<'a, ShouldSave>,
        ReadStorage<'a, Transform>,
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
            collisions,
            players,
            features,
            mut checkpoints,
        ): Self::SystemData,
    ) {
        if let Some((_, player_collision, player_transform)) =
            (&players, &collisions, &transforms).join().next()
        {
            let player_pos = Vector::from(player_transform);

            for (checkpoint_entity, checkpoint) in
                (&entities, &mut checkpoints).join()
            {
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
                            position:    player_pos.clone(),
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
