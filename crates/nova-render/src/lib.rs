pub mod camera;
pub mod camera_node;
pub mod color;
pub mod component;
pub mod depth_node;
pub mod light;
pub mod light_node;
pub mod mesh;
pub mod msaa_node;
pub mod render_commands;
pub mod render_node;
pub mod render_settings;
pub mod render_target;
pub mod render_texture;
pub mod renderer;
pub mod vertex;

use camera::{Camera, Cameras, MainCamera};
use camera_node::camera_system;
use component::MeshInstance;
use light::{AmbientLight, PointLight};
use mesh::MeshData;
use nova_assets::AssetsAppExt;
use nova_core::{plugin::Plugin, stage::PRE_UPDATE, AppBuilder};
use nova_wgpu::RenderPipeline;

pub use nova_derive::Vertex;
use render_settings::RenderSettings;
use render_texture::RenderTexture;
use renderer::Renderer;
pub use vertex::Vertex;

#[derive(Default)]
pub struct RenderPlugin(pub RenderSettings);

impl Plugin for RenderPlugin {
    fn build(self, app: &mut AppBuilder) {
        let mut renderer = Renderer::new();

        renderer.add_default_nodes();

        app.insert_resource(self.0)
            .add_system_to_stage(PRE_UPDATE, camera_system())
            .register_asset::<RenderPipeline>()
            .register_asset::<MeshData>()
            .register_asset::<RenderTexture>()
            .register_resource::<AmbientLight>()
            .insert_resource(renderer)
            .register_resource::<Cameras>()
            .register_component::<Camera>()
            .register_component::<MainCamera>()
            .register_component::<MeshInstance>()
            .register_component::<PointLight>();

        #[cfg(feature = "editor")]
        app.add_editor_system_to_stage(PRE_UPDATE, camera_system());
    }
}
