use glam::UVec2;
use nova_wgpu::*;

pub struct RenderTexture {
    pub texture: Texture,
    pub view: TextureView<'static>,
    pub desc: TextureDescriptor<Option<&'static str>>,
}

impl RenderTexture {
    #[inline]
    pub fn new(
        instance: &Instance,
        format: TextureFormat,
        size: impl Into<UVec2>,
        samples: u32,
    ) -> Self {
        let size = size.into();

        let desc = TextureDescriptor {
            label: Some("render_texture"),
            size: Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: samples,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsage::RENDER_ATTACHMENT | TextureUsage::SAMPLED,
        };

        let texture = instance.create_texture(&desc);

        let view = texture.view();

        Self {
            texture,
            view,
            desc,
        }
    }

    #[inline]
    pub fn size(&self) -> UVec2 {
        UVec2::new(self.desc.size.width, self.desc.size.height)
    }

    #[inline]
    pub fn should_resize(&self, size: impl Into<UVec2>) -> bool {
        let size = size.into();
        self.desc.size.width != size.x || self.desc.size.height != size.y
    }

    #[inline]
    pub fn resize(&mut self, instance: &Instance, new_size: impl Into<UVec2>) {
        let new_size = new_size.into();

        self.desc.size.width = new_size.x;
        self.desc.size.height = new_size.y;

        self.texture = instance.create_texture(&self.desc);

        self.view = self.texture.view();
    }
}
