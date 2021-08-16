use std::{any::type_name, collections::BTreeSet};

use crossbeam::queue::SegQueue;

use crate::{
    component::{Component, Components},
    world::ComponentWorld,
    Read, Write,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NodeId(pub u64);

enum Command {
    MarkNoPreUpdate(&'static str),
    MarkNoUpdate(&'static str),
    MarkNoPostUpdate(&'static str),
}

pub struct Node {
    pub name: String,
    pub components: Components,
    pub id: Option<NodeId>,
    commands: SegQueue<Command>,
    pre_update: BTreeSet<&'static str>,
    update: BTreeSet<&'static str>,
    post_update: BTreeSet<&'static str>,
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
        self.pre_update.insert(type_name::<T>());
        self.update.insert(type_name::<T>());
        self.post_update.insert(type_name::<T>());
    }

    #[inline]
    pub unsafe fn insert_raw(&mut self, name: &'static str, component: Box<dyn Component>) {
        self.components.insert_raw(name, component);
        self.pre_update.insert(name);
        self.update.insert(name);
        self.post_update.insert(name);
    }

    #[inline]
    pub fn contains<T: Component>(&self) -> bool {
        self.components.components.contains_key(type_name::<T>())
    }

    #[inline]
    pub fn component_mut<T: Component>(&mut self) -> Option<&mut T> {
        self.components.get_mut::<T>()
    }

    #[inline]
    pub fn read_component<T: Component>(&self) -> Option<Read<T>> {
        self.components.read::<T>()
    }

    #[inline]
    pub fn write_component<'a, T: Component>(&'a self) -> Option<Write<'a, T>> {
        self.components.write::<T>()
    }

    /// Calls init on add [`Component`]s.
    #[inline]
    pub fn init(&self, world: &mut ComponentWorld) {
        for mut component in self.components.iter_mut() {
            component.init(self, world);
        }
    }

    #[inline]
    pub(crate) fn mark_no_pre_update(&self, id: &'static str) {
        self.commands.push(Command::MarkNoPreUpdate(id));
    }

    #[inline]
    pub(crate) fn mark_no_update(&self, id: &'static str) {
        self.commands.push(Command::MarkNoUpdate(id));
    }

    #[inline]
    pub(crate) fn mark_no_post_update(&self, id: &'static str) {
        self.commands.push(Command::MarkNoPostUpdate(id));
    }

    /// Calls update on add [`Component`]s.
    #[inline]
    pub fn pre_update(&self, world: &mut ComponentWorld) {
        for id in &self.pre_update {
            if let Some(component) = self.components.components.get(id) {
                component.write().unwrap().pre_update(self, world);
            }
        }
    }

    /// Calls update on add [`Component`]s.
    #[inline]
    pub fn update(&self, world: &mut ComponentWorld) {
        for id in &self.update {
            if let Some(component) = self.components.components.get(id) {
                component.write().unwrap().update(self, world);
            }
        }
    }

    /// Calls update on add [`Component`]s.
    #[inline]
    pub fn post_update(&self, world: &mut ComponentWorld) {
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
