use std::{collections::BTreeMap, sync::RwLock};

use crate::{
    node::{Node, NodeId},
    system::{System, Systems},
    Read, Write,
};

#[derive(Default)]
pub struct World {
    pub systems: Systems,
    pub nodes: BTreeMap<NodeId, RwLock<Node>>,
    pub next_node_id: NodeId,
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
    pub fn add_node(&mut self, node: Node) -> NodeId {
        let id = self.generate_node_id();
        self.nodes.insert(id, node.into());
        id
    }

    #[inline]
    pub fn node(&self, id: &NodeId) -> Option<Read<Node>> {
        self.nodes.get(id)?.read().ok()
    }

    #[inline]
    pub fn node_mut(&self, id: &NodeId) -> Option<Write<Node>> {
        self.nodes.get(id)?.write().ok()
    }

    #[inline]
    pub fn nodes(&self) -> impl Iterator<Item = Option<Read<Node>>> {
        self.nodes.iter().map(|(_, lock)| lock.read().ok())
    }

    #[inline]
    pub fn nodes_mut(&self) -> impl Iterator<Item = Option<Write<Node>>> {
        self.nodes.iter().map(|(_, lock)| lock.write().ok())
    }

    #[inline]
    pub fn nodes_filtered(&self) -> impl Iterator<Item = Read<Node>> {
        self.nodes.iter().filter_map(|(_, lock)| lock.read().ok())
    }

    #[inline]
    pub fn nodes_mut_filtered(&self) -> impl Iterator<Item = Write<Node>> {
        self.nodes.iter().filter_map(|(_, lock)| lock.write().ok())
    }

    #[inline]
    pub fn insert_system<T: System>(&mut self, system: T) {
        self.systems.insert(system);
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
    pub fn init(&mut self) {
        for system in self.systems.iter_mut_filtered() {
            system.init(self);
        }

        for node in self.nodes_filtered() {
            node.init(self);
        }
    }

    #[inline]
    pub fn update(&mut self) {
        for system in self.systems.iter_mut_filtered() {
            system.update(self);
        }

        for node in self.nodes_filtered() {
            node.update(self);
        }
    }
}
