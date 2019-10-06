use super::system_prelude::*;

const DEATH_FLOOR: f32 = -200.0;

#[derive(Default)]
pub struct DeathFloorSystem;

impl<'a> System<'a> for DeathFloorSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Gravity>,
    );

    fn run(&mut self, (entities, transforms, gravities): Self::SystemData) {
        for (entity, transform, _) in
            (&entities, &transforms, &gravities).join()
        {
            let pos = transform.translation();
            if pos.y < DEATH_FLOOR {
                entities.delete(entity).expect(
                    "Should delete entity, because they fell below the death \
                     floor",
                );
            }
        }
    }
}
