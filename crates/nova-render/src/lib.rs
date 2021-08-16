pub mod camera;
pub mod camera_stage;
pub mod color;
pub mod component;
pub mod depth_stage;
pub mod light;
pub mod light_stage;
pub mod mesh;
pub mod msaa_stage;
pub mod render_commands;
pub mod render_settings;
pub mod render_stage;
pub mod render_target;
pub mod render_texture;
pub mod renderer;
pub mod vertex;

use camera::CameraSystem;
use light::AmbientLight;
use mesh::MeshData;
use nova_assets::Assets;
use nova_core::plugin::Plugin;
use nova_wgpu::RenderPipeline;

pub use nova_derive::Vertex;
use render_settings::RenderSettings;
use render_texture::RenderTexture;
use renderer::RendererSystem;
pub use vertex::Vertex;

#[derive(Default)]
pub struct RenderPlugin(pub RenderSettings);

impl Plugin for RenderPlugin {
    fn build(self, world: &mut nova_core::world::World) {
        world.insert_resource(self.0);
        world.register_resource::<AmbientLight>();
        world.register_system::<Assets<RenderPipeline>>();
        world.register_system::<Assets<MeshData>>();
        world.register_system::<Assets<RenderTexture>>();
        world.register_system::<RendererSystem>();
        world.register_system::<CameraSystem>();
    }
}
