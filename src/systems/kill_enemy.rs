use super::system_prelude::*;

#[derive(Default)]
pub struct KillEnemySystem;

impl<'a> System<'a> for KillEnemySystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, Collision>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Spike>,
    );

    fn run(
        &mut self,
        (
            entities,
            players,
            enemies,
            collisions,
            mut velocities,
            mut spikes,
        ): Self::SystemData,
    ) {
        if let Some((player, player_collision, player_velocity)) =
            (&players, &collisions, &mut velocities).join().next()
        {
            if player_velocity.y < 0.0 {
                for (enemy_entity, _, enemy_spike_opt) in
                    (&entities, &enemies, (&mut spikes).maybe()).join()
                {
                    if player_collision.in_collision_with(enemy_entity.id()) {
                        // Kill enemy and make the player bounce off their head
                        player_velocity.y = player.settings.bounce_strength;
                        entities
                            .delete(enemy_entity)
                            .expect("Tried to kill enemy");
                        if let Some(enemy_spike) = enemy_spike_opt {
                            enemy_spike.enabled = false;
                        }
                    }
                }
            }
        }
    }
}
