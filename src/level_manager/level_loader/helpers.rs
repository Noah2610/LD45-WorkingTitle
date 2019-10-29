//! A lot of this module is copied from LD44.

use amethyst::renderer::sprite::SpriteSheetHandle;
use json::JsonValue;

use crate::components::prelude::*;

/// Generate an Animation from the given properties.
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

/// Generate a AnimationsContainer from the the given animations ron file.
pub fn animations_container_from_file<T>(
    file: T,
    spritesheet_handle: SpriteSheetHandle,
) -> AnimationsContainer
where
    T: ToString,
{
    let animations_container_config = load_animations_container_config(file);
    let mut animations_container = AnimationsContainer::new();

    for animation_config in animations_container_config.animations {
        let mut animation = Animation::new()
            .default_sprite_sheet_handle(spritesheet_handle.clone());
        if let Some(default_delay_ms) = animation_config.default_delay_ms {
            animation = animation.default_delay_ms(default_delay_ms);
        }
        if let Some(delays_ms) = animation_config.delays_ms {
            animation = animation.delays_ms(delays_ms);
        }
        animation = animation.sprite_ids(animation_config.sprite_ids);

        animations_container = animations_container
            .insert(animation_config.name, animation.build());
    }

    if let Some(current) = animations_container_config.current {
        animations_container = animations_container.current(current);
    }

    animations_container.build()
}

#[derive(Deserialize)]
struct AnimationConfig {
    pub name:             String,
    pub sprite_ids:       Vec<usize>,
    pub delays_ms:        Option<Vec<u64>>,
    pub default_delay_ms: Option<u64>,
}

#[derive(Deserialize)]
struct AnimationsContainerConfig {
    pub animations: Vec<AnimationConfig>,
    pub current:    Option<String>,
}

fn load_animations_container_config<T>(filepath: T) -> AnimationsContainerConfig
where
    T: ToString,
{
    use std::fs::File;

    let file = File::open(filepath.to_string())
        .expect("Couldn't open animations file");
    ron::de::from_reader(file).expect("Failed parsing animations file")
}
