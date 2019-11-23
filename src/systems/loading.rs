use super::system_prelude::*;

#[derive(Default)]
pub struct LoadingSystem;

impl<'a> System<'a> for LoadingSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Indicator>,
        ReadStorage<'a, Loader>,
        ReadStorage<'a, Loadable>,
        WriteStorage<'a, Loaded>,
        WriteStorage<'a, Hidden>,
    );

    fn run(
        &mut self,
        (
            entities,
            transforms,
            sizes,
            indicators,
            loaders,
            loadables,
            mut loadeds,
            mut hiddens,
        ): Self::SystemData,
    ) {
        let mut entities_loader = EntitiesLoader::default();

        for (loader, loader_transform) in (&loaders, &transforms).join() {
            let loader_pos = Vector::from(loader_transform);

            for (
                target_entity,
                target_transform,
                target_size_opt,
                _,
                target_loaded_opt,
            ) in (
                &entities,
                &transforms,
                sizes.maybe(),
                &loadables,
                loadeds.maybe(),
            )
                .join()
            {
                let target_pos = Vector::from(target_transform);
                let dist = {
                    let size = target_size_opt
                        .map(|s| (s.w, s.h))
                        .unwrap_or((0.0, 0.0));
                    (
                        ((loader_pos.0 - target_pos.0).abs() + size.0 * 0.5),
                        ((loader_pos.1 - target_pos.1).abs() + size.1 * 0.5),
                    )
                };

                let in_distance = dist.0 <= loader.loading_distance.0
                    && dist.1 <= loader.loading_distance.1;

                match target_loaded_opt {
                    None if in_distance => {
                        entities_loader.load(target_entity);
                    }
                    Some(_) => {
                        if in_distance {
                            entities_loader.maintain_loaded(target_entity);
                        } else {
                            entities_loader.unload(target_entity);
                        }
                    }
                    _ => (),
                }
            }
        }

        entities_loader.work(&mut loadeds, &mut hiddens, &indicators);
    }
}

/// Copied from LD44.
#[derive(Default)]
struct EntitiesLoader {
    to_load:            Vec<Entity>,
    to_unload:          Vec<Entity>,
    to_maintain_loaded: Vec<Entity>,
}

impl EntitiesLoader {
    pub fn load(&mut self, entity: Entity) {
        if !self.to_load.contains(&entity) {
            self.to_load.push(entity);
            self.maintain_loaded(entity);
        }
    }

    pub fn unload(&mut self, entity: Entity) {
        // Only unload if it isn't already staged for loading.
        if !self.to_load.contains(&entity) && !self.to_unload.contains(&entity)
        {
            self.to_unload.push(entity);
        }
    }

    pub fn maintain_loaded(&mut self, entity: Entity) {
        if !self.to_maintain_loaded.contains(&entity) {
            self.to_maintain_loaded.push(entity);
        }
    }

    pub fn work(
        self,
        loadeds: &mut WriteStorage<Loaded>,
        hiddens: &mut WriteStorage<Hidden>,
        indicators: &ReadStorage<Indicator>,
    ) {
        for entity in self.to_unload {
            if loadeds.contains(entity) {
                if !self.to_maintain_loaded.contains(&entity) {
                    loadeds.remove(entity);
                    // Don't hide Indicators
                    if !indicators.contains(entity) {
                        hiddens.insert(entity, Hidden).unwrap();
                    }
                }
            }
        }
        for entity in self.to_load {
            if !loadeds.contains(entity) {
                loadeds.insert(entity, Loaded).unwrap();
                // Don't show Indicators
                if !indicators.contains(entity) {
                    hiddens.remove(entity);
                }
            }
        }
    }
}
