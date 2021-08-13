use glam::UVec2;
use nova_wgpu::*;

pub struct RenderTexture {
    pub texture: Texture,
    pub view: TextureView<'static>,
    pub format: TextureFormat,
    pub size: UVec2,
}

impl RenderTexture {
    #[inline]
    pub fn new(instance: &dyn Instance, format: TextureFormat, size: impl Into<UVec2>) -> Self {
        let size = size.into();

        let texture = instance.create_texture(&TextureDescriptor {
            label: Some("render_texture"),
            size: Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsage::RENDER_ATTACHMENT | TextureUsage::SAMPLED,
        });

        let view = texture.view();

        Self {
            texture,
            view,
            format,
            size,
        }
    }

    #[inline]
    pub fn resize(&mut self, instance: &dyn Instance, new_size: impl Into<UVec2>) {
        let new_size = new_size.into();

        self.size = new_size;

        self.texture = instance.create_texture(&TextureDescriptor {
            label: Some("render_texture"),
            size: Extent3d {
                width: self.size.x,
                height: self.size.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: self.format,
            usage: TextureUsage::RENDER_ATTACHMENT | TextureUsage::SAMPLED,
        });

        self.view = self.texture.view();
    }
}
