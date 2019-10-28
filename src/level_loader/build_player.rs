use super::*;

impl LevelLoader {
    pub(super) fn build_player(&self, world: &mut World) {
        if let Some(EntityData {
            pos,
            size: _,
            properties,
            sprite: _,
        }) = self.player_data.as_ref()
        {
            let player_settings =
                world.read_resource::<Settings>().player.clone();

            let mut transform = Transform::default();
            transform.set_translation_xyz(
                pos.0,
                pos.1,
                properties[PROPERTY_Z_KEY].as_f32().unwrap_or(PLAYER_Z),
            );
            let size = Size::new(
                player_settings.animation_sizes.no_sprite.0,
                player_settings.animation_sizes.no_sprite.1,
            );

            let spritesheet_path = resource(format!(
                "spritesheets/{}",
                PLAYER_SPRITESHEET_FILENAME
            ));
            let (spritesheet_handle, sprite_render) = {
                let spritesheet_handle = world
                    .write_resource::<SpriteSheetHandles>()
                    .get_or_load(spritesheet_path, &world);
                (spritesheet_handle.clone(), SpriteRender {
                    sprite_sheet:  spritesheet_handle.clone(),
                    sprite_number: 0,
                })
            };

            let mut entity = world
                .create_entity()
                .with(Player::from(player_settings.clone()))
                .with(transform)
                .with(size)
                .with(Velocity::default())
                .with(sprite_render)
                .with(Transparent)
                .with(DecreaseVelocity::from(player_settings.decr_velocity))
                .with(ScaleOnce::default())
                .with(Collision::default())
                .with(CheckCollision::default())
                .with(Solid::new(SolidTag::PlayerNoCollision))
                .with(animations_container_from_file(
                    resource("animations/player.ron"),
                    spritesheet_handle,
                ))
                .with(Followed::new(FollowTag::Player));

            if let Some(level_size) = self.level_size.as_ref() {
                entity = entity.with(Confined::new(
                    Rect::builder()
                        .top(level_size.1)
                        .right(level_size.0)
                        .build(),
                ))
            }

            entity.build();
        } else {
            panic!("No player object in level");
        }
    }
}
