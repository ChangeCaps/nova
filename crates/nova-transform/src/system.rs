use std::collections::BTreeMap;

use nova_core::{node::NodeId, system::System, world::World};

use crate::component::{GlobalTransform, Parent, Transform};

#[inline]
fn update_transform(
    children: &BTreeMap<NodeId, Vec<NodeId>>,
    world: &World,
    node: NodeId,
    parent: Transform,
) {
    let node = world.node(&node).unwrap();

    if let Some(transform) = node.component::<Transform>() {
        let transform = parent * transform.as_ref().clone();

        if let Some(mut global_transform) = node.component_mut::<GlobalTransform>() {
            global_transform.0 = transform.clone();
        }

        if let Some(child_nodes) = children.get(&node.id()) {
            for child in child_nodes {
                update_transform(children, world, *child, transform.clone());
            }
        }
    };
}

#[derive(Clone, Debug, Default)]
pub struct TransformSystem;

impl System for TransformSystem {
    #[inline]
    fn post_update(&mut self, world: &World) {
        let mut children: BTreeMap<NodeId, Vec<NodeId>> = BTreeMap::new();

        for node in world.nodes() {
            if let Some(parent) = node.component::<Parent>() {
                children.entry(parent.0).or_default().push(node.id());
            }
        }

        for node in world.nodes() {
            if !node.contains::<Parent>() {
                if let Some(transform) = node.component::<Transform>() {
                    if let Some(mut global_transform) = node.component_mut::<GlobalTransform>() {
                        ***global_transform = transform.as_ref().clone();
                    }

                    if let Some(child_nodes) = children.get(&node.id()) {
                        for child in child_nodes {
                            update_transform(&children, world, *child, transform.as_ref().clone());
                        }
                    }
                }
            }
        }
    }
}
