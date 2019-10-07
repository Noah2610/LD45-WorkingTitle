use super::state_prelude::*;

#[derive(Default)]
pub struct Win;

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Win {
    fn update(
        &mut self,
        data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(data.world, "win").unwrap();
        Trans::None
    }
}
