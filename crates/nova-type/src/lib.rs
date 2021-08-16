#[allow(unused)]
use nova_core::{component::Component, resources::Resource, system::System};
#[allow(unused)]
use std::{any::type_name, collections::HashMap};

#[cfg(feature = "serialize")]
pub mod serde_traits;

#[derive(Clone, Default)]
pub struct TypeRegistry {
    #[cfg(feature = "serialize")]
    pub serialize_system: HashMap<&'static str, serde_traits::SerializeSystem>,
    #[cfg(feature = "serialize")]
    pub serialize_resource: HashMap<&'static str, serde_traits::SerializeResource>,
    #[cfg(feature = "serialize")]
    pub serialize_component: HashMap<&'static str, serde_traits::SerializeComponent>,
    #[cfg(feature = "serialize")]
    pub deserialize_system: HashMap<&'static str, serde_traits::DeserializeSystem>,
    #[cfg(feature = "serialize")]
    pub deserialize_resource: HashMap<&'static str, serde_traits::DeserializeResource>,
    #[cfg(feature = "serialize")]
    pub deserialize_component: HashMap<&'static str, serde_traits::DeserializeComponent>,
}

impl TypeRegistry {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(feature = "serialize")]
impl TypeRegistry {
    #[inline]
    pub fn register_serde_system<T: serde_traits::SerdeSystem + serde::de::DeserializeOwned>(
        &mut self,
    ) {
        unsafe fn serialize<T: serde_traits::SerdeSystem>(
            system: &dyn System,
        ) -> &dyn serde_traits::SerdeSystem {
            &*(system as *const _ as *const T)
        }

        fn deserialize<T: serde_traits::SerdeSystem + serde::de::DeserializeOwned>(
            deserializer: &mut dyn erased_serde::Deserializer,
        ) -> Result<Box<dyn System>, erased_serde::Error> {
            Ok(Box::new(T::deserialize(deserializer)?))
        }

        self.serialize_system
            .insert(type_name::<T>(), serialize::<T>);
        self.deserialize_system
            .insert(type_name::<T>(), deserialize::<T>);
    }

    #[inline]
    pub fn register_serde_resource<T: serde_traits::SerdeResource + serde::de::DeserializeOwned>(
        &mut self,
    ) {
        unsafe fn serialize<T: serde_traits::SerdeResource>(
            system: &dyn Resource,
        ) -> &dyn serde_traits::SerdeResource {
            &*(system as *const _ as *const T)
        }

        fn deserialize<T: serde_traits::SerdeResource + serde::de::DeserializeOwned>(
            deserializer: &mut dyn erased_serde::Deserializer,
        ) -> Result<Box<dyn Resource>, erased_serde::Error> {
            Ok(Box::new(T::deserialize(deserializer)?))
        }

        self.serialize_resource
            .insert(type_name::<T>(), serialize::<T>);
        self.deserialize_resource
            .insert(type_name::<T>(), deserialize::<T>);
    }

    #[inline]
    pub fn register_serde_component<
        T: serde_traits::SerdeComponent + serde::de::DeserializeOwned,
    >(
        &mut self,
    ) {
        unsafe fn serialize<T: serde_traits::SerdeComponent>(
            system: &dyn Component,
        ) -> &dyn serde_traits::SerdeComponent {
            &*(system as *const _ as *const T)
        }

        fn deserialize<T: serde_traits::SerdeComponent + serde::de::DeserializeOwned>(
            deserializer: &mut dyn erased_serde::Deserializer,
        ) -> Result<Box<dyn Component>, erased_serde::Error> {
            Ok(Box::new(T::deserialize(deserializer)?))
        }

        self.serialize_component
            .insert(type_name::<T>(), serialize::<T>);
        self.deserialize_component
            .insert(type_name::<T>(), deserialize::<T>);
    }
}
