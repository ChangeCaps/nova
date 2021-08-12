pub mod camera;
pub mod component;
pub mod mesh;
pub mod render_commands;
pub mod render_system;
pub mod renderable;
pub mod vertex;

use camera::CameraSystem;
use component::MeshInstance;
use mesh::MeshData;
use nova_assets::Assets;
use nova_core::plugin::Plugin;
use nova_wgpu::RenderPipeline;

pub use nova_derive::Vertex;
use render_system::RenderSystem;
pub use vertex::Vertex;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(self, world: &mut nova_core::world::World) {
        let mut render_system = RenderSystem::default();
        render_system.register_renderable::<MeshInstance>();

        world.register_system::<Assets<RenderPipeline>>();
        world.register_system::<Assets<MeshData>>();
        world.insert_system(render_system);
        world.register_system::<CameraSystem>();
    }
}
