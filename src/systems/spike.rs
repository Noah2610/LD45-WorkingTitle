use super::system_prelude::*;

#[derive(Default)]
pub struct SpikeSystem;

impl<'a> System<'a> for SpikeSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, ResetLevel>,
        ReadStorage<'a, Spike>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Collision>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut reset_level,
            spikes,
            players,
            collisions
        ): Self::SystemData,
    ) {
        if let Some((_, player_collision)) =
            (&players, &collisions).join().next()
        {
            for (spike_entity, spike) in (&entities, &spikes).join() {
                if spike.enabled
                    && player_collision.in_collision_with(spike_entity.id())
                {
                    reset_level.0 = true;
                    break;
                }
            }
        }
    }
}
