use super::system_prelude::*;

#[derive(Default)]
pub struct HandleSolidCollisionsSystem;

impl<'a> System<'a> for HandleSolidCollisionsSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Collision>,
        ReadStorage<'a, Solid<SolidTag>>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (entities, collisions, solids, mut velocities): Self::SystemData,
    ) {
        for (collision, solid, mut velocity) in
            (&collisions, &solids, &mut velocities).join()
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
        }
    }
}
