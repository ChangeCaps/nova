use std::{any::type_name, collections::HashMap};

use crossbeam::sync::ShardedLock;

use crate::{
    node::{Node, NodeId},
    plugin::Plugin,
    resources::{Resource, Resources},
    system::{System, Systems},
    Read, Write,
};

#[derive(Default)]
pub struct WorldData {
    pub systems: Systems,
    pub resources: Resources,
    pub nodes: Nodes,
    pub next_node_id: NodeId,
    pub running: bool,
}

impl WorldData {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn world(&mut self) -> World {
        World {
            systems: &mut self.systems,
            resources: &mut self.resources,
            nodes: &mut self.nodes,
            next_node_id: &mut self.next_node_id,
            running: &self.running,
        }
    }

    #[inline]
    pub fn system_world(&mut self) -> SystemWorld {
        World {
            systems: &self.systems,
            resources: &mut self.resources,
            nodes: &mut self.nodes,
            next_node_id: &mut self.next_node_id,
            running: &self.running,
        }
    }

    #[inline]
    pub fn ref_world(&mut self) -> RefWorld {
        World {
            systems: &self.systems,
            resources: &self.resources,
            nodes: &self.nodes,
            next_node_id: &mut self.next_node_id,
            running: &self.running,
        }
    }

    #[inline]
    pub fn dequeue(&mut self) {
        self.systems.dequeue();
        self.resources.dequeue();

        for node in self.nodes.values_mut() {
            node.dequeue();
        }
    }

    #[inline]
    pub fn init(&mut self) {
        let mut world = World {
            systems: &self.systems,
            resources: &mut self.resources,
            nodes: &mut self.nodes,
            next_node_id: &mut self.next_node_id,
            running: &self.running,
        };

        for mut system in self.systems.iter_mut() {
            system.init(&mut world);
        }

        self.dequeue();

        let mut world = World {
            systems: &mut self.systems,
            resources: &mut self.resources,
            nodes: &self.nodes,
            next_node_id: &mut self.next_node_id,
            running: &self.running,
        };

        for node in self.nodes.values() {
            node.init(&mut world);
        }

        self.dequeue();
    }

    #[inline]
    pub fn pre_update(&mut self) {
        let mut world = World {
            systems: &self.systems,
            resources: &mut self.resources,
            nodes: &mut self.nodes,
            next_node_id: &mut self.next_node_id,
            running: &self.running,
        };

        for mut system in self.systems.iter_mut() {
            system.pre_update(&mut world);
        }

        self.dequeue();

        let mut world = World {
            systems: &mut self.systems,
            resources: &mut self.resources,
            nodes: &self.nodes,
            next_node_id: &mut self.next_node_id,
            running: &self.running,
        };

        for node in self.nodes.values() {
            node.pre_update(&mut world);
        }

        self.dequeue();
    }

    #[inline]
    pub fn update(&mut self) {
        let mut world = World {
            systems: &self.systems,
            resources: &mut self.resources,
            nodes: &mut self.nodes,
            next_node_id: &mut self.next_node_id,
            running: &self.running,
        };

        for mut system in self.systems.iter_mut() {
            system.update(&mut world);
        }

        self.dequeue();

        let mut world = World {
            systems: &mut self.systems,
            resources: &mut self.resources,
            nodes: &self.nodes,
            next_node_id: &mut self.next_node_id,
            running: &self.running,
        };

        for node in self.nodes.values() {
            node.update(&mut world);
        }

        self.dequeue();
    }

    #[inline]
    pub fn post_update(&mut self) {
        let mut world = World {
            systems: &self.systems,
            resources: &mut self.resources,
            nodes: &mut self.nodes,
            next_node_id: &mut self.next_node_id,
            running: &self.running,
        };

        for mut system in self.systems.iter_mut() {
            system.post_update(&mut world);
        }

        self.dequeue();

        let mut world = World {
            systems: &mut self.systems,
            resources: &mut self.resources,
            nodes: &self.nodes,
            next_node_id: &mut self.next_node_id,
            running: &self.running,
        };

        for node in self.nodes.values() {
            node.post_update(&mut world);
        }

        self.dequeue();
    }
}

pub struct Ref;

impl<'a, T: 'a> Borrow<'a, T> for Ref {
    type Borrow = &'a T;

    #[inline]
    fn as_ref<'b>(borrow: &'b Self::Borrow) -> &'b T
    where
        'a: 'b,
    {
        *borrow
    }
}

pub struct Mut;

impl<'a, T: 'a> Borrow<'a, T> for Mut {
    type Borrow = &'a mut T;

    #[inline]
    fn as_ref<'b>(borrow: &'b Self::Borrow) -> &'b T
    where
        'a: 'b,
    {
        *borrow
    }
}

pub trait Borrow<'a, T> {
    type Borrow;

    fn as_ref<'b>(borrow: &'b Self::Borrow) -> &'b T
    where
        'a: 'b;
}

