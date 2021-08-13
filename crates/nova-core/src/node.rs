use std::{any::TypeId, collections::BTreeSet};

use crossbeam::queue::SegQueue;

use crate::{
    component::{Component, Components},
    world::World,
    Read, Write,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct NodeId(pub u64);

enum Command {
    MarkNoPreUpdate(TypeId),
    MarkNoUpdate(TypeId),
    MarkNoPostUpdate(TypeId),
}

pub struct Node {
    pub name: String,
    pub components: Components,
    pub id: Option<NodeId>,
    commands: SegQueue<Command>,
    pre_update: BTreeSet<TypeId>,
    update: BTreeSet<TypeId>,
    post_update: BTreeSet<TypeId>,
}

impl Node {
    #[inline]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            components: Components::new(),
            id: None,
            commands: SegQueue::new(),
            pre_update: BTreeSet::new(),
            update: BTreeSet::new(),
            post_update: BTreeSet::new(),
        }
    }

    #[inline]
    pub fn id(&self) -> NodeId {
        self.id.expect("id not set")
    }

    #[inline]
    pub fn add_component<T: Component>(&mut self, component: T) {
        self.components.insert(component);
        self.pre_update.insert(TypeId::of::<T>());
        self.update.insert(TypeId::of::<T>());
        self.post_update.insert(TypeId::of::<T>());
    }

    #[inline]
    pub fn contains<T: Component>(&self) -> bool {
        self.components.components.contains_key(&TypeId::of::<T>())
    }

    #[inline]
    pub fn component<T: Component>(&self) -> Option<Read<Box<T>>> {
        self.components.get::<T>()
    }

    #[inline]
    pub fn component_mut<'a, T: Component>(&'a self) -> Option<Write<'a, Box<T>>> {
        self.components.get_mut::<T>()
    }

    /// Calls init on add [`Component`]s.
    #[inline]
    pub fn init(&self, world: &World) {
        for mut component in self.components.iter_mut_filtered() {
            component.init(self, world);
        }
    }

    #[inline]
    pub(crate) fn mark_no_pre_update(&self, id: TypeId) {
        self.commands.push(Command::MarkNoPreUpdate(id));
    }

    #[inline]
    pub(crate) fn mark_no_update(&self, id: TypeId) {
        self.commands.push(Command::MarkNoUpdate(id));
    }

    #[inline]
    pub(crate) fn mark_no_post_update(&self, id: TypeId) {
        self.commands.push(Command::MarkNoPostUpdate(id));
    }

    /// Calls update on add [`Component`]s.
    #[inline]
    pub fn pre_update(&self, world: &World) {
        for id in &self.pre_update {
            if let Some(component) = self.components.components.get(id) {
                component.write().unwrap().pre_update(self, world);
            }
        }
    }

    /// Calls update on add [`Component`]s.
    #[inline]
    pub fn update(&self, world: &World) {
        for id in &self.update {
            if let Some(component) = self.components.components.get(id) {
                component.write().unwrap().update(self, world);
            }
        }
    }

    /// Calls update on add [`Component`]s.
    #[inline]
    pub fn post_update(&self, world: &World) {
        for id in &self.post_update {
            if let Some(component) = self.components.components.get(id) {
                component.write().unwrap().post_update(self, world);
            }
        }
    }

    #[inline]
    pub fn dequeue(&mut self) {
        for _ in 0..self.commands.len() {
            match self.commands.pop().unwrap() {
                Command::MarkNoPreUpdate(id) => self.pre_update.remove(&id),
                Command::MarkNoUpdate(id) => self.update.remove(&id),
                Command::MarkNoPostUpdate(id) => self.post_update.remove(&id),
            };
        }
    }
}
