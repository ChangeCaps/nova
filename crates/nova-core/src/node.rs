use crate::{
    component::{Component, Components},
    world::World,
    Read, Write,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct NodeId(pub u64);

pub struct Node {
    pub name: String,
    pub parent: Option<NodeId>,
    pub components: Components,
}

impl Node {
    #[inline]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parent: None,
            components: Components::new(),
        }
    }

    #[inline]
    pub fn add_component<T: Component>(&mut self, component: T) {
        self.components.insert(component);
    }

    #[inline]
    pub fn component<T: Component>(&self) -> Option<Read<Box<T>>> {
        self.components.get::<T>()
    }

    #[inline]
    pub fn component_mut<T: Component>(&self) -> Option<Write<Box<T>>> {
        self.components.get_mut::<T>()
    }

    /// Calls init on add [`Component`]s.
    #[inline]
    pub fn init(&self, world: &World) {
        for mut component in self.components.iter_mut_filtered() {
            component.init(self, world);
        }
    }

    /// Calls update on add [`Component`]s.
    #[inline]
    pub fn update(&self, world: &World) {
        for mut component in self.components.iter_mut_filtered() {
            component.update(self, world);
        }
    }
}
