use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
    sync::RwLock,
};

use crate::{node::Node, world::World, Read, Write};

#[allow(unused)]
pub trait Component: 'static {
    #[inline]
    fn init(&mut self, node: &Node, world: &World) {}

    #[inline]
    fn update(&mut self, node: &Node, world: &World) {}
}

/// A collection of [`Component`]s.
#[derive(Default)]
pub struct Components {
    components: BTreeMap<TypeId, RwLock<Box<dyn Component>>>,
}

impl Components {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn insert<T: Component>(&mut self, component: T) {
        self.components
            .insert(component.type_id(), RwLock::new(Box::new(component)));
    }

    #[inline]
    pub fn get<T: Component>(&self) -> Option<Read<Box<T>>> {
        if let Some(lock) = self.components.get(&TypeId::of::<T>()) {
            let read = lock.read().ok()?;

            // SAFETY: we know that type ids are equal so transmuting is safe.
            Some(unsafe { std::mem::transmute(read) })
        } else {
            None
        }
    }

    #[inline]
    pub fn get_mut<T: Component>(&self) -> Option<Write<Box<T>>> {
        if let Some(lock) = self.components.get(&TypeId::of::<T>()) {
            let write = lock.write().ok()?;

            // SAFETY: we know that type ids are equal so transmuting is safe.
            Some(unsafe { std::mem::transmute(write) })
        } else {
            None
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = Option<Read<Box<dyn Component>>>> {
        self.components.iter().map(|(_, lock)| lock.read().ok())
    }

    #[inline]
    pub fn iter_mut(&self) -> impl Iterator<Item = Option<Write<Box<dyn Component>>>> {
        self.components.iter().map(|(_, lock)| lock.write().ok())
    }

    #[inline]
    pub fn iter_filtered(&self) -> impl Iterator<Item = Read<Box<dyn Component>>> {
        self.components
            .iter()
            .filter_map(|(_, lock)| lock.read().ok())
    }

    #[inline]
    pub fn iter_mut_filtered(&self) -> impl Iterator<Item = Write<Box<dyn Component>>> {
        self.components
            .iter()
            .filter_map(|(_, lock)| lock.write().ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct TestComponent(u32);

    impl Component for TestComponent {}

    #[test]
    fn components_get() {
        let mut components = Components::new();

        components.insert(TestComponent(3));

        let component = components.get::<TestComponent>().unwrap();

        assert_eq!(*component, Box::new(TestComponent(3)));
    }
}
