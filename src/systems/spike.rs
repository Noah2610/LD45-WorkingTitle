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
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut reset_level,
            spikes,
            players,
            collisions,
            loadables,
            loadeds,
        ): Self::SystemData,
    ) {
        if let Some((_, player_collision)) =
            (&players, &collisions).join().next()
        {
            for (spike_entity, spike, loadable_opt, loaded_opt) in
                (&entities, &spikes, loadables.maybe(), loadeds.maybe()).join()
            {
                if let (None, None) | (Some(_), Some(_)) =
                    (loadable_opt, loaded_opt)
                {
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
}
