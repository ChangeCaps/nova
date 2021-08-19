use std::{any::type_name, collections::HashMap};

use erased_serde::{Deserializer, Serializer};
use legion::{
    query::Any,
    serialize::Canon,
    storage::Component,
    systems::{ParallelRunnable, Resource},
    Registry, Resources, Schedule, World,
};
use serde::{
    de::{DeserializeOwned, DeserializeSeed},
    Serialize,
};

use crate::{system::RunnableContainer, Plugin};

pub mod stage {
    pub const START: &'static str = "start";
    pub const PRE_UPDATE: &'static str = "pre_update";
    pub const UPDATE: &'static str = "update";
    pub const POST_UPDATE: &'static str = "post_update";
    pub const END: &'static str = "end";
}

#[derive(Default)]
pub struct Stage {
    pub systems: Vec<Box<dyn RunnableContainer>>,
    pub thread_locals: Vec<Box<dyn FnMut(&mut World, &mut Resources)>>,
}

pub struct App {
    pub world: World,
    pub resources: Resources,
    pub registry: Registry<String>,
    pub update: fn(&mut Schedule, &mut World, &mut Resources),
    pub serialize: fn(
        &World,
        &Registry<String>,
        &mut dyn Serializer,
    ) -> Result<(), Box<dyn std::error::Error>>,
    pub deserialize: fn(
        &mut World,
        &Registry<String>,
        &mut dyn Deserializer,
    ) -> Result<(), Box<dyn std::error::Error>>,
    pub unload: fn(Self),
    pub startup_schedule: Schedule,
    pub schedule: Schedule,
    #[cfg(feature = "editor")]
    pub editor_schedule: Schedule,
}

pub fn update(schedule: &mut Schedule, world: &mut World, resources: &mut Resources) {
    schedule.execute(world, resources);
}

pub fn serialize(
    world: &World,
    registry: &Registry<String>,
    serializer: &mut dyn Serializer,
) -> Result<(), Box<dyn std::error::Error>> {
    let canon = Canon::default();

    let serialize = world.as_serializable(Any, registry, &canon);

    serialize.serialize(serializer)?;

    Ok(())
}

pub fn deserialize(
    world: &mut World,
    registry: &Registry<String>,
    deserializer: &mut dyn Deserializer,
) -> Result<(), Box<dyn std::error::Error>> {
    let canon = Canon::default();

    let deserialize = registry.as_deserialize_into_world(world, &canon);

    deserialize.deserialize(deserializer)?;

    Ok(())
}

pub fn unload(app: App) {
    drop(app);
}

#[derive(Default)]
pub struct AppBuilder {
    pub world: World,
    pub resources: Resources,
    pub registry: Registry<String>,
    pub order: Vec<&'static str>,
    pub stages: HashMap<&'static str, Stage>,
    pub startup: Stage,
    #[cfg(feature = "editor")]
    pub editor_order: Vec<&'static str>,
    #[cfg(feature = "editor")]
    pub editor_stages: HashMap<&'static str, Stage>,
}

impl AppBuilder {
    #[inline]
    pub fn new() -> Self {
        let mut app_builder = Self {
            world: World::new(Default::default()),
            resources: Resources::default(),
            registry: Registry::new(),
            order: Vec::new(),
            stages: HashMap::new(),
            startup: Stage::default(),
            #[cfg(feature = "editor")]
            editor_order: Vec::new(),
            #[cfg(feature = "editor")]
            editor_stages: HashMap::new(),
        };

        app_builder
            .push_stage(stage::START)
            .push_stage(stage::PRE_UPDATE)
            .push_stage(stage::UPDATE)
            .push_stage(stage::POST_UPDATE)
            .push_stage(stage::END);

        #[cfg(feature = "editor")]
        app_builder
            .push_editor_stage(stage::START)
            .push_editor_stage(stage::PRE_UPDATE)
            .push_editor_stage(stage::UPDATE)
            .push_editor_stage(stage::POST_UPDATE)
            .push_editor_stage(stage::END);

        app_builder
    }

    #[inline]
    pub fn register_component<T: Component + Serialize + DeserializeOwned>(&mut self) -> &mut Self {
        self.registry.register::<T>(String::from(type_name::<T>()));

        self
    }

    #[inline]
    pub fn insert_resource<T: Resource>(&mut self, resource: T) -> &mut Self {
        self.resources.insert(resource);

        self
    }

    #[inline]
    pub fn register_resource<T: Resource + Default>(&mut self) -> &mut Self {
        if !self.resources.contains::<T>() {
            self.resources.insert(T::default());
        }

        self
    }

    #[inline]
    pub fn add_startup_system<T: ParallelRunnable + 'static>(&mut self, system: T) -> &mut Self {
        self.startup.systems.push(Box::new(system));

