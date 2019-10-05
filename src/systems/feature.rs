use super::system_prelude::*;
use crate::solid_tag::SolidTag;

#[derive(Default)]
pub struct FeatureSystem;

impl<'a> System<'a> for FeatureSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Collision>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Feature>,
        WriteStorage<'a, Solid<SolidTag>>,
    );

    fn run(
        &mut self,
        (entities, collisions, mut players, mut features, mut solids): Self::SystemData,
    ) {
        if let Some((player, player_collision, player_solid)) =
            (&mut players, &collisions, &mut solids).join().next()
        {
            for (feature_entity, feature) in (&entities, &mut features).join() {
                if player_collision.in_collision_with(feature_entity.id()) {
                    match &feature.feature_type {
                        FeatureType::AddCollisions => {
                            player_solid.tag = SolidTag::PlayerWithCollision;
                        }
                    }
                }
            }
        }
    }
}
