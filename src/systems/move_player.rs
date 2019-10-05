use super::system_prelude::*;

#[derive(Default)]
pub struct MovePlayerSystem;

impl<'a> System<'a> for MovePlayerSystem {
    type SystemData = (
        Read<'a, Time>,
        Read<'a, InputManager<Bindings>>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, DecreaseVelocity>,
    );

    fn run(
        &mut self,
        (time, input_manager, players, mut velocities, mut decr_velocities): Self::SystemData,
    ) {
        if let Some((player, player_velocity, player_decr_velocity)) =
            (&players, &mut velocities, &mut decr_velocities)
                .join()
                .next()
        {
            let dt = time.delta_seconds();

            if let Some(x) = input_manager.axis_value(AxisBinding::PlayerX) {
                if x != 0.0 {
                    player_velocity.increase_x_with_max(
                        player.acceleration.0 * x * dt,
                        player.max_velocity.0,
                    );
                    // Don't decrease velocity when moving
                    if x > 0.0
                        && player
                            .max_velocity
                            .0
                            .map(|max| player_velocity.x <= max)
                            .unwrap_or(true)
                    {
                        player_decr_velocity.dont_decrease_x_when_pos();
                    } else if x < 0.0
                        && player
                            .max_velocity
                            .0
                            .map(|max| player_velocity.x >= -max)
                            .unwrap_or(true)
                    {
                        player_decr_velocity.dont_decrease_x_when_neg();
                    }
                }
            }
            if let Some(y) = input_manager.axis_value(AxisBinding::PlayerY) {
                if y != 0.0 {
                    player_velocity.increase_y_with_max(
                        player.acceleration.0 * y * dt,
                        player.max_velocity.0,
                    );
                    // Don't decrease velocity when moving
                    if y > 0.0
                        && player
                            .max_velocity
                            .1
                            .map(|max| player_velocity.y <= max)
                            .unwrap_or(true)
                    {
                        player_decr_velocity.dont_decrease_y_when_pos();
                    } else if y < 0.0
                        && player
                            .max_velocity
                            .1
                            .map(|max| player_velocity.y >= -max)
                            .unwrap_or(true)
                    {
                        player_decr_velocity.dont_decrease_y_when_neg();
                    }
                }
            }
        }
    }
}
