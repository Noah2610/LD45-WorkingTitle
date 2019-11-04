use amethyst::ecs::Entity;

use super::state_prelude::*;

#[derive(Default)]
pub struct Ingame {
    timer_display_entities: Vec<Entity>,
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Ingame {
    fn on_start(&mut self, data: StateData<CustomGameData<CustomData>>) {
        // Start timer
        if let Some(timer) = data.world.write_resource::<TimerRes>().0.as_mut()
        {
            if timer.state.is_stopped() || timer.state.is_finished() {
                timer.start().unwrap();
            }
        }
        // Display timer
        if data.world.read_resource::<ShouldDisplayTimer>().0 {
            self.create_timer_display(data.world);
        }
    }

    fn on_stop(&mut self, data: StateData<CustomGameData<CustomData>>) {
        // Stop timer
        if let Some(timer) = data.world.write_resource::<TimerRes>().0.as_mut()
        {
            if timer.state.is_running() || timer.state.is_paused() {
                // Quit to main menu from pause menu
                if data.world.read_resource::<ToMainMenu>().0 {
                    timer.stop().unwrap();
                // Beat the level
                } else {
                    timer.finish().unwrap();
                    println!("---\nLEVEL TIME: {}\n---", timer.time_output());
                }
            }
        }
        // Delete timer display
        data.world
            .delete_entities(&self.timer_display_entities)
            .unwrap();
    }

    fn on_resume(&mut self, data: StateData<CustomGameData<CustomData>>) {
        // Resume timer
        if let Some(timer) = data.world.write_resource::<TimerRes>().0.as_mut()
        {
            if timer.state.is_paused() {
                timer.resume().unwrap();
            }
        }
    }

    fn on_pause(&mut self, data: StateData<CustomGameData<CustomData>>) {
        // Pause timer
        if let Some(timer) = data.world.write_resource::<TimerRes>().0.as_mut()
        {
            if timer.state.is_running() {
                timer.pause().unwrap();
            }
        }
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(data.world, "ingame").unwrap();

        // Handle input
        if let Some(trans) = self.handle_keys(&data.world) {
            return trans;
        }

        // Win game
        if data.world.read_resource::<WinGame>().0 {
            data.world.write_resource::<WinGame>().0 = false;
            return Trans::Switch(Box::new(Win::default()));
        }

        // To main menu (DifficultySelect)
        if data.world.read_resource::<ToMainMenu>().0 {
            return Trans::Pop;
        }

        Trans::None
    }
}

impl Ingame {
    fn handle_keys<'a, 'b>(
        &self,
        world: &World,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        let input = world.read_resource::<InputManager<IngameBindings>>();

        if input.is_down(IngameActionBinding::Quit) {
            Some(Trans::Quit)
        } else if input.is_down(IngameActionBinding::TogglePause) {
            Some(Trans::Push(Box::new(Paused::default())))
        } else {
            None
        }
    }

    fn create_timer_display(&mut self, world: &mut World) {
        use amethyst::assets::Loader;
        use amethyst::core::Parent;
        use amethyst::prelude::Builder;
        use amethyst::ui::{
            Anchor,
            FontHandle,
            TtfFormat,
            UiImage,
            UiText,
            UiTransform,
        };

        let (font_size, color, bg_color) = {
            let settings = &world.read_resource::<Settings>().timer_display;
            (settings.font_size, settings.color, settings.bg_color)
        };

        let font_handle: FontHandle = {
            let loader = world.read_resource::<Loader>();
            loader.load(
                resource("fonts/undefined-medium.ttf"),
                TtfFormat,
                (),
                &world.read_resource(),
            )
        };

        let parent_transform = UiTransform::new(
            "timer_display_container".to_string(), // id
            Anchor::TopLeft,                       // anchor
            Anchor::TopLeft,                       // pivot
            0.0,                                   // x
            0.0,                                   // y
            1.0,                                   // z
            256.0,                                 // width
            64.0,                                  // height
        )
        .into_transparent();
        let bg_color = UiImage::SolidColor(bg_color);

        let transform = UiTransform::new(
            "timer_display".to_string(), // id
            Anchor::MiddleLeft,          // anchor
            Anchor::MiddleLeft,          // pivot
            0.0,                         // x
            0.0,                         // y
            1.1,                         // z
            1.0,                         // width
            1.0,                         // height
        )
        .into_transparent()
        .into_percent();
        let text = UiText::new(
            font_handle,   // font: FontHandle,
            String::new(), // text: String,
            color,         // color
            font_size,     // font_size
        );

        let parent_entity = world
            .create_entity()
            .with(parent_transform)
            .with(bg_color)
            .build();
        let entity = world
            .create_entity()
            .with(TimerDisplay::default())
            .with(Parent {
                entity: parent_entity,
            })
            .with(transform)
            .with(text)
            .build();

        self.timer_display_entities.push(parent_entity);
        self.timer_display_entities.push(entity);
    }
}
