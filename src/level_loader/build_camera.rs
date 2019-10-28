use super::*;

impl LevelLoader {
    pub(super) fn build_camera(&self, world: &mut World) {
        if let Some(player_data) = self.player_data.as_ref() {
            let player_pos = player_data.pos;

            let camera_settings =
                world.read_resource::<Settings>().camera.clone();

            let mut transform = Transform::default();
            transform.set_translation_xyz(player_pos.0, player_pos.1, CAMERA_Z);

            let size = Size::from(camera_settings.size);
            const LOADING_DISTANCE_PADDING: (f32, f32) = (64.0, 0.0);
            let loading_distance = (
                size.w * 0.5 + LOADING_DISTANCE_PADDING.0,
                size.h * 0.5 + LOADING_DISTANCE_PADDING.1,
            )
                .into();

            let mut entity = world
                .create_entity()
                .with(transform)
                .with(AmethystCamera::standard_2d(size.w, size.h))
                .with(size)
                .with(
                    Follower::new(FollowTag::Player)
                        .with_priority(1)
                        .with_round_pos(),
                )
                .with(Followed::new(FollowTag::Camera))
                .with(Loader::new(loading_distance));

            if let Some(level_size) = self.level_size.as_ref() {
                entity = entity.with(Confined::new(
                    Rect::builder()
                        .top(level_size.1)
                        .right(level_size.0)
                        .build(),
                ));
            }

            entity.build();
        }
    }
}
