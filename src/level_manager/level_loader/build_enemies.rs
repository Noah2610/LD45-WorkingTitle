use super::*;

impl LevelLoader {
    pub(super) fn build_enemies(&self, world: &mut World) {
        // Delete existing entities
        world.exec(|(entities, enemies): (Entities, ReadStorage<Enemy>)| {
            for (entity, _) in (&entities, &enemies).join() {
                entities.delete(entity).unwrap();
            }
        });

        let enemies_settings =
            world.read_resource::<Settings>().enemies.clone();

        for EntityData {
            pos,
            size: _,
            sprite: _,
            properties,
        } in &self.enemies_data
        {
            let enemy_type = EnemyType::from(
                properties["enemy_type"]
                    .as_str()
                    .expect("Enemy has to have 'enemy_type' property"),
            );

            let pace_distance = {
                (
                    if let Some(x) = properties["pace_distance_x"].as_f32() {
                        Some(x)
                    } else {
                        None
                    },
                    if let Some(y) = properties["pace_distance_y"].as_f32() {
                        Some(y)
                    } else {
                        None
                    },
                )
            };

            let enemy_ai = match enemy_type {
                EnemyType::Ground => EnemyAi::Pacer(
                    enemy_ai_data::PacerData::new(pos.clone(), pace_distance),
                ),
                EnemyType::Flying => EnemyAi::Pacer(
                    enemy_ai_data::PacerData::new(pos.clone(), pace_distance),
                ),
            };

            let enemy_settings = enemy_type.settings(&enemies_settings);

            let mut transform = Transform::default();
            transform.set_translation_xyz(
                pos.0,
                pos.1,
                properties[PROPERTY_Z_KEY].as_f32().unwrap_or(ENEMY_Z),
            );

            let spritesheet_path = enemy_type.spritesheet_path();
            let animations_path = enemy_type.animations_config_path();

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
                .with(transform)
                .with(Velocity::default())
                .with(Size::from(enemy_settings.size))
                .with(Enemy::new(enemy_type, &enemy_settings))
                .with(enemy_ai)
                .with(Collision::default())
                .with(sprite_render)
                .with(ScaleOnce)
                .with(animations_container_from_file(
                    animations_path,
                    spritesheet_handle,
                ))
                .with(Spike::default())
                .with(DynamicAnimationTrigger::default());

            if is_solid(&properties) {
                entity = entity.with(Solid::new(SolidTag::Enemy));
            }

            if let Some(gravity) = enemy_settings.gravity {
                entity = entity.with(Gravity::new(gravity.0, gravity.1));
            }

            if !is_always_loaded(&properties) {
                entity = entity.with(Loadable::default()).with(Hidden);
            }

            if is_loader(&properties) {
                entity = entity.with(Loader::new(
                    (enemy_settings.size.0 * 2.0, enemy_settings.size.1 * 2.0)
                        .into(),
                ));
            }

            entity.build();
        }
    }
}
