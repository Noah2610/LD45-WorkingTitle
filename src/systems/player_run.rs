use super::system_prelude::*;

#[derive(Default)]
pub struct PlayerRunSystem;

impl<'a> System<'a> for PlayerRunSystem {
    type SystemData = (
        Read<'a, InputManager<Bindings>>,
        ReadStorage<'a, CanRun>,
        WriteStorage<'a, Player>,
    );

    fn run(
        &mut self,
        (input_manager, can_runs, mut players): Self::SystemData,
    ) {
        if let Some((player, _)) = (&mut players, &can_runs).join().next() {
            if input_manager.is_down(ActionBinding::PlayerRun) {
                player.set_run_speed();
            } else if input_manager.is_up(ActionBinding::PlayerRun) {
                player.set_normal_speed();
            }
        }
    }
}
