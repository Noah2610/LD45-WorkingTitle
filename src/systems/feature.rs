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
        ReadStorage<'a, ForceApplyFeature>,
        ReadStorage<'a, Indicator>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Feature>,
        WriteStorage<'a, Size>,
        WriteStorage<'a, AnimationsContainer>,
        WriteStorage<'a, ScaleOnce>,
        WriteStorage<'a, Solid<SolidTag>>,
        WriteStorage<'a, Gravity>,
        WriteStorage<'a, CanJump>,
        WriteStorage<'a, CanWallJump>,
        WriteStorage<'a, HasSingleSprite>,
        WriteStorage<'a, HasAnimatedSprite>,
        WriteStorage<'a, CanRun>,
        WriteStorage<'a, CanDash>,
        WriteStorage<'a, Confined>,
        WriteStorage<'a, DecreaseVelocity>,
        WriteStorage<'a, CanHover>,
        WriteStorage<'a, Hidden>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut music,
            collisions,
            enemies,
            force_apply_features,
            indicators,
            mut players,
            mut features,
            mut sizes,
            mut animations_containers,
            mut scale_onces,
            mut solids,
            mut gravities,
            mut can_jumps,
            mut can_wall_jumps,
            mut has_single_sprites,
            mut has_animated_sprites,
            mut can_runs,
            mut can_dashes,
            mut confineds,
            mut decr_velocities,
            mut can_hovers,
            mut hiddens,
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
                if !feature.applied {
                    if player_collision.in_collision_with(feature_entity.id())
                        || force_apply_features.contains(feature_entity)
                    {
                        match &feature.feature_type {
                            FeatureType::AddCollisions => {
                                player_solid.tag =
                                    SolidTag::PlayerWithCollision;
                            }
                            FeatureType::AddGravity1 => {
                                let jump_settings =
                                    player.settings.jump_data1.clone();
                                add_gravity(
                                    jump_settings,
                                    player_entity,
                                    player,
                                    &mut gravities,
                                    &mut decr_velocities,
                                );
                                confineds.remove(player_entity);
                            }
                            FeatureType::AddGravity2 => {
                                let jump_settings =
                                    player.settings.jump_data2.clone();
                                add_gravity(
                                    jump_settings,
                                    player_entity,
                                    player,
                                    &mut gravities,
                                    &mut decr_velocities,
                                );
                                confineds.remove(player_entity);
                                can_hovers
                                    .insert(player_entity, CanHover::default())
                                    .expect("Should add CanHover to Player");
                            }
                            FeatureType::AddJump => {
                                can_jumps
                                    .insert(player_entity, CanJump::default())
                                    .expect("Should add CanJump to Player");
                                can_wall_jumps
                                    .insert(
                                        player_entity,
                                        CanWallJump::default(),
                                    )
                                    .expect("Should add CanWallJump to Player");
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
                            FeatureType::AddRun => {
                                can_runs
                                    .insert(player_entity, CanRun::default())
                                    .expect("Should add CanRun to Player");
                            }
                            FeatureType::AddDash => {
                                can_dashes
                                    .insert(player_entity, CanDash::default())
                                    .expect("Should add CanDash to Player");
                            }
                            FeatureType::SetSong(n) => {
                                if force_apply_features.contains(feature_entity)
                                {
                                    music.force_set(*n);
                                } else {
                                    music.set(*n);
                                }
                            }
                        }
                        feature.applied = true;

                        // Show indicator(s)
                        for indicator_entity in (&entities, &indicators)
                            .join()
                            .filter_map(|(indicator_entity, indicator)| {
                                if indicator.feature_trigger
                                    == feature.feature_type
                                {
                                    Some(indicator_entity.clone())
                                } else {
                                    None
                                }
                            })
                        {
                            hiddens.remove(indicator_entity);
                        }
                    }
                }
            }
        }
    }
}

fn add_gravity(
    jump_settings: PlayerJumpSettings,
    player_entity: Entity,
    player: &mut Player,
    gravities: &mut WriteStorage<Gravity>,
    decr_velocities: &mut WriteStorage<DecreaseVelocity>,
) {
    gravities
        .insert(
            player_entity,
            Gravity::new(jump_settings.gravity.0, jump_settings.gravity.1),
        )
        .expect("Should add Gravity to Player");
    decr_velocities
        .insert(
            player_entity,
            DecreaseVelocity::new(
                jump_settings.decr_velocity.0,
                jump_settings.decr_velocity.1,
            ),
        )
        .expect("Should add DecreaseVelocity to Player");
    player.jump_data = Some(jump_settings);
}
