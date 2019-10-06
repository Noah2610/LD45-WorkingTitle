mod helpers;

use amethyst::ecs::{World, WorldExt};
use amethyst::prelude::Builder;
use deathframe::handles::SpriteSheetHandles;
use json::JsonValue;

use crate::components::prelude::*;
use crate::helpers::*;
use crate::settings::prelude::*;
use crate::solid_tag::SolidTag;
use helpers::*;

const LEVELS_DIR: &str = "levels";
const TILE_SIZE: (f32, f32) = (16.0, 16.0);
const TILE_Z: f32 = 0.0;
const PROPERTY_Z_KEY: &str = "z";
const CAMERA_Z: f32 = 10.0;
const PLAYER_Z: f32 = 2.0;
const PLAYER_SPRITESHEET_FILENAME: &str = "player.png";
const ENEMY_Z: f32 = 1.0;

struct EntityData {
    pub pos:        Vector,
    pub size:       Vector,
    pub properties: JsonValue,
    pub sprite:     Option<SpriteData>,
}

struct SpriteData {
    pub spritesheet_path: String,
    pub sprite_id:        usize,
}

#[derive(Default)]
pub struct LevelLoader {
    finished_loading: bool,
    level_size:       Option<Vector>,
    player_data:      Option<EntityData>,
    tiles_data:       Vec<EntityData>,
    enemies_data:     Vec<EntityData>,
    features_data:    Vec<EntityData>,
}

impl LevelLoader {
    pub fn load<S>(&mut self, filename: S)
    where
        S: ToString,
    {
        use std::fs::File;
        use std::io::Read;

        self.finished_loading = false;

        let path = resource(format!("{}/{}", LEVELS_DIR, filename.to_string()));
        let mut file = File::open(&path)
            .expect(&format!("Couldn't open level file {}", &path));
        let mut raw = String::new();
        file.read_to_string(&mut raw)
            .expect("Couldn't read file contents to string");
        let json = json::parse(&raw).expect("Couldn't parse level JSON file");

        self.load_level_data(&json["level"]);
        self.load_objects(&json["objects"]);
        self.load_tiles(&json["tiles"]);
    }

    pub fn build(&mut self, world: &mut World) {
        self.build_player(world);
        self.build_camera(world);
        self.build_tiles(world);
        self.build_enemies(world);
        self.build_features(world);

        self.finished_loading = true;
    }

    pub fn is_finished(&self) -> bool {
        self.finished_loading
    }

    fn load_level_data(&mut self, json: &JsonValue) {
        const ERRMSG: &str = "\"level\".\"size\" values should be f32";
        for (key, val) in json.entries() {
            match key {
                "size" => {
                    self.level_size = Some(Vector::new(
                        val["w"].as_f32().expect(ERRMSG),
                        val["h"].as_f32().expect(ERRMSG),
                    ))
                }
                _ => (),
            }
        }
    }

    fn load_objects(&mut self, json: &JsonValue) {
        for object_data in json.members() {
            if let (
                Some(obj_type),
                (Some(x), Some(y)),
                (Some(w), Some(h)),
                properties,
            ) = (
                object_data["type"].as_str(),
                (
                    object_data["pos"]["x"].as_f32(),
                    object_data["pos"]["y"].as_f32(),
                ),
                (
                    object_data["size"]["w"].as_f32(),
                    object_data["size"]["h"].as_f32(),
                ),
                &object_data["properties"],
            ) {
                let size = Vector::new(w, h);
                let pos = Vector::new(x + size.0 * 0.5, y - size.1 * 0.5);

                match obj_type {
                    "Player" => {
                        self.player_data = Some(EntityData {
                            pos,
                            size,
                            sprite: None,
                            properties: properties.clone(),
                        });
                    }
                    "Feature" => {
                        self.features_data.push(EntityData {
                            pos,
                            size,
                            sprite: None,
                            properties: properties.clone(),
                        });
                    }
                    "Enemy" => self.enemies_data.push(EntityData {
                        pos,
                        size,
                        sprite: None,
                        properties: properties.clone(),
                    }),
                    _ => {
                        eprintln!("Unknown object type: {}", obj_type);
                    }
                }
            }
        }
    }

    fn load_tiles(&mut self, json: &JsonValue) {
        for tile_data in json.members() {
            if let (
                Some(id),
                (Some(x), Some(y)),
                properties,
                Some(tileset_name),
            ) = (
                tile_data["id"].as_usize(),
                (
                    tile_data["pos"]["x"].as_f32(),
                    tile_data["pos"]["y"].as_f32(),
                ),
                &tile_data["properties"],
                tile_data["ts"].as_str(),
            ) {
                let spritesheet_path =
                    resource(format!("spritesheets/{}.png", tileset_name));
                let size = Vector::from(TILE_SIZE);
                let pos = Vector::new(x + size.0 * 0.5, y - size.1 * 0.5);

                self.tiles_data.push(EntityData {
                    pos,
                    size,
                    properties: properties.clone(),
                    sprite: Some(SpriteData {
                        spritesheet_path,
                        sprite_id: id,
                    }),
                });
            }
        }
    }

    fn build_player(&self, world: &mut World) {
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
                ));

            if let Some(level_size) = self.level_size.as_ref() {
                entity = entity.with(Confined::new(
                    Rect::new().top(level_size.1).right(level_size.0).build(),
                ))
            }

            entity.build();
        } else {
            panic!("No player object in level");
        }
    }

    fn build_camera(&self, world: &mut World) {
        if let Some(player_data) = self.player_data.as_ref() {
            let player_pos = player_data.pos;

            let camera_settings =
                world.read_resource::<Settings>().camera.clone();

            let mut transform = Transform::default();
            transform.set_translation_xyz(player_pos.0, player_pos.1, CAMERA_Z);

            let size = Size::from(camera_settings.size);

            let mut entity = world
                .create_entity()
                .with(transform)
                .with(AmethystCamera::standard_2d(size.w, size.h))
                .with(size);

            if let Some(level_size) = self.level_size.as_ref() {
                entity = entity.with(Confined::new(
                    Rect::new().top(level_size.1).right(level_size.0).build(),
                ));
            }

            entity.build();
        }
    }

    fn build_tiles(&self, world: &mut World) {
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
                .with(transform)
                .with(Size::from(*size))
                .with(ScaleOnce::default())
                .with(Transparent);

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

            entity.build();
        }
    }

    fn build_enemies(&self, world: &mut World) {
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
            transform.set_translation_xyz(pos.0, pos.1, ENEMY_Z);

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
                .with(Solid::new(SolidTag::Enemy))
                .with(Collision::default())
                .with(sprite_render)
                .with(animations_container_from_file(
                    animations_path,
                    spritesheet_handle,
                ))
                .with(Spike::default());

            if let Some(gravity) = enemy_settings.gravity {
                entity = entity.with(Gravity::new(gravity.0, gravity.1));
            }

            entity.build();
        }
    }

    fn build_features(&self, world: &mut World) {
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
