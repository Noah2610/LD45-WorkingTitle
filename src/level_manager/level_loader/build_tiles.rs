use deathframe::components::animations_container::AnimationsContainerBuilder;
use deathframe::components::Animation;

use super::*;

impl LevelLoader {
    pub(super) fn build_tiles(&self, world: &mut World) {
        // Delete existing entities
        world.exec(|(entities, tiles): (Entities, ReadStorage<Tile>)| {
            for (entity, _) in (&entities, &tiles).join() {
                entities.delete(entity).unwrap();
            }
        });

        for EntityData {
            pos,
            size,
            properties,
            sprite,
        } in &self.tiles_data
        {
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                pos.0,
                pos.1,
                properties[PROPERTY_Z_KEY].as_f32().unwrap_or(TILE_Z),
            );

            let (sprite_render_opt, animation_opt, animations_container_opt) =
                if let Some(sprite_data) = sprite {
                    let spritesheet_handle = world
                        .write_resource::<SpriteSheetHandles>()
                        .get_or_load(&sprite_data.spritesheet_path, &world);
                    (
                        Some(SpriteRender {
                            sprite_sheet:  spritesheet_handle.clone(),
                            sprite_number: sprite_data.sprite_id,
                        }),
                        animation_from(spritesheet_handle.clone(), &properties),
                        properties["animation_config"].as_str().map(|f| {
                            animations_container_from_file(
                                resource(&format!("animations/{}", f)),
                                spritesheet_handle,
                            )
                        }),
                    )
                } else {
                    (None, None, None)
                };

            let mut entity = world
                .create_entity()
                .with(Tile::default())
                .with(transform)
                .with(Size::from(*size))
                .with(ScaleOnce::default())
                .with(Transparent)
                .with(Loadable::default());

            if let Some(sprite_render) = sprite_render_opt {
                entity = entity.with(sprite_render.clone());

                if let Some(animations_container) = animations_container_opt {
                    let animations_container =
                        AnimationsContainerBuilder::from(animations_container)
                            .insert(
                                "default",
                                Animation::new()
                                    .default_delay_ms(1000)
                                    .sprite_renders(vec![sprite_render])
                                    .build(),
                            )
                            .current("default")
                            .build();
                    entity = entity
                        .with(animations_container)
                        .with(DynamicAnimation::default())
                        .with(Collision::default());
                } else if let Some(animation) = animation_opt {
                    entity = entity.with(animation);
                }
            }

            if is_solid(&properties) {
                entity = entity
                    .with(Solid::new(SolidTag::Tile))
                    .with(Collision::default());
            }

            if is_spike(&properties) {
                entity =
                    entity.with(Collision::default()).with(Spike::default());
            }

            if !is_always_loaded(&properties) {
                entity = entity.with(Loadable::default());
            }

            entity.build();
        }
    }
}