        self
    }

    #[inline]
    pub fn add_startup_thread_local<F: FnMut(&mut World, &mut Resources) + 'static>(
        &mut self,
        f: F,
    ) -> &mut Self {
        self.startup.thread_locals.push(Box::new(f));

        self 
    }

    #[inline]
    pub fn add_system<T: ParallelRunnable + 'static>(&mut self, system: T) -> &mut Self {
        self.add_system_to_stage(stage::UPDATE, system);

        self
    }

    #[inline]
    #[cfg(feature = "editor")]
    pub fn add_editor_system<T: ParallelRunnable + 'static>(&mut self, system: T) -> &mut Self {
        self.add_editor_system_to_stage(stage::UPDATE, system);

        self
    }

    #[inline]
    pub fn add_system_to_stage<T: ParallelRunnable + 'static>(
        &mut self,
        stage: &str,
        system: T,
    ) -> &mut Self {
        self.stages
            .get_mut(stage)
            .unwrap()
            .systems
            .push(Box::new(system));

        self
    }

    #[cfg(feature = "editor")]
    #[inline]
    pub fn add_editor_system_to_stage<T: ParallelRunnable + 'static>(
        &mut self,
        stage: &str,
        system: T,
    ) -> &mut Self {
        self.editor_stages
            .get_mut(stage)
            .unwrap()
            .systems
            .push(Box::new(system));

        self
    }

    #[inline]
    pub fn add_thread_local_to_stage<F: FnMut(&mut World, &mut Resources) + 'static>(
        &mut self,
        stage: &str,
        f: F,
    ) -> &mut Self {
        self.stages
            .get_mut(stage)
            .unwrap()
            .thread_locals
            .push(Box::new(f));

        self
    }

    #[inline]
    pub fn push_stage(&mut self, stage: &'static str) -> &mut Self {
        self.order.push(stage);
        self.stages.insert(stage, Stage::default());

        self
    }

    #[cfg(feature = "editor")]
    #[inline]
    pub fn push_editor_stage(&mut self, stage: &'static str) -> &mut Self {
        self.editor_order.push(stage);
        self.editor_stages.insert(stage, Stage::default());

        self
    }

    #[inline]
    pub fn add_stage_before(&mut self, stage: &'static str, before: &str) -> &mut Self {
        let idx = self.order.iter().position(|s| *s == before).unwrap();
        self.order.insert(idx, stage);
        self.stages.insert(stage, Stage::default());

        self
    }

    #[cfg(feature = "editor")]
    #[inline]
    pub fn add_editor_stage_before(&mut self, stage: &'static str, before: &str) -> &mut Self {
        let idx = self.editor_order.iter().position(|s| *s == before).unwrap();
        self.editor_order.insert(idx, stage);
        self.editor_stages.insert(stage, Stage::default());

        self
    }

    #[inline]
    pub fn add_stage_after(&mut self, stage: &'static str, after: &str) -> &mut Self {
        let idx = self.order.iter().position(|s| *s == after).unwrap();
        self.order.insert(idx + 1, stage);
        self.stages.insert(stage, Stage::default());

        self
    }

    #[cfg(feature = "editor")]
    #[inline]
    pub fn add_editor_stage_after(&mut self, stage: &'static str, after: &str) -> &mut Self {
        let idx = self.editor_order.iter().position(|s| *s == after).unwrap();
        self.editor_order.insert(idx + 1, stage);
        self.editor_stages.insert(stage, Stage::default());

        self
    }

    #[inline]
    pub fn with_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        plugin.build(self);

        self
    }

    #[inline]
    pub fn build(mut self) -> App {
        let mut startup_schedule = Schedule::builder();

        for system in self.startup.systems {
            startup_schedule.add_system(system);
        }

        for thread_local in self.startup.thread_locals {
            startup_schedule.add_thread_local_fn(thread_local);
        }

        let mut schedule = Schedule::builder();

        for stage in self.order {
            let stage = self.stages.remove(stage).unwrap();

            for system in stage.systems {
                schedule.add_system(system);
            }

            for thread_local in stage.thread_locals {
                schedule.add_thread_local_fn(thread_local);
            }

            schedule.flush();
        }

        #[cfg(feature = "editor")]
        let mut editor_schedule = Schedule::builder();

        #[cfg(feature = "editor")]
        for stage in self.editor_order {
            let stage = self.editor_stages.remove(stage).unwrap();

            for system in stage.systems {
                editor_schedule.add_system(system);
            }

            for thread_local in stage.thread_locals {
                editor_schedule.add_thread_local_fn(thread_local);
            }

            editor_schedule.flush();
        }

        App {
            world: self.world,
            resources: self.resources,
            registry: self.registry,
            update,
            serialize,
            deserialize,
            unload,
            startup_schedule: startup_schedule.build(),
            schedule: schedule.build(),
            #[cfg(feature = "editor")]
            editor_schedule: editor_schedule.build(),
        }
    }
}
