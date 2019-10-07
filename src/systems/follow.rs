use std::collections::HashMap;

use super::system_prelude::*;

#[derive(Default)]
pub struct FollowSystem;

impl<'a> System<'a> for FollowSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Followed>,
        ReadStorage<'a, Follower>,
        WriteStorage<'a, Transform>,
    );

    fn run(
        &mut self,
        (entities, followeds, followers, mut transforms): Self::SystemData,
    ) {
        let mut followeds_data = (&followeds, &transforms)
            .join()
            .map(|(followed, followed_transform)| {
                (followed.tag.clone(), Vector::from(followed_transform))
            })
            .collect::<HashMap<FollowTag, Vector>>();

        let mut join_vec = (&entities, &followers, &mut transforms)
            .join()
            .collect::<Vec<_>>();
        join_vec.sort_by(|(_, follower_a, _), (_, follower_b, _)| {
            follower_a.partial_cmp(follower_b).unwrap()
        });

        for (follower_entity, follower, follower_transform) in join_vec {
            if let Some(followed_pos) =
                followeds_data.get(&follower.tag).cloned()
            {
                let pos = if follower.round_pos {
                    (followed_pos.0.round(), followed_pos.1.round()).into()
                } else {
                    followed_pos
                };
                follower_transform.set_translation_x(pos.0);
                follower_transform.set_translation_y(pos.1);

                if let Some(follower_followed) = followeds.get(follower_entity)
                {
                    if let Some(pos) =
                        followeds_data.get_mut(&follower_followed.tag)
                    {
                        *pos = pos.clone();
                    }
                }
            }
        }
    }
}
