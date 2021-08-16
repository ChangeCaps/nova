use std::{any::type_name, collections::BTreeMap};

use crossbeam::{queue::SegQueue, sync::ShardedLock};

use crate::{component::AsAny, Read, Write};

pub trait Resource: AsAny + Send + Sync + 'static {}

impl<T: Send + Sync + 'static> Resource for T {}

#[derive(Default)]
pub struct Resources {
    queue: SegQueue<(&'static str, Box<dyn Resource>)>,
    pub(crate) resources: BTreeMap<&'static str, ShardedLock<Box<dyn Resource>>>,
}

impl Resources {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn insert<T: Resource>(&self, system: T) {
        self.queue.push((type_name::<T>(), Box::new(system)));
    }

    #[inline]
    pub unsafe fn insert_raw(&self, name: &'static str, system: Box<dyn Resource>) {
        self.queue.push((name, system));
    }

    #[inline]
    pub fn contains<T: Resource>(&self) -> bool {
        self.resources.contains_key(type_name::<T>())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.resources.len()
    }

    #[inline]
    pub fn get_mut<T: Resource>(&mut self) -> Option<&mut T> {
        let resource = self
            .resources
            .get_mut(&type_name::<T>())?
            .get_mut()
            .ok()?
            .as_mut();

        Some(unsafe { &mut *(resource as *mut _ as *mut _) })
    }

    #[inline]
    pub fn read<T: Resource>(&self) -> Option<Read<T>> {
        if let Some(lock) = self.resources.get(type_name::<T>()) {
            let read = lock.read().ok()?;

            // SAFETY: we know that type ids are equal so transmuting is safe.
            Some(Read(unsafe { std::mem::transmute(read) }))
        } else {
            None
        }
    }

    #[inline]
    pub fn write<T: Resource>(&self) -> Option<Write<T>> {
        if let Some(lock) = self.resources.get(&type_name::<T>()) {
            let write = lock.write().ok()?;

            // SAFETY: we know that type ids are equal so transmuting is safe.
            Some(Write(unsafe { std::mem::transmute(write) }))
        } else {
            None
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&str, Read<dyn Resource>)> {
        self.resources
            .iter()
            .map(|(name, lock)| (*name, Read(lock.read().unwrap())))
    }

    #[inline]
    pub fn iter_mut(&self) -> impl Iterator<Item = Write<dyn Resource>> {
        self.resources
            .iter()
            .map(|(_, lock)| Write(lock.write().unwrap()))
    }

    #[inline]
    pub fn dequeue(&mut self) {
        for _ in 0..self.queue.len() {
            let (id, system) = self.queue.pop().unwrap();
            self.resources.insert(id, system.into());
        }
    }
}
