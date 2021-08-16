pub use nova_core as core;

pub mod prelude {
    pub use crate::{export_world, register_types};
    pub use glam::{swizzles::*, *};
    pub use nova_assets::{Assets, Handle};
    pub use nova_core::{
        component::Component,
        node::{Node, NodeId},
        plugin::Plugin,
        system::System,
        world::{ComponentWorld, RefWorld, SystemWorld, World, WorldData},
        Read, Write,
    };
    pub use nova_derive::Vertex;
    pub use nova_input::{key::Key, mouse_button::MouseButton, Input, InputPlugin};
    pub use nova_render::{
        camera::{Camera, CameraSystem, MainCamera},
        color::Color,
        component::MeshInstance,
        light::{AmbientLight, PointLight},
        mesh::{Mesh, MeshData},
        render_commands::RenderCommands,
        render_settings::RenderSettings,
        render_stage::{RenderData, RenderStage},
        render_texture::RenderTexture,
        renderer::RendererSystem,
        vertex::Vertex,
        RenderPlugin,
    };
    pub use nova_transform::{
        component::{GlobalTransform, Parent, Transform},
        TransformPlugin,
    };
    pub use nova_type::TypeRegistry;
    pub use nova_wgpu as wgpu;
    pub use nova_window::{Window, Windows};
}

#[macro_export]
macro_rules! register_types {
    ($expr:expr) => {
        mod __register_types {
            use super::*;

            #[no_mangle]
            unsafe fn register_types(type_registry: &mut TypeRegistry) {
                $expr(type_registry);
            }
        }
    };
}

#[macro_export]
macro_rules! export_world {
    ($expr:expr) => {
        mod __export_world {
            use super::*;

            #[no_mangle]
            unsafe fn export_world(world: &mut World) {
                $expr(world);
            }
        }
    };
}
