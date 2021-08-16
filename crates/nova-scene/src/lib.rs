use std::collections::HashMap;

use erased_serde::Deserializer;
use nova_core::{
    component::Component,
    node::{Node, NodeId},
    resources::Resource,
    system::System,
    world::{RefWorld, SystemWorld},
};
use nova_type::{
    serde_traits::{DeserializeComponent, DeserializeResource, DeserializeSystem},
    TypeRegistry,
};
use serde::{
    de::{DeserializeSeed, Error, Visitor},
    ser::{SerializeMap, SerializeStruct},
    Serialize,
};

#[derive(Default)]
pub struct Scene {
    pub systems: HashMap<String, Box<dyn System>>,
    pub resources: HashMap<String, Box<dyn Resource>>,
    pub nodes: HashMap<NodeId, HashMap<String, Box<dyn Component>>>,
}

impl Scene {
    #[inline]
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        type_registry: &TypeRegistry,
        deserializer: D,
    ) -> Result<Self, D::Error> {
        SceneDeserializer::new(type_registry).deserialize(deserializer)
    }

    #[inline]
    pub fn apply(self, world: &mut SystemWorld) {
        for (_, system) in self.systems {
            unsafe {
                world
                    .systems
                    .insert_raw(system.as_ref().type_name(), system)
            };
        }

        for (_, resource) in self.resources {
            unsafe {
                world
                    .resources
                    .insert_raw(resource.as_ref().type_name(), resource)
            };
        }

        for (id, components) in self.nodes {
            let mut node = Node::new("loaded");

            for (_, component) in components {
                unsafe { node.insert_raw(component.as_ref().type_name(), component) };
            }

            world.insert_node(id, node);
        }
    }
}

pub struct WorldSerializer<'a, 'b> {
    pub world: &'a RefWorld<'b>,
    pub type_registry: &'a TypeRegistry,
}

impl Serialize for WorldSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_struct("Scene", 3)?;

        seq.serialize_field(
            "systems",
            &WorldSystemsSerializer {
                world: self.world,
                type_registry: self.type_registry,
            },
        )?;

        seq.serialize_field(
            "resources",
            &WorldResourcesSerializer {
                world: self.world,
                type_registry: self.type_registry,
            },
        )?;

        seq.serialize_field(
            "nodes",
            &WorldNodesSerializer {
                world: self.world,
                type_registry: self.type_registry,
            },
        )?;

        seq.end()
    }
}

struct WorldSystemsSerializer<'a, 'b> {
    world: &'a RefWorld<'b>,
    type_registry: &'a TypeRegistry,
}

impl Serialize for WorldSystemsSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_map(None)?;

        for (name, system) in self.world.systems.iter() {
            if let Some(serialize) = self.type_registry.serialize_system.get(name) {
                let system = unsafe { serialize(&*system) };
                seq.serialize_entry(name, system)?;
            }
        }

        seq.end()
    }
}

struct WorldResourcesSerializer<'a, 'b> {
    world: &'a RefWorld<'b>,
    type_registry: &'a TypeRegistry,
}

impl Serialize for WorldResourcesSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_map(None)?;

        for (name, resource) in self.world.resources.iter() {
            if let Some(serialize) = self.type_registry.serialize_resource.get(name) {
                let resource = unsafe { serialize(&*resource) };
                seq.serialize_entry(name, resource)?;
            }
        }

        seq.end()
    }
}

struct WorldNodesSerializer<'a, 'b> {
    world: &'a RefWorld<'b>,
    type_registry: &'a TypeRegistry,
}

impl Serialize for WorldNodesSerializer<'_, '_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_map(None)?;

        for (name, node) in self.world.nodes.iter() {
            seq.serialize_entry(
                name,
                &WorldComponentsSerializer {
                    node,
                    type_registry: self.type_registry,
                },
            )?;
        }

        seq.end()
    }
}

