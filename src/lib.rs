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
        component::MeshInstance,
        mesh::{Mesh, MeshData},
        render_commands::RenderCommands,
        render_system::RenderSystem,
        renderable::Renderable,
        vertex::Vertex,
        RenderPlugin,
    };
    pub use nova_transform::component::Transform;
    pub use nova_wgpu as wgpu;
}
