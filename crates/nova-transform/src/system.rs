use std::collections::BTreeMap;

use nova_core::{node::NodeId, system::System, world::SystemWorld};

use crate::component::{GlobalTransform, Parent, Transform};

#[inline]
fn update_transform(
    children: &BTreeMap<NodeId, Vec<NodeId>>,
    world: &SystemWorld,
    node: NodeId,
    parent: Transform,
) {
    let node = world.node(&node).unwrap();

    if let Some(transform) = node.read_component::<Transform>() {
        let transform = parent * transform.clone();

        if let Some(mut global_transform) = node.write_component::<GlobalTransform>() {
            global_transform.0 = transform.clone();
        }

        if let Some(child_nodes) = children.get(&node.id()) {
            for child in child_nodes {
                update_transform(children, world, *child, transform.clone());
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct TransformSystem;

impl System for TransformSystem {
    #[inline]
    fn post_update(&mut self, world: &mut SystemWorld) {
        let mut children: BTreeMap<NodeId, Vec<NodeId>> = BTreeMap::new();

        for node in world.nodes() {
            if let Some(parent) = node.read_component::<Parent>() {
                children.entry(parent.0).or_default().push(node.id());
            }
        }

        for node in world.nodes() {
            if !node.contains::<Parent>() {
                if let Some(transform) = node.read_component::<Transform>() {
                    if let Some(mut global_transform) = node.write_component::<GlobalTransform>() {
                        **global_transform = transform.clone();
                    }

                    if let Some(child_nodes) = children.get(&node.id()) {
                        for child in child_nodes {
                            update_transform(&children, world, *child, transform.clone());
                        }
                    }
                }
            }
        }
    }
}
