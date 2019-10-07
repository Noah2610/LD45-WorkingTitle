use super::system_prelude::*;

const DEATH_FLOOR: f32 = -200.0;

#[derive(Default)]
pub struct DeathFloorSystem;

impl<'a> System<'a> for DeathFloorSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, ResetLevel>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Gravity>,
        ReadStorage<'a, Player>,
    );

    fn run(
        &mut self,
        (entities, mut reset_level, transforms, gravities, players): Self::SystemData,
    ) {
        for (entity, transform, _) in
            (&entities, &transforms, &gravities).join()
        {
            let pos = transform.translation();
            if pos.y < DEATH_FLOOR {
                entities.delete(entity).expect(
                    "Should delete entity, because they fell below the death \
                     floor",
                );
                if players.contains(entity) {
                    reset_level.0 = true;
                }
            }
        }
    }
}