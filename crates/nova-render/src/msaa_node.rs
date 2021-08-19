use nova_core::{App, Resources, World};
use nova_wgpu::Instance;

use crate::{
    render_node::{RenderData, RenderNode, Target},
    render_settings::RenderSettings,
    render_texture::RenderTexture,
};

pub struct MsaaNode;

impl MsaaNode {
    pub const TEXTURE: &'static str = "main_msaa_texture";
}

impl RenderNode for MsaaNode {
    #[inline]
    fn run(
        &mut self,
        _world: &World,
        resources: &Resources,
        target: &Target,
        data: &mut RenderData,
    ) {
        let settings = resources.get::<RenderSettings>().unwrap();

        if settings.msaa <= 1 {
            return;
        }

        let instance = resources.get::<Instance>().unwrap();

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
