use amethyst::ui::Anchor;

use super::*;

impl LevelLoader {
    pub(super) fn build_checkpoints(&self, world: &mut World) {
        // Delete existing entities
        world.exec(
            |(entities, checkpoints): (Entities, ReadStorage<Checkpoint>)| {
                for (entity, _) in (&entities, &checkpoints).join() {
                    entities.delete(entity).unwrap();
                }
            },
        );

        for (
            id,
            EntityData {
                pos,
                size,
                sprite: _,
                properties,
            },
        ) in self.checkpoints_data.iter().enumerate()
        {
            let mut transform = Transform::default();
            transform.set_translation_xyz(pos.0, pos.1, 0.0);

            let anchor = if let Some(anchor_str) =
                properties["respawn_anchor"].as_str()
            {
                serde_plain::from_str(anchor_str).expect(&format!(
                    "Checkpoint's respawn_anchor property '{}' is not valid",
                    anchor_str
                ))
            } else {
                Anchor::Middle
            };

            world
                .create_entity()
                .with(transform)
                .with(Size::from(*size))
                .with(Checkpoint::new(id, anchor))
                .with(Collision::default())
                .build();
        }
    }
}
