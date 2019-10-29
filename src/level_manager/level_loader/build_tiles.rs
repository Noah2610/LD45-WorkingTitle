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

            let (sprite_render_opt, animation_opt) =
                if let Some(sprite_data) = sprite {
                    let (sprite_render, animation_opt) = {
                        let spritesheet_handle = world
                            .write_resource::<SpriteSheetHandles>()
                            .get_or_load(&sprite_data.spritesheet_path, &world);
                        (
                            SpriteRender {
                                sprite_sheet:  spritesheet_handle.clone(),
                                sprite_number: sprite_data.sprite_id,
                            },
                            animation_from(spritesheet_handle, &properties),
                        )
                    };
                    (Some(sprite_render), animation_opt)
                } else {
                    (None, None)
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
                entity = entity.with(sprite_render);
            }

            if let Some(animation) = animation_opt {
                entity = entity.with(animation);
            }

            if let Some(is_solid) = properties["solid"].as_bool() {
                if is_solid {
                    entity = entity
                        .with(Solid::new(SolidTag::Tile))
                        .with(Collision::default());
                }
            }

            if let Some(is_spike) = properties["spike"].as_bool() {
                if is_spike {
                    entity = entity
                        .with(Collision::default())
                        .with(Spike::default());
                }
            }

            if !is_always_loaded(&properties) {
                entity = entity.with(Loadable::default());
            }

            entity.build();
        }
    }
}
