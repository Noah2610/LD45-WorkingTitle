use super::*;

impl LevelLoader {
    pub(super) fn build_backgrounds(&self, world: &mut World) {
        // Delete existing entities
        world.exec(
            |(entities, backgrounds): (Entities, ReadStorage<Background>)| {
                for (entity, _) in (&entities, &backgrounds).join() {
                    entities.delete(entity).unwrap();
                }
            },
        );

        for EntityData {
            pos,
            size,
            sprite: _,
            properties,
        } in &self.backgrounds_data
        {
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                pos.0,
                pos.1,
                properties[PROPERTY_Z_KEY].as_f32().unwrap_or(BACKGROUND_Z),
            );

            let image_name = properties["image"]
                .as_str()
                .expect("Background object has to have an 'image' property");

            let spritesheet_path =
                resource(format!("{}/{}", BACKGROUNDS_DIR, image_name));
            let sprite_render = {
                let spritesheet_handle = world
                    .write_resource::<SpriteSheetHandles>()
                    .get_or_load(spritesheet_path, &world);
                SpriteRender {
                    sprite_sheet:  spritesheet_handle,
                    sprite_number: 0,
                }
            };

            let mut entity = world
                .create_entity()
                .with(transform)
                .with(Size::from(*size))
                .with(Background::default())
                .with(sprite_render)
                .with(ScaleOnce::default());

            if let Some(level_size) = self.level_size.as_ref() {
                entity = entity.with(Confined::new(
                    Rect::builder()
                        .top(level_size.1)
                        .right(level_size.0)
                        .build(),
                ));
            }

            // if !is_always_loaded(&properties) {
            //     entity = entity.with(Loadable::default());
            // }

            entity.build();
        }
    }
}
