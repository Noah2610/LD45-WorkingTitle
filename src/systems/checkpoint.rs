use super::system_prelude::*;

#[derive(Default)]
pub struct CheckpointSystem;

impl<'a> System<'a> for CheckpointSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, CheckpointRes>,
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
                if !checkpoint.applied {
                    if player_collision
                        .in_collision_with(checkpoint_entity.id())
                    {
                        let checkpoint_data = CheckpointData {
                            position: player_pos.clone(),
                            features: features
                                .join()
                                .filter_map(|feature| {
                                    if feature.applied {
                                        Some(feature.feature_type.clone())
                                    } else {
                                        None
                                    }
                                })
                                .collect(),
                        };
                        checkpoint_res.0 = Some(checkpoint_data);
                        checkpoint.applied = true;
                    }
                }
            }
        }
    }
}
