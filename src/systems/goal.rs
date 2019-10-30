use super::system_prelude::*;

#[derive(Default)]
pub struct GoalSystem;

impl<'a> System<'a> for GoalSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, WinLevel>,
        ReadStorage<'a, Collision>,
        ReadStorage<'a, Goal>,
        ReadStorage<'a, Player>,
    );

    fn run(
        &mut self,
        (entities, mut win_level, collisions, goals, players): Self::SystemData,
    ) {
        if let Some((_, player_collision)) =
            (&players, &collisions).join().next()
        {
            for (goal_entity, _) in (&entities, &goals).join() {
                if player_collision.in_collision_with(goal_entity.id()) {
                    win_level.0 = true;
                }
            }
        }
    }
}
