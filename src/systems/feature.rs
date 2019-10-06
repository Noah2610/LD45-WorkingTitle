use super::system_prelude::*;
use crate::solid_tag::SolidTag;

#[derive(Default)]
pub struct FeatureSystem;

impl<'a> System<'a> for FeatureSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, Music>,
        ReadStorage<'a, Collision>,
        ReadStorage<'a, Enemy>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Feature>,
        WriteStorage<'a, Size>,
        WriteStorage<'a, AnimationsContainer>,
        WriteStorage<'a, ScaleOnce>,
        WriteStorage<'a, Solid<SolidTag>>,
        WriteStorage<'a, Gravity>,
        WriteStorage<'a, CanJump>,
        WriteStorage<'a, HasSingleSprite>,
        WriteStorage<'a, HasAnimatedSprite>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut music,
            collisions,
            enemies,
            mut players,
            mut features,
            mut sizes,
            mut animations_containers,
            mut scale_onces,
            mut solids,
            mut gravities,
            mut can_jumps,
            mut has_single_sprites,
            mut has_animated_sprites,
        ): Self::SystemData,
    ) {
        if let Some((
            player_entity,
            player,
            player_size,
            player_animations,
            player_collision,
            player_solid,
        )) = (
            &entities,
            &mut players,
            &mut sizes,
            &mut animations_containers,
            &collisions,
            &mut solids,
        )
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
                            FeatureType::AddSingleSprite => {
                                has_single_sprites
                                    .insert(
                                        player_entity,
                                        HasSingleSprite::default(),
                                    )
                                    .expect(
                                        "Should add HasSingleSprite to Player",
                                    );
                                player_animations.set("single_sprite");
                                let animation_size = player
                                    .settings
                                    .animation_sizes
                                    .single_sprite;
                                player_size.w = animation_size.0;
                                player_size.h = animation_size.1;
                                scale_onces
                                    .insert(player_entity, ScaleOnce::default())
                                    .expect("Should add ScaleOnce to Player");
                            }
                            FeatureType::AddAnimatedSprite => {
                                has_single_sprites.remove(player_entity);
                                has_animated_sprites
                                    .insert(
                                        player_entity,
                                        HasAnimatedSprite::default(),
                                    )
                                    .expect(
                                        "Should add HasAnimatedSprite to \
                                         Player",
                                    );
                                player_animations.set("idle");
                                let animation_size = player
                                    .settings
                                    .animation_sizes
                                    .animated_sprite;
                                player_size.w = animation_size.0;
                                player_size.h = animation_size.1;
                                scale_onces
                                    .insert(player_entity, ScaleOnce::default())
                                    .expect("Should add ScaleOnce to Player");
                            }
                            FeatureType::AddEnemySprite => {
                                for (enemy_entity, _) in
                                    (&entities, &enemies).join()
                                {
                                    has_animated_sprites
                                        .insert(
                                            enemy_entity,
                                            HasAnimatedSprite::default(),
                                        )
                                        .expect(
                                            "Should add HasAnimatedSprite to \
                                             Enemy",
                                        );
                                }
                            }
                            FeatureType::SetSong1 => {
                                music.set(Song::Song1);
                            }
                            FeatureType::SetSong2 => {
                                music.set(Song::Song2);
                            }
                        }
                        feature.applied = true;
                    }
                }
            }
        }
    }
}
