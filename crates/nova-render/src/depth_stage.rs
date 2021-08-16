use nova_core::world::SystemWorld;
use nova_wgpu::{Instance, TextureFormat};

use crate::{
    render_settings::RenderSettings,
    render_stage::{RenderData, RenderStage, Target},
    render_texture::RenderTexture,
};

pub struct DepthStage;

impl DepthStage {
    pub const TEXTURE: &'static str = "main_depth_texture";
}

impl RenderStage for DepthStage {
    #[inline]
    fn render(&mut self, world: &mut SystemWorld, target: &Target, data: &mut RenderData) {
        let settings = world.resource_mut::<RenderSettings>().unwrap().clone();
        let instance = world.read_resource::<Instance>().unwrap();

        if let Some(render_texture) = data.get_mut::<RenderTexture>(Self::TEXTURE) {
            if render_texture.should_resize(target.size) {
                render_texture.resize(&instance, target.size);
            }
        } else {
            let texture = RenderTexture::new(
                &instance,
                TextureFormat::Depth24Plus,
                target.size,
                settings.msaa,
            );

            data.insert(Self::TEXTURE, texture);
        }
    }
}