struct WorldComponentsSerializer<'a> {
    node: &'a Node,
    type_registry: &'a TypeRegistry,
}

impl Serialize for WorldComponentsSerializer<'_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_map(None)?;

        for (name, component) in self.node.components.iter() {
            if let Some(serialize) = self.type_registry.serialize_component.get(name) {
                let component = unsafe { serialize(&*component) };
                seq.serialize_entry(name, component)?;
            }
        }

        seq.end()
    }
}

pub struct SceneDeserializer<'a> {
    pub type_registry: &'a TypeRegistry,
}

impl<'a> SceneDeserializer<'a> {
    #[inline]
    pub fn new(type_registry: &'a TypeRegistry) -> Self {
        Self { type_registry }
    }
}

#[derive(serde::Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
enum Field {
    Systems,
    Resources,
    Nodes,
}

impl<'a, 'de> DeserializeSeed<'de> for SceneDeserializer<'a> {
    type Value = Scene;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["systems", "resources", "nodes"];

        struct SceneVisitor<'a> {
            type_registry: &'a TypeRegistry,
        }

        impl<'de> Visitor<'de> for SceneVisitor<'_> {
            type Value = Scene;

            #[inline]
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "Scene")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut systems = None;
                let mut resources = None;
                let mut nodes = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Systems => {
                            if systems.is_some() {
                                return Err(Error::duplicate_field("systems"));
                            }
                            systems = Some(map.next_value_seed(SystemsDeserializer {
                                type_registry: self.type_registry,
                            })?);
                        }
                        Field::Resources => {
                            if resources.is_some() {
                                return Err(Error::duplicate_field("resources"));
                            }
                            resources = Some(map.next_value_seed(ResourcesDeserializer {
                                type_registry: self.type_registry,
                            })?);
                        }
                        Field::Nodes => {
                            if nodes.is_some() {
                                return Err(Error::duplicate_field("nodes"));
                            }
                            nodes = Some(map.next_value_seed(NodesDeserializer {
                                type_registry: self.type_registry,
                            })?);
                        }
                    }
                }

                let systems = systems.ok_or_else(|| Error::missing_field("systems"))?;
                let resources = resources.ok_or_else(|| Error::missing_field("systems"))?;
                let nodes = nodes.ok_or_else(|| Error::missing_field("systems"))?;
                Ok(Scene {
                    systems,
                    resources,
                    nodes,
                })
            }
        }

        deserializer.deserialize_struct(
            "",
            FIELDS,
            SceneVisitor {
                type_registry: self.type_registry,
            },
        )
    }
}

struct SystemsDeserializer<'a> {
    type_registry: &'a TypeRegistry,
}

impl<'a, 'de> DeserializeSeed<'de> for SystemsDeserializer<'a> {
    type Value = HashMap<String, Box<dyn System>>;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SystemsVisitor<'a> {
            type_registry: &'a TypeRegistry,
        }

        impl<'a, 'de> Visitor<'de> for SystemsVisitor<'a> {
            type Value = HashMap<String, Box<dyn System>>;

            #[inline]
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "systems")
            }

            #[inline]
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut systems = HashMap::with_capacity(map.size_hint().unwrap_or(0));

                while let Some(key) = map.next_key::<String>()? {
                    if let Some(deserialize) =
                        self.type_registry.deserialize_system.get(key.as_str())
                    {
                        let system = map.next_value_seed(SystemDeserializer { deserialize })?;
                        systems.insert(key, system);
                    }
                }

                Ok(systems)
            }
        }

        deserializer.deserialize_map(SystemsVisitor {
            type_registry: self.type_registry,
        })
    }
}

struct ResourcesDeserializer<'a> {
    type_registry: &'a TypeRegistry,
}

