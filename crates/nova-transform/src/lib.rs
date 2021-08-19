use component::{GlobalTransform, Parent, Transform};
use nova_core::{stage, AppBuilder, Plugin};
use system::child_system;

pub mod component;
pub mod system;

pub struct TransformPlugin;

impl Plugin for TransformPlugin {
    #[inline]
    fn build(self, app: &mut AppBuilder) {
        app.add_system_to_stage(stage::PRE_UPDATE, child_system())
            .register_component::<Transform>()
            .register_component::<GlobalTransform>()
            .register_component::<Parent>();

        #[cfg(feature = "editor")]
        app.add_editor_system_to_stage(stage::PRE_UPDATE, child_system());
    }
}
