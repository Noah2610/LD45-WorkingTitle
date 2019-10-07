use super::system_prelude::*;

#[derive(Default)]
pub struct BackgroundSystem;

impl<'a> System<'a> for BackgroundSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Background>,
        ReadStorage<'a, AmethystCamera>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
        WriteStorage<'a, Follower>,
    );

    fn run(
        &mut self,
        (
            entities,
            transforms,
            sizes,
            backgrounds,
            cameras,
            loadables,
            loadeds,
            mut followers,
        ): Self::SystemData,
    ) {
        if let Some((_, camera_transform, camera_size)) =
            (&cameras, &transforms, &sizes).join().next()
        {
            let camera_left = {
                let pos = camera_transform.translation();
                pos.x - camera_size.w * 0.5
            };

            for (
                bg_entity,
                _,
                bg_transform,
                bg_size,
                loadable_opt,
                loaded_opt,
            ) in (
                &entities,
                &backgrounds,
                &transforms,
                &sizes,
                loadables.maybe(),
                loadeds.maybe(),
            )
                .join()
            {
                if let (None, None) | (Some(_), Some(_)) =
                    (loadable_opt, loaded_opt)
                {
                    if !followers.contains(bg_entity) {
                        let bg_left = {
                            let pos = bg_transform.translation();
                            pos.x - bg_size.w * 0.5
                        };

                        if bg_left <= camera_left {
                            followers
                                .insert(
                                    bg_entity,
                                    Follower::new(FollowTag::Camera),
                                )
                                .expect("Should add Follower to Background");
                        }
                    }
                }
            }
        }
    }
}
