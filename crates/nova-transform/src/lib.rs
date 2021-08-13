use nova_core::{plugin::Plugin, world::World};
use system::TransformSystem;

pub mod component;
pub mod system;

pub struct TransformPlugin;

impl Plugin for TransformPlugin {
    #[inline]
    fn build(self, world: &mut World) {
        world.register_system_now::<TransformSystem>();
    }
}
