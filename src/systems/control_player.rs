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
        ReadStorage<'a, CanWallJump>,
        ReadStorage<'a, CanDash>,
        ReadStorage<'a, HasAnimatedSprite>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, DecreaseVelocity>,
        WriteStorage<'a, Gravity>,
        WriteStorage<'a, AnimationsContainer>,
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
            can_wall_jumps,
            can_dashes,
            has_animated_sprites,
            mut players,
            mut transforms,
            mut velocities,
            mut decr_velocities,
            mut gravities,
            mut animations_containers,
        ): Self::SystemData,
    ) {
        if let Some((
            player_entity,
            player,
            player_transform,
            player_velocity,
            player_decr_velocity,
            player_collision,
            player_solid,
            player_gravity_opt,
            player_animations_container,
        )) = (
            &entities,
            &mut players,
            &mut transforms,
            &mut velocities,
            &mut decr_velocities,
            &collisions,
            &solids,
            (&mut gravities).maybe(),
            &mut animations_containers,
        )
            .join()
            .next()
        {
            let dt = time.delta_seconds();
            let sides_touching = SidesTouching::new(
                &entities,
                player_collision,
                player_solid,
                &collisions,
                &solids,
            );

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
            if player_gravity_opt.is_none() {
                if let Some(y) = input_manager.axis_value(AxisBinding::PlayerY)
                {
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

            // JUMPING
            if can_jumps.contains(player_entity) {
                if let Some(jump_data) = player.jump_data.as_ref() {
                    let mut jumped = false;
                    let is_button_down =
                        input_manager.is_down(ActionBinding::PlayerJump);
                    let is_touching_bottom = sides_touching.is_touching_bottom;
                    let is_touching_horizontally =
                        sides_touching.is_touching_horizontally();

                    if is_touching_bottom {
                        player.used_dash = false;
                    }

                    let can_jump = is_button_down && is_touching_bottom;
                    let can_wall_jump = can_wall_jumps.contains(player_entity)
                        && is_button_down
                        && !sides_touching.is_touching_bottom
                        && is_touching_horizontally;
                    let can_dash = is_button_down
                        && can_dashes.contains(player_entity)
                        && !player.used_dash;

                    if is_touching_horizontally {
                        if player_velocity.y < player.settings.slide_velocity {
                            player_velocity.y = player.settings.slide_velocity;
                        }
                    }

                    if can_wall_jump {
                        if player_velocity.y < 0.0 {
                            player_velocity.y = 0.0;
                        }
                        jumped = true;
                        player_velocity.y += jump_data.wall_jump_strength.0;
                        if sides_touching.is_touching_left {
                            if player_velocity.x < 0.0 {
                                player_velocity.x = 0.0;
                            }
                            player_velocity.x += jump_data.wall_jump_strength.0;
                        } else if sides_touching.is_touching_right {
                            if player_velocity.x > 0.0 {
                                player_velocity.x = 0.0;
                            }
                            player_velocity.x -= jump_data.wall_jump_strength.0;
                        }
                    } else if can_jump {
                        if player_velocity.y < 0.0 {
                            player_velocity.y = 0.0;
                        }
                        jumped = true;
                        player_velocity.y += jump_data.jump_strength;
                    } else if can_dash {
                        let dashed = match (
                            input_manager.axis_value(AxisBinding::PlayerX),
                            input_manager.axis_value(AxisBinding::PlayerY),
                        ) {
                            // RIGHT-UP
                            (Some(x), Some(y)) if x > 0.0 && y > 0.0 => {
                                player_velocity.x = jump_data.dash_strength.0;
                                player_velocity.y = jump_data.dash_strength.1;
                                true
                            }
                            // RIGHT-DOWN
                            (Some(x), Some(y)) if x > 0.0 && y < 0.0 => {
                                player_velocity.x = jump_data.dash_strength.0;
                                player_velocity.y = -jump_data.dash_strength.1;
                                true
                            }
                            // LEFT-UP
                            (Some(x), Some(y)) if x < 0.0 && y > 0.0 => {
                                player_velocity.x = -jump_data.dash_strength.0;
                                player_velocity.y = jump_data.dash_strength.1;
                                true
                            }
                            // LEFT-DOWN
                            (Some(x), Some(y)) if x < 0.0 && y < 0.0 => {
                                player_velocity.x = -jump_data.dash_strength.0;
                                player_velocity.y = -jump_data.dash_strength.1;
                                true
                            }
                            // RIGHT
                            (Some(x), Some(y)) if x > 0.0 && y == 0.0 => {
                                player_velocity.x = jump_data.dash_strength.0;
                                true
                            }
                            // LEFT
                            (Some(x), Some(y)) if x < 0.0 && y == 0.0 => {
                                player_velocity.x = -jump_data.dash_strength.0;
                                true
                            }
                            // UP
                            (Some(x), Some(y)) if x == 0.0 && y > 0.0 => {
                                player_velocity.y = jump_data.dash_strength.1;
                                true
                            }
                            // DOWN
                            (Some(x), Some(y)) if x == 0.0 && y < 0.0 => {
                                player_velocity.y = -jump_data.dash_strength.1;
                                true
                            }
                            _ => false,
                        };
                        if dashed {
                            player.used_dash = true;
                        }
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

            if has_animated_sprites.contains(player_entity) {
                // Set animation
                if sides_touching.is_touching_bottom {
                    if player_velocity.x == 0.0 {
                        player_animations_container.set("idle");
                    } else {
                        player_animations_container.set("walk");
                    }
                } else {
                    if player_velocity.y > 0.0 {
                        player_animations_container.set("jump");
                    } else {
                        player_animations_container.set("fall");
                    }
                }
                // Flip sprite
                if player_velocity.x > 0.0 {
                    let scale = player_transform.scale_mut();
                    scale.x = scale.x.abs();
                } else if player_velocity.x < 0.0 {
                    let scale = player_transform.scale_mut();
                    scale.x = -scale.x.abs();
                }
            }
        }
    }
}
