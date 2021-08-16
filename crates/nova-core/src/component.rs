use std::{
    any::{type_name, Any, TypeId},
    collections::BTreeMap,
};

use crossbeam::sync::ShardedLock;

use crate::{node::Node, world::ComponentWorld, Read, Write};

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn type_id(&self) -> TypeId;
    fn type_name(&self) -> &'static str;
}

impl<T: Any> AsAny for T {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    #[inline]
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    #[inline]
    fn type_name(&self) -> &'static str {
        type_name::<Self>()
    }
}

#[allow(unused)]
pub trait Component: AsAny + Send + Sync + 'static {
    #[inline]
    fn init(&mut self, node: &Node, world: &mut ComponentWorld) {}

    #[inline]
    fn pre_update(&mut self, node: &Node, world: &mut ComponentWorld) {
        node.mark_no_pre_update(AsAny::type_name(self));
    }

    #[inline]
    fn update(&mut self, node: &Node, world: &mut ComponentWorld) {
        node.mark_no_update(AsAny::type_name(self));
    }

    #[inline]
    fn post_update(&mut self, node: &Node, world: &mut ComponentWorld) {
        node.mark_no_post_update(AsAny::type_name(self));
    }
}

/// A collection of [`Component`]s.
#[derive(Default)]
pub struct Components {
    pub(crate) components: BTreeMap<&'static str, ShardedLock<Box<dyn Component>>>,
}

impl Components {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn insert<T: Component>(&mut self, component: T) {
        self.components
            .insert(type_name::<T>(), ShardedLock::new(Box::new(component)));
    }

    #[inline]
    pub unsafe fn insert_raw(&mut self, name: &'static str, component: Box<dyn Component>) {
        self.components.insert(name, ShardedLock::new(component));
    }

    #[inline]
    pub fn get_mut<T: Component>(&mut self) -> Option<&mut T> {
        let component = self
            .components
            .get_mut(&type_name::<T>())?
            .get_mut()
            .ok()?
            .as_mut();

        Some(unsafe { &mut *(component as *mut _ as *mut _) })
    }

    #[inline]
    pub fn read<T: Component>(&self) -> Option<Read<T>> {
        if let Some(lock) = self.components.get(type_name::<T>()) {
            let read = lock.read().ok()?;

            // SAFETY: we know that type ids are equal so transmuting is safe.
            Some(Read(unsafe { std::mem::transmute(read) }))
        } else {
            None
        }
    }

    #[inline]
    pub fn write<T: Component>(&self) -> Option<Write<T>> {
        if let Some(lock) = self.components.get(type_name::<T>()) {
            let write = lock.write().ok()?;

            // SAFETY: we know that type ids are equal so transmuting is safe.
            Some(Write(unsafe { std::mem::transmute(write) }))
        } else {
            None
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&str, Read<dyn Component>)> {
        self.components
            .iter()
            .map(|(name, lock)| (*name, Read(lock.read().unwrap())))
    }

    #[inline]
    pub fn iter_mut(&self) -> impl Iterator<Item = Write<dyn Component>> {
        self.components
            .iter()
            .map(|(_, lock)| Write(lock.write().unwrap()))
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

        let component = components.read::<TestComponent>().unwrap();

        assert_eq!(*component, TestComponent(3));
    }
}
