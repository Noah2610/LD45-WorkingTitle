mod build_backgrounds;
mod build_camera;
mod build_checkpoints;
mod build_enemies;
mod build_features;
mod build_goal;
mod build_indicators;
mod build_player;
mod build_tiles;
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

const PROPERTY_Z_KEY: &str = "z";
const BACKGROUND_Z: f32 = -1.0;
const CAMERA_Z: f32 = 10.0;
const ENEMY_Z: f32 = 1.0;
const PLAYER_Z: f32 = 2.0;
const TILE_Z: f32 = 0.0;
const INDICATOR_Z: f32 = 0.5;
const LEVELS_DIR: &str = "levels";
const TILE_SIZE: (f32, f32) = (16.0, 16.0);
const PLAYER_SPRITESHEET_FILENAME: &str = "player.png";
const BACKGROUNDS_DIR: &str = "spritesheets/bg";
const INDICATORS_DIR: &str = "spritesheets/indicators";

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
    backgrounds_data: Vec<EntityData>,
    checkpoints_data: Vec<EntityData>,
    goal_data:        Option<EntityData>,
    indicators_data:  Vec<EntityData>,
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
        self.build_indicators(world);
        self.build_backgrounds(world);
        self.build_checkpoints(world);
        self.build_goal(world);

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
                        })
                    }
                    "Feature" => self.features_data.push(EntityData {
                        pos,
                        size,
                        sprite: None,
                        properties: properties.clone(),
                    }),
                    "Enemy" => self.enemies_data.push(EntityData {
                        pos,
                        size,
                        sprite: None,
                        properties: properties.clone(),
                    }),
                    "Background" => self.backgrounds_data.push(EntityData {
                        pos,
                        size,
                        sprite: None,
                        properties: properties.clone(),
                    }),
                    "Checkpoint" => self.checkpoints_data.push(EntityData {
                        pos,
                        size,
                        sprite: None,
                        properties: properties.clone(),
                    }),
                    "Goal" => {
                        self.goal_data = Some(EntityData {
                            pos,
                            size,
                            sprite: None,
                            properties: properties.clone(),
                        })
                    }
                    "Indicator" => self.indicators_data.push(EntityData {
                        pos,
                        size,
                        sprite: None,
                        properties: properties.clone(),
                    }),
                    _ => {
                        eprintln!("WARNING: Unknown object type: {}", obj_type);
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
}

fn is_always_loaded(properties: &JsonValue) -> bool {
    properties["always_loaded"].as_bool().unwrap_or(false)
}
