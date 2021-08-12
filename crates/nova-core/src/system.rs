use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
    sync::RwLock,
};

use crossbeam::queue::SegQueue;

use crate::{world::World, Read, Write};

#[allow(unused)]
pub trait System: Send + Sync + 'static {
    #[inline]
    fn init(&mut self, world: &World) {}

    #[inline]
    fn pre_update(&mut self, world: &World) {}

    #[inline]
    fn update(&mut self, world: &World) {}

    #[inline]
    fn post_update(&mut self, world: &World) {}
}

#[derive(Default)]
pub struct Systems {
    queue: SegQueue<(TypeId, Box<dyn System>)>,
    systems: BTreeMap<TypeId, RwLock<Box<dyn System>>>,
}

impl Systems {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn insert<T: System>(&self, system: T) {
        self.queue.push((system.type_id(), Box::new(system)));
    }

    #[inline]
    pub fn contains<T: System>(&self) -> bool {
        self.systems.contains_key(&TypeId::of::<T>())
    }

    #[inline]
    pub fn get<T: System>(&self) -> Option<Read<Box<T>>> {
        if let Some(lock) = self.systems.get(&TypeId::of::<T>()) {
            let read = lock.read().ok()?;

            // SAFETY: we know that type ids are equal so transmuting is safe.
            Some(unsafe { std::mem::transmute(read) })
        } else {
            None
        }
    }

    #[inline]
    pub fn get_mut<T: System>(&self) -> Option<Write<Box<T>>> {
        if let Some(lock) = self.systems.get(&TypeId::of::<T>()) {
            let write = lock.write().ok()?;

            // SAFETY: we know that type ids are equal so transmuting is safe.
            Some(unsafe { std::mem::transmute(write) })
        } else {
            None
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = Option<Read<Box<dyn System>>>> {
        self.systems.iter().map(|(_, lock)| lock.read().ok())
    }

    #[inline]
    pub fn iter_mut(&self) -> impl Iterator<Item = Option<Write<Box<dyn System>>>> {
        self.systems.iter().map(|(_, lock)| lock.write().ok())
    }

    #[inline]
    pub fn iter_filtered(&self) -> impl Iterator<Item = Read<Box<dyn System>>> {
        self.systems.iter().filter_map(|(_, lock)| lock.read().ok())
    }

    #[inline]
    pub fn iter_mut_filtered(&self) -> impl Iterator<Item = Write<Box<dyn System>>> {
        self.systems
            .iter()
            .filter_map(|(_, lock)| lock.write().ok())
    }

    #[inline]
    pub fn dequeue(&mut self) {
        for _ in 0..self.queue.len() {
            let (id, system) = self.queue.pop().unwrap();
            self.systems.insert(id, system.into());
        }
    }
}
