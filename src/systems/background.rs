use super::system_prelude::*;

#[derive(Default)]
pub struct BackgroundSystem;

impl<'a> System<'a> for BackgroundSystem {
    type SystemData = (
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Background>,
        ReadStorage<'a, AmethystCamera>,
    );

    fn run(
        &mut self,
        (transforms, sizes, backgrounds, cameras): Self::SystemData,
    ) {
        if let Some((_, camera_transform, camera_size)) =
            (&cameras, &transforms, &sizes).join().next()
        {
            let camera_left = {
                let pos = camera_transform.translation();
                pos.x - camera_size.w * 0.5
            };

            for (_, bg_transform, bg_size) in
                (&backgrounds, &transforms, &sizes).join()
            {
                let bg_left = {
                    let pos = bg_transform.translation();
                    pos.x - bg_size.w * 0.5
                };

                if bg_left <= camera_left {
                    // TODO: Make bg follow camera
                }
            }
        }
    }
}
