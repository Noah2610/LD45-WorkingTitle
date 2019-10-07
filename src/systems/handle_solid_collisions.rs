use super::system_prelude::*;

#[derive(Default)]
pub struct HandleSolidCollisionsSystem;

impl<'a> System<'a> for HandleSolidCollisionsSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Collision>,
        ReadStorage<'a, Solid<SolidTag>>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (
            entities,
            collisions,
            solids,
            loadables,
            loadeds,
            mut velocities
        ): Self::SystemData,
    ) {
        for (collision, solid, mut velocity, loadable_opt, loaded_opt) in (
            &collisions,
            &solids,
            &mut velocities,
            loadables.maybe(),
            loadeds.maybe(),
        )
            .join()
        {
            if let (None, None) | (Some(_), Some(_)) =
                (loadable_opt, loaded_opt)
            {
                let sides_touching = SidesTouching::new(
                    &entities,
                    collision,
                    solid,
                    &collisions,
                    &solids,
                );

                if (sides_touching.is_touching_top && velocity.y > 0.0)
                    || (sides_touching.is_touching_bottom && velocity.y < 0.0)
                {
                    velocity.y = 0.0;
                }
                if (sides_touching.is_touching_right && velocity.x > 0.0)
                    || (sides_touching.is_touching_left && velocity.x < 0.0)
                {
                    velocity.x = 0.0;
                }
            }
        }
    }
}