pub type Nodes = HashMap<NodeId, Node>;

pub struct World<'a, S = Mut, R = Mut, N = Mut>
where
    S: Borrow<'a, Systems>,
    R: Borrow<'a, Resources>,
    N: Borrow<'a, Nodes>,
{
    pub systems: S::Borrow,
    pub resources: R::Borrow,
    pub nodes: N::Borrow,
    next_node_id: &'a mut NodeId,
    running: &'a bool,
}

impl<'a, R, N> World<'a, Mut, R, N>
where
    R: Borrow<'a, Resources>,
    N: Borrow<'a, Nodes>,
{
    #[inline]
    pub fn insert_system<T: System>(&mut self, system: T) {
        if !self.systems.contains::<T>() {
            self.systems
                .systems
                .insert(type_name::<T>(), ShardedLock::new(Box::new(system)));
        }
    }

    #[inline]
    pub fn register_system<T: System + Default>(&mut self) {
        if !self.systems.contains::<T>() {
            self.insert_system(T::default());
        }
    }

    #[inline]
    pub fn system_mut<T: System>(&mut self) -> Option<&mut T> {
        self.systems.get_mut()
    }
}

impl<'a, S, N> World<'a, S, Mut, N>
where
    S: Borrow<'a, Systems>,
    N: Borrow<'a, Nodes>,
{
    #[inline]
    pub fn insert_resource<T: Resource>(&mut self, resource: T) {
        if !self.resources.contains::<T>() {
            self.resources
                .resources
                .insert(type_name::<T>(), ShardedLock::new(Box::new(resource)));
        }
    }

    #[inline]
    pub fn register_resource<T: Resource + Default>(&mut self) {
        if !self.resources.contains::<T>() {
            self.insert_resource(T::default());
        }
    }

    #[inline]
    pub fn resource_mut<T: Resource>(&mut self) -> Option<&mut T> {
        self.resources.get_mut()
    }
}

impl<'a, S, R> World<'a, S, R, Mut>
where
    S: Borrow<'a, Systems>,
    R: Borrow<'a, Resources>,
{
    #[inline]
    pub fn add_node(&mut self, mut node: Node) -> NodeId {
        if *self.running {
            //node.init(self);
            unimplemented!()
        }

        let id = self.next_id();
        node.id = Some(id);
        self.nodes.insert(id, node);
        id
    }

    #[inline]
    pub fn insert_node(&mut self, id: NodeId, mut node: Node) {
        self.next_node_id.0 = self.next_node_id.0.max(id.0 + 1);
        node.id = Some(id);
        self.nodes.insert(id, node);
    }

    #[inline]
    pub fn node_mut(&mut self, id: &NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    #[inline]
    pub fn nodes_mut(&mut self) -> impl Iterator<Item = &mut Node> {
        self.nodes.values_mut()
    }
}

impl<'a, S, R> World<'a, S, R, Ref>
where
    S: Borrow<'a, Systems>,
    R: Borrow<'a, Resources>,
{
}

impl<'a, S, R, N> World<'a, S, R, N>
where
    S: Borrow<'a, Systems>,
    R: Borrow<'a, Resources>,
    N: Borrow<'a, Nodes>,
{
    #[inline]
    pub fn next_id(&mut self) -> NodeId {
        let id = *self.next_node_id;
        self.next_node_id.0 += 1;
        id
    }

    #[inline]
    pub fn has_system<T: System>(&self) -> bool {
        S::as_ref(&self.systems).contains::<T>()
    }

    #[inline]
    pub fn has_resource<T: Resource>(&self) -> bool {
        R::as_ref(&self.resources).contains::<T>()
    }

    #[inline]
    pub fn read_system<T: System>(&self) -> Option<Read<T>> {
        S::as_ref(&self.systems).read()
    }

    #[inline]
    pub fn write_system<T: System>(&self) -> Option<Write<T>> {
        S::as_ref(&self.systems).write()
    }

    #[inline]
    pub fn read_resource<T: Resource>(&self) -> Option<Read<T>> {
        R::as_ref(&self.resources).read()
    }

    #[inline]
    pub fn write_resource<T: Resource>(&self) -> Option<Write<T>> {
        R::as_ref(&self.resources).write()
    }

    #[inline]
    pub fn node(&self, id: &NodeId) -> Option<&Node> {
        N::as_ref(&self.nodes).get(id)
    }

    #[inline]
    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        N::as_ref(&self.nodes).values()
    }
}

impl<'a> World<'a> {
    pub fn with_plugin(&mut self, plugin: impl Plugin) -> &mut Self {
        plugin.build(self);
        self
    }
}

pub type SystemWorld<'a> = World<'a, Ref, Mut, Mut>;
pub type ComponentWorld<'a> = World<'a, Mut, Mut, Ref>;
pub type RefWorld<'a> = World<'a, Ref, Ref, Ref>;
