use super::*;

impl LevelLoader {
    pub(super) fn build_checkpoints(&self, world: &mut World) {
        for EntityData {
            pos,
            size,
            sprite: _,
            properties: _,
        } in &self.checkpoints_data
        {
            let mut transform = Transform::default();
            transform.set_translation_xyz(pos.0, pos.1, 0.0);

            world
                .create_entity()
                .with(transform)
                .with(Size::from(*size))
                .with(Checkpoint::default())
                .with(Collision::default())
                .build();
        }
    }
}
