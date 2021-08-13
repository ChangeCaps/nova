pub use nova_core as core;

pub mod prelude {
    pub use glam::{swizzles::*, *};
    pub use nova_assets::{Assets, Handle};
    pub use nova_core::{
        component::Component,
        node::{Node, NodeId},
        plugin::Plugin,
        system::System,
        world::World,
        Read, Write,
    };
    pub use nova_derive::Vertex;
    pub use nova_render::{
        camera::{Camera, CameraSystem, MainCamera},
        color::Color,
        component::MeshInstance,
        light::PointLight,
        mesh::{Mesh, MeshData},
        render_commands::RenderCommands,
        render_system::RenderSystem,
        render_texture::RenderTexture,
        renderable::Renderable,
        vertex::Vertex,
        RenderPlugin,
    };
    pub use nova_transform::{
        component::{Parent, Transform},
        TransformPlugin,
    };
    pub use nova_wgpu as wgpu;
    pub use nova_window::{Window, WindowSystem};
}
