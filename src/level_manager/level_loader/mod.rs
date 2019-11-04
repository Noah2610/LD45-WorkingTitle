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

use amethyst::ecs::{Entities, Join, ReadStorage, World, WorldExt};
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

#[derive(PartialEq)]
pub enum BuildType {
    Backgrounds,
    Camera,
    Checkpoints,
    Enemies,
    Features,
    Goal,
    Indicators,
    Player,
    Tiles,
}

pub struct ToBuild {
    to_build: Vec<BuildType>,
}

impl ToBuild {
    pub fn none() -> Self {
        Self {
            to_build: Vec::new(),
        }
    }

    pub fn all() -> Self {
        Self {
            to_build: vec![
                BuildType::Backgrounds,
                BuildType::Camera,
                BuildType::Checkpoints,
                BuildType::Enemies,
                BuildType::Features,
                BuildType::Goal,
                BuildType::Indicators,
                BuildType::Player,
                BuildType::Tiles,
            ],
        }
    }

    pub fn with(mut self, build_type: BuildType) -> Self {
        if !self.to_build.contains(&build_type) {
            self.to_build.push(build_type);
        }
        self
    }

    pub fn without(mut self, build_type: BuildType) -> Self {
        if let Some((i, _)) = self
            .to_build
            .iter()
            .enumerate()
            .find(|(_, b)| *b == &build_type)
        {
            self.to_build.remove(i);
        }
        self
    }

    pub fn should_build(&self, build_type: BuildType) -> bool {
        self.to_build.contains(&build_type)
    }
}

impl Default for ToBuild {
    fn default() -> Self {
        Self::all()
    }
}

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
    pub to_build:     ToBuild,
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

        self.level_size = Default::default();
        self.player_data = Default::default();
        self.tiles_data = Default::default();
        self.enemies_data = Default::default();
        self.features_data = Default::default();
        self.backgrounds_data = Default::default();
        self.checkpoints_data = Default::default();
        self.goal_data = Default::default();
        self.indicators_data = Default::default();

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
        self.finished_loading = false;

        if self.to_build.should_build(BuildType::Player) {
            self.build_player(world);
        }
        if self.to_build.should_build(BuildType::Camera) {
            self.build_camera(world);
        }
        if self.to_build.should_build(BuildType::Tiles) {
            self.build_tiles(world);
        }
        if self.to_build.should_build(BuildType::Enemies) {
            self.build_enemies(world);
        }
        if self.to_build.should_build(BuildType::Features) {
            self.build_features(world);
        }
        if self.to_build.should_build(BuildType::Indicators) {
            self.build_indicators(world);
        }
        if self.to_build.should_build(BuildType::Backgrounds) {
            self.build_backgrounds(world);
        }
        if self.to_build.should_build(BuildType::Checkpoints) {
            self.build_checkpoints(world);
        }
        if self.to_build.should_build(BuildType::Goal) {
            self.build_goal(world);
        }

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

                if let Some(pushable_entity_data) = match obj_type {
                    "Player" => {
                        self.player_data = Some(EntityData {
                            pos,
                            size,
                            sprite: None,
                            properties: properties.clone(),
                        });
                        None
                    }
                    "Feature" => Some(&mut self.features_data),
                    "Enemy" => Some(&mut self.enemies_data),
                    "Background" => Some(&mut self.backgrounds_data),
                    "Checkpoint" => Some(&mut self.checkpoints_data),
                    "Goal" => {
                        self.goal_data = Some(EntityData {
                            pos,
                            size,
                            sprite: None,
                            properties: properties.clone(),
                        });
                        None
                    }
                    "Indicator" => Some(&mut self.indicators_data),
                    _ => {
                        eprintln!("WARNING: Unknown object type: {}", obj_type);
                        None
                    }
                } {
                    pushable_entity_data.push(EntityData {
                        pos,
                        size,
                        sprite: None,
                        properties: properties.clone(),
                    });
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

fn is_solid(properties: &JsonValue) -> bool {
    properties["solid"].as_bool().unwrap_or(false)
}

fn is_always_loaded(properties: &JsonValue) -> bool {
    properties["always_loaded"].as_bool().unwrap_or(false)
}

fn is_loader(properties: &JsonValue) -> bool {
    properties["loader"].as_bool().unwrap_or(false)
}

fn is_spike(properties: &JsonValue) -> bool {
    properties["spike"].as_bool().unwrap_or(false)
}
