use super::state_prelude::*;

#[derive(Default)]
pub struct Startup;

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Startup {
    fn on_start(&mut self, data: StateData<CustomGameData<CustomData>>) {
        insert_resources(data.world);
        initialize_music(data.world);
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(data.world, "startup").unwrap();
        Trans::Push(Box::new(DifficultySelect::default()))
    }
}
