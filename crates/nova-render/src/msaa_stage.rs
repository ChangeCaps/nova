use nova_core::world::SystemWorld;
use nova_wgpu::Instance;

use crate::{
    render_settings::RenderSettings,
    render_stage::{RenderData, RenderStage, Target},
    render_texture::RenderTexture,
};

pub struct MsaaStage;

impl MsaaStage {
    pub const TEXTURE: &'static str = "main_msaa_texture";
}

impl RenderStage for MsaaStage {
    #[inline]
    fn render(&mut self, world: &mut SystemWorld, target: &Target, data: &mut RenderData) {
        let settings = world.resource_mut::<RenderSettings>().unwrap().clone();

        if settings.msaa <= 1 {
            return;
        }

        let instance = world.read_resource::<Instance>().unwrap();

        if let Some(render_texture) = data.get_mut::<RenderTexture>(Self::TEXTURE) {
            if render_texture.should_resize(target.size) {
                render_texture.resize(&instance, target.size);
            }
        } else {
            let texture = RenderTexture::new(&instance, target.format, target.size, settings.msaa);

            data.insert(Self::TEXTURE, texture);
        }
    }
}
