use super::system_prelude::*;

const ANIM_NAME_COLLISION_STEADY: &str = "in_collision";
const ANIM_NAME_COLLISION_ENTER: &str = "on_collision_enter";
const ANIM_NAME_COLLISION_LEAVE: &str = "on_collision_leave";

#[derive(Default)]
pub struct DynamicAnimationSystem;

impl<'a> System<'a> for DynamicAnimationSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, DynamicAnimation>,
        ReadStorage<'a, DynamicAnimationTrigger>,
        ReadStorage<'a, Collision>,
        WriteStorage<'a, AnimationsContainer>,
    );

    fn run(
        &mut self,
        (
            entities,
            dynamic_animations,
            dynamic_animation_triggers,
            collisions,
            mut animations_containers,
        ): Self::SystemData,
    ) {
        for (target_animations, target_collision, _) in
            (&mut animations_containers, &collisions, &dynamic_animations)
                .join()
        {
            let mut set_animation = false;

            for (trigger_entity, _) in
                (&entities, &dynamic_animation_triggers).join()
            {
                let trigger_id = trigger_entity.id();

                if let Some(collision_data) =
                    target_collision.collision_with(trigger_id)
                {
                    match collision_data.state {
                        collision::State::Enter => {
                            if target_animations
                                .has_animation(ANIM_NAME_COLLISION_ENTER)
                            {
                                target_animations
                                    .play(ANIM_NAME_COLLISION_ENTER);
                            }
                        }
                        collision::State::Leave => {
                            if target_animations
                                .has_animation(ANIM_NAME_COLLISION_LEAVE)
                            {
                                target_animations
                                    .play(ANIM_NAME_COLLISION_LEAVE);
                            }
                            target_animations.set_if_has("default");
                        }
                        collision::State::Steady | _ => {
                            target_animations
                                .set_if_has(ANIM_NAME_COLLISION_STEADY);
                        }
                    }
                    // Only run for a single trigger.
                    // Ignore any other triggers that may also be in collision.
                    set_animation = true;
                    break;
                }
            }

            if !set_animation {
                target_animations.set_if_has("default");
            }
        }
    }
}
