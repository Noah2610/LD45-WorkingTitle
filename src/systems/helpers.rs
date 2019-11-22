use std::ops::Deref;

use amethyst::ecs::storage::{MaskedStorage, Storage};
use deathframe::components::solid::SolidTag as _;

use super::system_prelude::*;
use crate::level_manager::Level;
use crate::solid_tag::SolidTag;

// Copied from LD44
#[derive(Default, Clone, Debug)]
pub struct SidesTouching {
    pub is_touching_top:    bool,
    pub is_touching_bottom: bool,
    pub is_touching_left:   bool,
    pub is_touching_right:  bool,
}

impl SidesTouching {
    pub fn new<D>(
        entities: &Entities,
        target_collision: &Collision,
        target_solid: &Solid<SolidTag>,
        collisions: &Storage<Collision, D>,
        solids: &ReadStorage<Solid<SolidTag>>,
    ) -> Self
    where
        D: Deref<Target = MaskedStorage<Collision>>,
    {
        let mut is_touching_top = false;
        let mut is_touching_bottom = false;
        let mut is_touching_left = false;
        let mut is_touching_right = false;
        if target_collision.in_collision() {
            for (other_entity, _, other_solid) in
                (entities, collisions, solids).join()
            {
                if target_solid.tag.collides_with(&other_solid.tag) {
                    if let Some(colliding_with) =
                        target_collision.collision_with(other_entity.id())
                    {
                        match colliding_with.side {
                            Side::Top => is_touching_top = true,
                            Side::Bottom => is_touching_bottom = true,
                            Side::Left => is_touching_left = true,
                            Side::Right => is_touching_right = true,
                            _ => (),
                        }
                        if is_touching_top
                            && is_touching_bottom
                            && is_touching_left
                            && is_touching_right
                        {
                            break;
                        }
                    }
                }
            }
        }
        Self {
            is_touching_top,
            is_touching_bottom,
            is_touching_left,
            is_touching_right,
        }
    }

    pub fn is_touching_horizontally(&self) -> bool {
        self.is_touching_left || self.is_touching_right
    }

    pub fn is_touching_vertically(&self) -> bool {
        self.is_touching_top || self.is_touching_bottom
    }
}

pub fn is_level_locked(world: &World, level: &Level) -> bool {
    let mut level_locked = false;
    let level_manager_settings =
        &world.read_resource::<Settings>().level_manager;
    let level_settings = level_manager_settings.level(level);
    if level_settings.initially_locked {
        level_locked = true;
        if let Some(savefile_data) = &world.read_resource::<SavefileDataRes>().0
        {
            if let Some(unlocked_by_any) =
                level_settings.unlocked_by_any.as_ref()
            {
                level_locked =
                    !unlocked_by_any.iter().any(|unlocked_by_level| {
                        if let Some(level_save) =
                            savefile_data.level(unlocked_by_level)
                        {
                            level_save.won
                        } else {
                            false
                        }
                    });
            }
        }
    }
    level_locked
}
