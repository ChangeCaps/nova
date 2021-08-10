use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
    sync::RwLock,
};

use crate::{world::World, Read, Write};

#[allow(unused)]
pub trait System: 'static {
    #[inline]
    fn init(&self, world: &World) {}

    #[inline]
    fn update(&self, world: &World) {}
}

#[derive(Default)]
pub struct Systems {
    systems: BTreeMap<TypeId, RwLock<Box<dyn System>>>,
}

impl Systems {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn insert<T: System>(&mut self, system: T) {
        self.systems
            .insert(system.type_id(), RwLock::new(Box::new(system)));
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
}
