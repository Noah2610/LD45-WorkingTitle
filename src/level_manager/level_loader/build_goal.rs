use super::*;

impl LevelLoader {
    pub(super) fn build_goal(&self, world: &mut World) {
        // Delete existing entities
        world.exec(|(entities, goals): (Entities, ReadStorage<Goal>)| {
            for (entity, _) in (&entities, &goals).join() {
                entities.delete(entity).unwrap();
            }
        });

        if let Some(EntityData {
            pos,
            size,
            sprite: _,
            properties: _,
        }) = self.goal_data.as_ref()
        {
            let mut transform = Transform::default();
            transform.set_translation_xyz(pos.0, pos.1, 0.0);

            world
                .create_entity()
                .with(transform)
                .with(Size::from(*size))
                .with(Goal::default())
                .with(Collision::default())
                .build();
        }
    }
}
