use super::system_prelude::*;

#[derive(Default)]
pub struct EnemyAiSystem;

impl<'a> System<'a> for EnemyAiSystem {
    type SystemData = (
        Read<'a, Time>,
        ReadStorage<'a, Enemy>,
        WriteStorage<'a, EnemyAi>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (
            time,
            enemies,
            mut enemy_ais,
            mut transforms,
            mut velocities,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        for (enemy, enemy_ai, enemy_transform, enemy_velocity) in
            (&enemies, &mut enemy_ais, &mut transforms, &mut velocities).join()
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
