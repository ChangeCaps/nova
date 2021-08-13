use std::{any::TypeId, collections::BTreeMap, sync::RwLock};

use crate::{
    node::{Node, NodeId},
    plugin::Plugin,
    system::{System, Systems},
    Read, Write,
};

#[derive(Default)]
pub struct World {
    pub systems: Systems,
    pub nodes: BTreeMap<NodeId, Node>,
    pub next_node_id: NodeId,
    pub running: bool,
}

impl World {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn generate_node_id(&mut self) -> NodeId {
        let id = self.next_node_id;
        self.next_node_id.0 += 1;
        id
    }

    #[inline]
    pub fn insert_node(&mut self, mut node: Node) -> NodeId {
        if self.running {
            node.init(self);
        }

        let id = self.generate_node_id();
        node.id = Some(id);
        self.nodes.insert(id, node.into());
        id
    }

    #[inline]
    pub fn node(&self, id: &NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    #[inline]
    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    #[inline]
    pub fn nodes_mut(&mut self) -> impl Iterator<Item = &mut Node> {
        self.nodes.values_mut()
    }

    /// If system doesn't already exist, insert it.
    #[inline]
    pub fn register_system<T: System + Default>(&self) {
        if !self.systems.contains::<T>() {
            self.insert_system(T::default());
        }
    }

    /// Inserts system by adding it to a queue, and is at the next dequeue.
    #[inline]
    pub fn insert_system<T: System>(&self, system: T) {
        self.systems.insert(system);
    }

    /// If system doesn't already exist, insert it now.
    #[inline]
    pub fn register_system_now<T: System + Default>(&mut self) {
        if !self.systems.contains::<T>() {
            self.insert_system_now(T::default());
        }
    }

    /// Inserts system now, rather than queueing it.
    #[inline]
    pub fn insert_system_now<T: System>(&mut self, system: T) {
        self.systems
            .systems
            .insert(TypeId::of::<T>(), RwLock::new(Box::new(system)));
    }

    #[inline]
    pub fn system<T: System>(&self) -> Option<Read<Box<T>>> {
        self.systems.get()
    }

    #[inline]
    pub fn system_mut<T: System>(&self) -> Option<Write<Box<T>>> {
        self.systems.get_mut()
    }

    #[inline]
    pub fn with_plugin(&mut self, plugin: impl Plugin) -> &mut Self {
        plugin.build(self);
        self.dequeue();

        self
    }

    #[inline]
    pub fn dequeue(&mut self) {
        self.systems.dequeue();

        self.nodes.iter_mut().for_each(|(_id, node)| {
            node.dequeue();
        });
    }

    #[inline]
    pub fn init(&mut self) {
        for mut system in self.systems.iter_mut_filtered() {
            system.init(self);
        }

        for node in self.nodes() {
            node.init(self);
        }
    }

    #[inline]
    pub fn pre_update(&mut self) {
        for mut system in self.systems.iter_mut_filtered() {
            system.pre_update(self);
        }

        self.dequeue();

        self.nodes.iter().for_each(|(_id, node)| {
            node.pre_update(self);
        });

        self.dequeue();
    }

    #[inline]
    pub fn update(&mut self) {
        for mut system in self.systems.iter_mut_filtered() {
            system.update(self);
        }

        self.dequeue();

        self.nodes.iter().for_each(|(_id, node)| {
            node.update(self);
        });

        self.dequeue();
    }

    #[inline]
    pub fn post_update(&mut self) {
        for mut system in self.systems.iter_mut_filtered() {
            system.post_update(self);
        }

        self.dequeue();

        self.nodes.iter().for_each(|(_id, node)| {
            node.post_update(self);
        });

        self.dequeue();
    }
}