impl<'a, 'de> DeserializeSeed<'de> for ResourcesDeserializer<'a> {
    type Value = HashMap<String, Box<dyn Resource>>;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResourcesVisitor<'a> {
            type_registry: &'a TypeRegistry,
        }

        impl<'a, 'de> Visitor<'de> for ResourcesVisitor<'a> {
            type Value = HashMap<String, Box<dyn Resource>>;

            #[inline]
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "systems")
            }

            #[inline]
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut resources = HashMap::with_capacity(map.size_hint().unwrap_or(0));

                while let Some(key) = map.next_key::<String>()? {
                    if let Some(deserialize) =
                        self.type_registry.deserialize_resource.get(key.as_str())
                    {
                        let resource = map.next_value_seed(ResourceDeserializer { deserialize })?;
                        resources.insert(key, resource);
                    }
                }

                Ok(resources)
            }
        }

        deserializer.deserialize_map(ResourcesVisitor {
            type_registry: self.type_registry,
        })
    }
}

struct NodesDeserializer<'a> {
    type_registry: &'a TypeRegistry,
}

impl<'a, 'de> DeserializeSeed<'de> for NodesDeserializer<'a> {
    type Value = HashMap<NodeId, HashMap<String, Box<dyn Component>>>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct NodesVisitor<'a> {
            type_registry: &'a TypeRegistry,
        }

        impl<'a, 'de> Visitor<'de> for NodesVisitor<'a> {
            type Value = HashMap<NodeId, HashMap<String, Box<dyn Component>>>;

            #[inline]
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "systems")
            }

            #[inline]
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut nodes = HashMap::with_capacity(map.size_hint().unwrap_or(0));

                while let Some(key) = map.next_key::<NodeId>()? {
                    let components = map.next_value_seed(ComponentsDeserializer {
                        type_registry: self.type_registry,
                    })?;

                    nodes.insert(key, components);
                }

                Ok(nodes)
            }
        }

        deserializer.deserialize_map(NodesVisitor {
            type_registry: self.type_registry,
        })
    }
}

struct ComponentsDeserializer<'a> {
    type_registry: &'a TypeRegistry,
}

impl<'a, 'de> DeserializeSeed<'de> for ComponentsDeserializer<'a> {
    type Value = HashMap<String, Box<dyn Component>>;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ComponentsVisitor<'a> {
            type_registry: &'a TypeRegistry,
        }

        impl<'a, 'de> Visitor<'de> for ComponentsVisitor<'a> {
            type Value = HashMap<String, Box<dyn Component>>;

            #[inline]
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "components")
            }

            #[inline]
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut components = HashMap::with_capacity(map.size_hint().unwrap_or(0));

                while let Some(key) = map.next_key::<String>()? {
                    if let Some(deserialize) =
                        self.type_registry.deserialize_component.get(key.as_str())
                    {
                        let component =
                            map.next_value_seed(ComponentDeserializer { deserialize })?;
                        components.insert(key, component);
                    }
                }

                Ok(components)
            }
        }

        deserializer.deserialize_map(ComponentsVisitor {
            type_registry: self.type_registry,
        })
    }
}

struct SystemDeserializer<'a> {
    deserialize: &'a DeserializeSystem,
}

impl<'a, 'de> DeserializeSeed<'de> for SystemDeserializer<'a> {
    type Value = Box<dyn System>;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok((self.deserialize)(&mut <dyn Deserializer>::erase(deserializer)).unwrap())
    }
}

struct ResourceDeserializer<'a> {
    deserialize: &'a DeserializeResource,
}

impl<'a, 'de> DeserializeSeed<'de> for ResourceDeserializer<'a> {
    type Value = Box<dyn Resource>;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok((self.deserialize)(&mut <dyn Deserializer>::erase(deserializer)).unwrap())
    }
}
struct ComponentDeserializer<'a> {
    deserialize: &'a DeserializeComponent,
}

impl<'a, 'de> DeserializeSeed<'de> for ComponentDeserializer<'a> {
    type Value = Box<dyn Component>;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok((self.deserialize)(&mut <dyn Deserializer>::erase(deserializer)).unwrap())
    }
}
