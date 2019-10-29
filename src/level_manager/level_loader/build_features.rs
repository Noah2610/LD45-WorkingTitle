use super::*;

impl LevelLoader {
    pub(super) fn build_features(&self, world: &mut World) {
        // Delete existing entities
        world.exec(|(entities, features): (Entities, ReadStorage<Feature>)| {
            for (entity, _) in (&entities, &features).join() {
                entities.delete(entity).unwrap();
            }
        });

        for EntityData {
            pos,
            size,
            sprite: _,
            properties,
        } in &self.features_data
        {
            let feature_type = properties["feature_type"]
                .as_str()
                .expect("Feature has to have 'feature_type' property");
            let feature = FeatureType::from(feature_type);

            let mut transform = Transform::default();
            transform.set_translation_xyz(pos.0, pos.1, 0.0);

            world
                .create_entity()
                .with(transform)
                .with(Size::from(*size))
                .with(Feature::new(feature))
                .with(Collision::default())
                .build();
        }
    }
}
