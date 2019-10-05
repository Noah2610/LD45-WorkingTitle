use super::system_prelude::*;

#[derive(Default)]
pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Time>,
        Read<'a, InputManager<Bindings>>,
        ReadStorage<'a, Collision>,
        ReadStorage<'a, Solid<SolidTag>>,
        ReadStorage<'a, CanJump>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, DecreaseVelocity>,
        WriteStorage<'a, Gravity>,
    );

    fn run(
        &mut self,
        (
            entities,
            time,
            input_manager,
            collisions,
            solids,
            can_jumps,
            mut players,
            mut velocities,
            mut decr_velocities,
            mut gravities,
        ): Self::SystemData,
    ) {
        if let Some((
            player_entity,
            player,
            player_velocity,
            player_decr_velocity,
            player_collision,
            player_solid,
            player_gravity_opt,
        )) = (
            &entities,
            &mut players,
            &mut velocities,
            &mut decr_velocities,
            &collisions,
            &solids,
            (&mut gravities).maybe(),
        )
            .join()
            .next()
        {
            let dt = time.delta_seconds();

            // MOVEMENT
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

            // JUMPING
            if can_jumps.contains(player_entity) {
                let sides_touching = SidesTouching::new(
                    &entities,
                    player_collision,
                    player_solid,
                    &collisions,
                    &solids,
                );

                if let Some(jump_data) = player.jump_data.as_ref() {
                    let mut jumped = false;
                    let can_jump = input_manager
                        .is_down(ActionBinding::PlayerJump)
                        && sides_touching.is_touching_bottom;

                    if can_jump {
                        jumped = true;
                        player_velocity.y += jump_data.jump_strength;
                    }

                    if let Some(player_gravity) = player_gravity_opt {
                        if jumped {
                            // Set different gravity when jumping
                            player_gravity.x = jump_data.jump_gravity.0;
                            player_gravity.y = jump_data.jump_gravity.1;
                        } else if input_manager.is_up(ActionBinding::PlayerJump)
                        {
                            // Kill some of the upwards momentum, keeping at least a certain minimum velocity
                            if player_velocity.y > jump_data.decr_jump_strength
                            {
                                player_velocity.y = (player_velocity.y
                                    - jump_data.decr_jump_strength)
                                    .max(jump_data.min_jump_velocity);
                            }
                            // Set default gravity
                            player_gravity.x = jump_data.gravity.0;
                            player_gravity.y = jump_data.gravity.1;
                        }
                    }
                }
            }
        }
    }
}
