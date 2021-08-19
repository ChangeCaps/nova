use std::collections::HashMap;

use nova_core::{
    component, maybe_changed, query::EntityFilter, systems::Runnable, world::SubWorld, Entity,
    IntoQuery, Query, SystemBuilder,
};

use crate::component::{Children, GlobalTransform, Parent, Transform};

fn update_child<T: EntityFilter, U: EntityFilter>(
    world: &mut SubWorld,
    transforms: &mut Query<(&Transform, &mut GlobalTransform), T>,
    has_children: &mut Query<&Children, U>,
    entity: Entity,
    parent_transform: Transform,
) {
    if let Ok((transform, global_transform)) = transforms.get_mut(world, entity) {
        let transform = parent_transform.mul_transform(transform);
        global_transform.0 = transform.clone();

        if let Ok(children) = has_children.get(world, entity) {
            for child in children.children.clone() {
                update_child(world, transforms, has_children, child, transform.clone());
            }
        }
    }
}

pub fn child_system() -> impl Runnable {
    SystemBuilder::new("child_system")
        .with_query(<(Entity, &Parent)>::query().filter(maybe_changed::<Parent>()))
        .with_query(<&mut Children>::query())
        .with_query(<Entity>::query().filter(
            component::<Transform>() & component::<GlobalTransform>() & !component::<Parent>(),
        ))
        .with_query(<&Children>::query())
        .with_query(<(&Transform, &mut GlobalTransform)>::query())
        .build(
            |commands, world, _resources, (children, parents, roots, has_children, transforms)| {
                let parent_pairs = children
                    .iter(world)
                    .map(|(e, p)| (*e, p.0))
                    .collect::<Vec<_>>();

                let mut children: HashMap<Entity, Children> = HashMap::new();

                for (entity, parent) in parent_pairs {
                    if let Ok(children) = parents.get_mut(&mut *world, parent) {
                        if !children.children.contains(&entity) {
                            children.children.push(entity);
                        }
                    } else {
                        children.entry(parent).or_default().children.push(entity);
                    }
                }

                for (entity, children) in children {
                    commands.add_component(entity, children);
                }

                let roots: Vec<_> = roots.iter(&*world).cloned().collect();

                for root in roots {
                    let (transform, global_transform) = transforms.get_mut(world, root).unwrap();

                    let transform = transform.clone();
                    global_transform.0 = transform.clone();

                    if let Ok(children) = has_children.get(world, root) {
                        for child in children.children.clone() {
                            update_child(world, transforms, has_children, child, transform.clone());
                        }
                    }
                }
            },
        )
}
