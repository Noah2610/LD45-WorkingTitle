use super::system_prelude::*;

#[derive(Default)]
pub struct CameraSystem;

impl<'a> System<'a> for CameraSystem {
    type SystemData = (
        ReadStorage<'a, AmethystCamera>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (cameras, players, mut transforms): Self::SystemData) {
        let player_pos_opt = (&players, &transforms)
            .join()
            .next()
            .map(|(_, player_transform)| Vector::from(player_transform));

        if let Some(player_pos) = player_pos_opt {
            if let Some((_, camera_transform)) =
                (&cameras, &mut transforms).join().next()
            {
                camera_transform.set_translation_x(player_pos.0);
                camera_transform.set_translation_y(player_pos.1);
            }
        }
    }
}
