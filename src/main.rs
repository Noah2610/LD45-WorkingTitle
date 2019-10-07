extern crate amethyst;
extern crate deathframe;
extern crate json;
extern crate ron;
#[macro_use]
extern crate serde;

mod audio;
mod components;
mod helpers;
mod input;
mod level_loader;
mod settings;
mod solid_tag;
mod states;
mod systems;

use amethyst::audio::{AudioBundle, DjSystemDesc};
use amethyst::config::Config;
use amethyst::core::frame_limiter::FrameRateLimitConfig;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::{RenderFlat2D, RenderToWindow, RenderingBundle};
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::{ApplicationBuilder, LogLevelFilter, LoggerConfig};
use deathframe::custom_game_data::prelude::*;

fn main() -> Result<(), String> {
    init_game().map_err(|e| e.to_string())
}

fn init_game() -> amethyst::Result<()> {
    use helpers::resource;

    start_logger();

    let game_data = build_game_data()?;

    let mut game: amethyst::CoreApplication<CustomGameData<CustomData>> =
        ApplicationBuilder::new("./", states::Startup::default())?
            .with_frame_limit_config(FrameRateLimitConfig::load(resource(
                "config/frame_limiter.ron",
            )))
            .build(game_data)?;
    game.run();

    Ok(())
}

fn start_logger() {
    amethyst::start_logger(LoggerConfig {
        level_filter: LogLevelFilter::Error,
        ..Default::default()
    });
}

fn build_game_data<'a, 'b>(
) -> amethyst::Result<CustomGameDataBuilder<'a, 'b, CustomData>> {
    use audio::prelude::*;
    use deathframe::systems::InputManagerSystem;
    use helpers::resource;
    use systems::prelude::*;

    let display_config_file = resource("config/display.ron");

    // Bundles
    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_file)
                .with_clear([0.8, 0.8, 0.8, 1.0]),
        )
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderUi::default());
    let transform_bundle = TransformBundle::new();
    let input_bundle = input::input_bundle()
        .with_bindings_from_file(resource("config/bindings.ron"))
        .unwrap();
    let ui_bundle = UiBundle::<input::Bindings>::new();
    let audio_bundle = AudioBundle::default();

    let custom_game_data =
        CustomGameDataBuilder::<'a, 'b, CustomData>::default()
            .custom(CustomData::default())
            .dispatcher("startup")?
            .dispatcher("ingame")?
            .with_core_bundle(rendering_bundle)?
            .with_core_bundle(transform_bundle)?
            .with_core_bundle(input_bundle)?
            .with_core_bundle(ui_bundle)?
            .with_core_bundle(audio_bundle)?
            .with_core_desc(
                DjSystemDesc::new(|music: &mut Music| music.current()),
                "dj_system",
                &[],
            )?
            .with_core(
                InputManagerSystem::<input::Bindings>::default(),
                "input_manager_system",
                &[],
            )?
            .with(
                "ingame",
                PlayerRunSystem::default(),
                "player_run_system",
                &[],
            )?
            .with(
                "ingame",
                ControlPlayerSystem::default(),
                "control_player_system",
                &["player_run_system"],
            )?
            .with("ingame", GravitySystem::default(), "gravity_system", &[
                "control_player_system",
            ])?
            .with("ingame", EnemyAiSystem::default(), "enemy_ai_system", &[])?
            .with(
                "ingame",
                MoveEntitiesSystem::<solid_tag::SolidTag>::default(),
                "move_entities_system",
                &["control_player_system", "gravity_system", "enemy_ai_system"],
            )?
            .with(
                "ingame",
                HandleSolidCollisionsSystem::default(),
                "handle_solid_collisions_system",
                &["control_player_system", "move_entities_system"],
            )?
            .with(
                "ingame",
                DecreaseVelocitiesSystem::default(),
                "decrease_velocities_system",
                &["move_entities_system"],
            )?
            .with("ingame", FollowSystem::default(), "follow_system", &[
                "control_player_system",
                "move_entities_system",
            ])?
            .with(
                "ingame",
                ConfineEntitiesSystem::default(),
                "confine_entities_system",
                &["follow_system"],
            )?
            .with("ingame", CollisionSystem::default(), "collision_system", &[
            ])?
            .with("ingame", FeatureSystem::default(), "feature_system", &[
                "collision_system",
            ])?
            .with("ingame", AnimationSystem::default(), "animation_system", &[
                "feature_system",
            ])?
            .with(
                "ingame",
                ScaleSpritesSystem::default(),
                "scale_sprites_system",
                &["animation_system"],
            )?
            .with(
                "ingame",
                KillEnemySystem::default(),
                "kill_enemy_system",
                &[
                    "collision_system",
                    "control_player_system",
                    "gravity_system",
                ],
            )?
            .with("ingame", SpikeSystem::default(), "spike_system", &[
                "collision_system",
                "kill_enemy_system",
            ])?
            .with(
                "ingame",
                DeathFloorSystem::default(),
                "death_floor_system",
                &["move_entities_system"],
            )?
            .with(
                "ingame",
                BackgroundSystem::default(),
                "background_system",
                &["follow_system"],
            )?
            .with("ingame", LoadingSystem::default(), "loading_system", &[
                "move_entities_system",
                "confine_entities_system",
            ])?;

    Ok(custom_game_data)
}

#[derive(Default)]
pub struct CustomData;
