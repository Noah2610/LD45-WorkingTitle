use super::system_prelude::*;

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
        for (_, trigger_collision) in
            (&dynamic_animation_triggers, &collisions).join()
        {
            for (target_entity, target_animations, _) in
                (&entities, &mut animations_containers, &dynamic_animations)
                    .join()
            {
                let target_id = target_entity.id();
                if let Some(collision_data) =
                    trigger_collision.collision_with(target_id)
                {
                    match collision_data.state {
                        collision::State::Enter => {
                            if target_animations.has_animation("on_collision") {
                                target_animations.play("on_collision");
                            }
                        }
                        collision::State::Leave => {
                            if target_animations
                                .has_animation("on_leave_collision")
                            {
                                target_animations.play("on_leave_collision");
                            }
                            target_animations.set_if_has("default");
                        }
                        collision::State::Steady | _ => {
                            target_animations.set_if_has("in_collision");
                        }
                    }
                }
            }
        }
    }
}
