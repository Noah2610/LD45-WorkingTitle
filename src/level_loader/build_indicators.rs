use super::*;

impl LevelLoader {
    pub(super) fn build_indicators(&self, world: &mut World) {
        for EntityData {
            pos,
            size,
            sprite: _,
            properties,
        } in &self.indicators_data
        {
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                pos.0,
                pos.1,
                properties[PROPERTY_Z_KEY].as_f32().unwrap_or(INDICATOR_Z),
            );

            let image_name = properties["image"]
                .as_str()
                .expect("Indicator object has to have an 'image' property");
            let feature_trigger = FeatureType::from(
                properties["feature_trigger"].as_str().expect(
                    "Indicator object has to have a 'feature_trigger' property",
                ),
            );

            let spritesheet_path =
                resource(format!("{}/{}", INDICATORS_DIR, image_name));
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
                .with(Indicator::new(feature_trigger))
                .with(sprite_render)
                .with(ScaleOnce::default());

            if !is_always_loaded(&properties) {
                entity = entity.with(Loadable::default());
            }

            entity.build();
        }
    }
}
