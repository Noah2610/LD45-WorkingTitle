use super::system_prelude::*;

#[derive(Default)]
pub struct SpikeSystem;

impl<'a> System<'a> for SpikeSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, ResetLevel>,
        Write<'a, PlayerDeaths>,
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
            mut player_deaths,
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
                    if spike.enabled {
                        if let Some(collision::Data {
                            side: Side::Inner, ..
                        }) =
                            player_collision.collision_with(spike_entity.id())
                        {
                            reset_level.0 = true;
                            player_deaths.0 += 1;
                            break;
                        }
                    }
                }
            }
        }
    }
}
