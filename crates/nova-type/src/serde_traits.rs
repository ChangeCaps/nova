use erased_serde::{serialize_trait_object, Deserializer, Error};
use nova_core::{component::Component, resources::Resource, system::System};

pub trait SerdeSystem: System + erased_serde::Serialize {}

impl<T: System + erased_serde::Serialize> SerdeSystem for T {}

serialize_trait_object!(SerdeSystem);

pub type SerializeSystem = unsafe fn(&dyn System) -> &dyn SerdeSystem;
pub type DeserializeSystem = fn(&mut dyn Deserializer) -> Result<Box<dyn System>, Error>;

pub trait SerdeResource: Resource + erased_serde::Serialize {}

impl<T: Resource + erased_serde::Serialize> SerdeResource for T {}

serialize_trait_object!(SerdeResource);

pub type SerializeResource = unsafe fn(&dyn Resource) -> &dyn SerdeResource;
pub type DeserializeResource = fn(&mut dyn Deserializer) -> Result<Box<dyn Resource>, Error>;

pub trait SerdeComponent: Component + erased_serde::Serialize {}

impl<T: Component + erased_serde::Serialize> SerdeComponent for T {}

serialize_trait_object!(SerdeComponent);

pub type SerializeComponent = unsafe fn(&dyn Component) -> &dyn SerdeComponent;
pub type DeserializeComponent = fn(&mut dyn Deserializer) -> Result<Box<dyn Component>, Error>;
