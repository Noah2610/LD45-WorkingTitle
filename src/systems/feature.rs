use super::system_prelude::*;
use crate::solid_tag::SolidTag;

#[derive(Default)]
pub struct FeatureSystem;

impl<'a> System<'a> for FeatureSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Collision>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Feature>,
        WriteStorage<'a, Solid<SolidTag>>,
        WriteStorage<'a, Gravity>,
        WriteStorage<'a, CanJump>,
    );

    fn run(
        &mut self,
        (
            entities,
            collisions,
            mut players,
            mut features,
            mut solids,
            mut gravities,
            mut can_jumps,
        ): Self::SystemData,
    ) {
        if let Some((player_entity, player, player_collision, player_solid)) =
            (&entities, &mut players, &collisions, &mut solids)
                .join()
                .next()
        {
            for (feature_entity, feature) in (&entities, &mut features).join() {
                if player_collision.in_collision_with(feature_entity.id()) {
                    if !feature.applied {
                        match &feature.feature_type {
                            FeatureType::AddCollisions => {
                                player_solid.tag =
                                    SolidTag::PlayerWithCollision;
                            }
                            FeatureType::AddGravity1 => {
                                let jump_settings = &player.settings.jump_data1;
                                gravities
                                    .insert(
                                        player_entity,
                                        Gravity::new(
                                            jump_settings.gravity.0,
                                            jump_settings.gravity.1,
                                        ),
                                    )
                                    .expect("Should add Gravity to Player");
                                player.jump_data = Some(PlayerJumpData {
                                    jump_strength:      jump_settings
                                        .jump_strength,
                                    gravity:            jump_settings.gravity,
                                    jump_gravity:       jump_settings
                                        .jump_gravity,
                                    decr_jump_strength: jump_settings
                                        .decr_jump_strength,
                                    min_jump_velocity:  jump_settings
                                        .min_jump_velocity,
                                });
                            }
                            FeatureType::AddJump => {
                                can_jumps
                                    .insert(player_entity, CanJump::default())
                                    .expect("Should add CanJump to Player");
                            }
                        }
                        feature.applied = true;
                    }
                }
            }
        }
    }
}
