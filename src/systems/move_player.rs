use super::system_prelude::*;

#[derive(Default)]
pub struct MovePlayerSystem;

impl<'a> System<'a> for MovePlayerSystem {
    type SystemData = (
        Read<'a, Time>,
        Read<'a, InputManager<Bindings>>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (time, input_manager, players, mut velocities): Self::SystemData,
    ) {
        if let Some((player, player_velocity)) =
            (&players, &mut velocities).join().next()
        {
            let dt = time.delta_seconds();

            if let Some(x) = input_manager.axis_value(AxisBinding::PlayerX) {
                if x != 0.0 {
                    player_velocity.increase_x_with_max(
                        player.acceleration.0 * x * dt,
                        player.max_velocity.0,
                    );
                }
            }
            if let Some(y) = input_manager.axis_value(AxisBinding::PlayerY) {
                if y != 0.0 {
                    player_velocity.increase_y_with_max(
                        player.acceleration.0 * y * dt,
                        player.max_velocity.0,
                    );
                }
            }
        }
    }
}
