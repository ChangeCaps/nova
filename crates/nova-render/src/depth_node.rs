use nova_core::{App, Resources, World};
use nova_wgpu::{Instance, TextureFormat};

use crate::{
    render_node::{RenderData, RenderNode, Target},
    render_settings::RenderSettings,
    render_texture::RenderTexture,
};

pub struct DepthNode;

impl DepthNode {
    pub const TEXTURE: &'static str = "main_depth_texture";
}

impl RenderNode for DepthNode {
    #[inline]
    fn run(
        &mut self,
        _world: &World,
        resources: &Resources,
        target: &Target,
        data: &mut RenderData,
    ) {
        let settings = resources.get::<RenderSettings>().unwrap();
        let instance = resources.get::<Instance>().unwrap();

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
