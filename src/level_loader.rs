use amethyst::ecs::{World, WorldExt};
use amethyst::prelude::Builder;
use amethyst::renderer::sprite::SpriteSheetHandle;
use deathframe::handles::SpriteSheetHandles;
use json::JsonValue;

use crate::components::prelude::*;
use crate::helpers::*;
use crate::settings::prelude::*;
use crate::solid_tag::SolidTag;

const LEVELS_DIR: &str = "levels";
const TILE_SIZE: (f32, f32) = (32.0, 32.0);
const TILE_Z: f32 = 0.0;
const PROPERTY_Z_KEY: &str = "z";
const CAMERA_Z: f32 = 10.0;
const PLAYER_Z: f32 = 1.0;
const PLAYER_SPRITESHEET_FILENAME: &str = "player.png";

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
    player_data:      Option<EntityData>,
    tiles_data:       Vec<EntityData>,
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

        self.load_objects(&json["objects"]);
        self.load_tiles(&json["tiles"]);
    }

    pub fn build(&mut self, world: &mut World) {
        self.build_player(world);
        self.build_camera(world);
        self.build_tiles(world);

        self.finished_loading = true;
    }

    pub fn is_finished(&self) -> bool {
        self.finished_loading
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
            size,
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
            // let size = Size::from(*size);
            let size =
                Size::new(player_settings.size.0, player_settings.size.1);

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

            world
                .create_entity()
                .with(Player::from(player_settings.clone()))
                .with(transform)
                .with(size)
                .with(Velocity::default())
                .with(sprite_render)
                .with(Transparent)
                .with(DecreaseVelocity::from(player_settings.decr_velocity))
                .with(ScaleOnce)
                .build();
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

            world
                .create_entity()
                .with(transform)
                .with(AmethystCamera::standard_2d(
                    camera_settings.size.0,
                    camera_settings.size.1,
                ))
                .build();
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
                .with(ScaleOnce)
                .with(Transparent);

            if let Some(sprite_render) = sprite_render_opt {
                entity = entity.with(sprite_render);
            }

            if let Some(animation) = animation_opt {
                entity = entity.with(animation);
            }

            // TODO
            // if let Some(is_solid) = properties["solid"].as_bool() {
            //     if is_solid {
            //         entity = entity
            //             .with(Solid::new(SolidTag::default()))
            //             .with(Collision::new());
            //     }
            // }

            // TODO
            // if let Some(harmful_damage) = properties["harmful"].as_u32() {
            //     entity = entity
            //         .with(Collision::new())
            //         .with(Harmful::with_damage(harmful_damage));
            // }

            entity.build();
        }
    }
}

/// Generate an Animation from the given properties.
/// Copied from LD44.
pub fn animation_from(
    spritesheet_handle: SpriteSheetHandle,
    properties: &JsonValue,
) -> Option<Animation> {
    match (
        properties["animation_sprite_ids"].as_str(),
        properties["animation_delays_ms"].as_str(),
    ) {
        (Some(str_sprite_ids), Some(str_delays_ms)) => {
            let sprite_ids = str_sprite_ids
                .split(",")
                .map(|str_id| {
                    str_id.trim().parse::<usize>().expect(&format!(
                        "Couldn't parse string to usize '{}' in '{}' \
                         (animation_sprite_ids)",
                        str_id, str_sprite_ids
                    ))
                })
                .collect();
            let delays_ms = str_delays_ms
                .split(",")
                .map(|str_ms| {
                    str_ms.trim().parse::<u64>().expect(&format!(
                        "Couldn't parse string to u64 '{}' in '{}' \
                         (animation_delays_ms)",
                        str_ms, str_delays_ms
                    ))
                })
                .collect();
            Some(
                Animation::new()
                    .default_sprite_sheet_handle(spritesheet_handle)
                    .sprite_ids(sprite_ids)
                    .delays_ms(delays_ms)
                    .build(),
            )
        }
        (Some(_), None) | (None, Some(_)) => panic!(
            "Tile with animation needs both properties `animation_sprite_ids` \
             and `animation_delays_ms`"
        ),
        (None, None) => None,
    }
}
