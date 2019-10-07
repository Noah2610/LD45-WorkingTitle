use super::system_prelude::*;

#[derive(Default)]
pub struct EnemyAiSystem;

impl<'a> System<'a> for EnemyAiSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Time>,
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, HasAnimatedSprite>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
        WriteStorage<'a, EnemyAi>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, AnimationsContainer>,
    );

    fn run(
        &mut self,
        (
            entities,
            time,
            enemies,
            has_animated_sprites,
            loadables,
            loadeds,
            mut enemy_ais,
            mut transforms,
            mut velocities,
            mut animations_containers,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        for (
            enemy_entity,
            enemy,
            enemy_ai,
            enemy_transform,
            enemy_velocity,
            enemy_animations,
            loadable_opt,
            loaded_opt,
        ) in (
            &entities,
            &enemies,
            &mut enemy_ais,
            &mut transforms,
            &mut velocities,
            &mut animations_containers,
            loadables.maybe(),
            loadeds.maybe(),
        )
            .join()
        {
            if let (None, None) | (Some(_), Some(_)) =
                (loadable_opt, loaded_opt)
            {
                match enemy_ai {
                    EnemyAi::Pacer(pacer_data) => run_for_pacer_ai(
                        dt,
                        enemy,
                        pacer_data,
                        enemy_transform,
                        enemy_velocity,
                    ),
                }

                if has_animated_sprites.contains(enemy_entity) {
                    // Set animation
                    if enemy_velocity.x != 0.0 || enemy_velocity.y != 0.0 {
                        enemy_animations.set("walk");
                    } else {
                        enemy_animations.set("idle");
                    }
                    // Flip sprite
                    if enemy_velocity.x > 0.0 {
                        let scale = enemy_transform.scale_mut();
                        scale.x = scale.x.abs();
                    } else if enemy_velocity.x < 0.0 {
                        let scale = enemy_transform.scale_mut();
                        scale.x = -scale.x.abs();
                    }
                }
            }
        }
    }
}

fn run_for_pacer_ai(
    dt: f32,
    enemy: &Enemy,
    ai_data: &mut enemy_ai_data::PacerData,
    transform: &mut Transform,
    velocity: &mut Velocity,
) {
    let pos = {
        let trans = transform.translation();
        Vector::new(trans.x, trans.y)
    };

    // Pace X
    if let Some(pace_dist) = ai_data.pace_distance.0 {
        let mult = match ai_data.pacing_direction.0.clone() {
            enemy_ai_data::PacingDirectionX::Left => -1.0,
            enemy_ai_data::PacingDirectionX::Right => 1.0,
        };
        let dist = pos.0 - ai_data.origin.0;
        if dist.signum() == mult && dist.abs() >= pace_dist {
            ai_data.pacing_direction.0.invert();
            velocity.x = 0.0;
            transform.set_translation_x(ai_data.origin.0 + pace_dist * mult);
        } else {
            velocity.increase_x_with_max(
                enemy.acceleration.0 * mult * dt,
                enemy.max_velocity.0,
            );
        }
    }
    // Pace Y
    if let Some(pace_dist) = ai_data.pace_distance.1 {
        let mult = match ai_data.pacing_direction.1.clone() {
            enemy_ai_data::PacingDirectionY::Down => -1.0,
            enemy_ai_data::PacingDirectionY::Up => 1.0,
        };
        let dist = pos.1 - ai_data.origin.1;
        if dist.signum() == mult && dist.abs() >= pace_dist {
            ai_data.pacing_direction.1.invert();
            velocity.y = 0.0;
            transform.set_translation_y(ai_data.origin.1 + pace_dist * mult);
        } else {
            velocity.increase_y_with_max(
                enemy.acceleration.1 * mult * dt,
                enemy.max_velocity.1,
            );
        }
    }
}
