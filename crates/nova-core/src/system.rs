use std::{any::type_name, collections::BTreeMap};

use crossbeam::{queue::SegQueue, sync::ShardedLock};

use crate::{component::AsAny, world::SystemWorld, Read, Write};

#[allow(unused)]
pub trait System: AsAny + Send + Sync + 'static {
    #[inline]
    fn init(&mut self, world: &mut SystemWorld) {}

    #[inline]
    fn pre_update(&mut self, world: &mut SystemWorld) {}

    #[inline]
    fn update(&mut self, world: &mut SystemWorld) {}

    #[inline]
    fn post_update(&mut self, world: &mut SystemWorld) {}
}

#[derive(Default)]
pub struct Systems {
    queue: SegQueue<(&'static str, Box<dyn System>)>,
    pub(crate) systems: BTreeMap<&'static str, ShardedLock<Box<dyn System>>>,
}

impl Systems {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn insert<T: System>(&self, system: T) {
        self.queue.push((type_name::<T>(), Box::new(system)));
    }

    #[inline]
    pub unsafe fn insert_raw(&self, name: &'static str, system: Box<dyn System>) {
        self.queue.push((name, system));
    }

    #[inline]
    pub fn contains<T: System>(&self) -> bool {
        self.systems.contains_key(&type_name::<T>())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.systems.len()
    }

    #[inline]
    pub fn get_mut<T: System>(&mut self) -> Option<&mut T> {
        let system = self
            .systems
            .get_mut(&type_name::<T>())?
            .get_mut()
            .ok()?
            .as_mut();

        Some(unsafe { &mut *(system as *mut _ as *mut _) })
    }

    #[inline]
    pub fn read<T: System>(&self) -> Option<Read<T>> {
        if let Some(lock) = self.systems.get(&type_name::<T>()) {
            let read = lock.read().ok()?;

            // SAFETY: we know that type ids are equal so transmuting is safe.
            Some(Read(unsafe { std::mem::transmute(read) }))
        } else {
            None
        }
    }

    #[inline]
    pub fn write<T: System>(&self) -> Option<Write<T>> {
        if let Some(lock) = self.systems.get(&type_name::<T>()) {
            let write = lock.write().ok()?;

            // SAFETY: we know that type ids are equal so transmuting is safe.
            Some(Write(unsafe { std::mem::transmute(write) }))
        } else {
            None
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&str, Read<dyn System>)> {
        self.systems
            .iter()
            .map(|(name, lock)| (*name, Read(lock.read().unwrap())))
    }

    #[inline]
    pub fn iter_mut(&self) -> impl Iterator<Item = Write<dyn System>> {
        self.systems
            .iter()
            .map(|(_, lock)| Write(lock.write().unwrap()))
    }

    #[inline]
    pub fn dequeue(&mut self) {
        for _ in 0..self.queue.len() {
            let (id, system) = self.queue.pop().unwrap();
            self.systems.insert(id, system.into());
        }
    }
}
