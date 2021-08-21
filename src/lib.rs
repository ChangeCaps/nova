pub use nova_core as core;
#[doc(hidden)]
pub use nova_inspect;

pub mod prelude {
    pub use crate::export_app;
    pub use glam::{swizzles::*, *};
    pub use nova_assets::{Assets, Handle};
    pub use nova_core::*;
    pub use nova_derive::{Inspectable, Vertex};
    pub use nova_input::{key::Key, mouse_button::MouseButton, Input, InputPlugin};
    pub use nova_inspect::*;
    pub use nova_render::{
        camera::{Camera, Cameras, MainCamera},
        color::Color,
        component::MeshInstance,
        light::{AmbientLight, PointLight},
        mesh::{Mesh, MeshData},
        render_commands::RenderCommands,
        render_node::{RenderData, RenderNode, Target},
        render_settings::RenderSettings,
        render_target::RenderTarget,
        render_texture::RenderTexture,
        renderer::Renderer,
        vertex::Vertex,
        RenderPlugin,
    };
    pub use nova_transform::{
        component::{GlobalTransform, Parent, Transform},
        TransformPlugin,
    };
    pub use nova_wgpu as wgpu;
    pub use nova_window::{Window, Windows};
}

#[macro_export]
macro_rules! export_app {
    ($expr:expr) => {
        mod __nova_extern__ {
            use super::*;

            #[no_mangle]
            pub unsafe fn render_view(
                world: &mut $crate::prelude::World,
                resources: &mut $crate::prelude::Resources,
                target: &$crate::prelude::Target,
            ) {
                let mut renderer = resources.get_mut::<$crate::prelude::Renderer>().unwrap();
                renderer.render_view(world, resources, target);
            }

            #[no_mangle]
            pub unsafe fn export_app(
                mut app: $crate::prelude::AppBuilder,
                instance: $crate::prelude::wgpu::Instance,
                render_target: $crate::prelude::RenderTarget,
            ) -> $crate::prelude::App {
                app.insert_resource(instance);
                app.insert_resource(render_target);
                $expr(&mut app);
                app.build()
            }
        }
    };
}
