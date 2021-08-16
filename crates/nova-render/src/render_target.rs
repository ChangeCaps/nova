use glam::UVec2;
use nova_wgpu::{
    Instance, SwapChain, SwapChainError, TextureDescriptor, TextureFormat, TextureView,
};

pub enum RenderTarget {
    Texture {
        view: TextureView<'static>,
        desc: TextureDescriptor<Option<&'static str>>,
    },
    SwapChain(SwapChain),
}

impl RenderTarget {
    #[inline]
    pub fn format(&self) -> TextureFormat {
        match self {
            Self::Texture { desc, .. } => desc.format,
            Self::SwapChain(sc) => sc.format(),
        }
    }

    #[inline]
    pub fn size(&self) -> UVec2 {
        match self {
            Self::Texture { desc, .. } => UVec2::new(desc.size.width, desc.size.height),
            Self::SwapChain(sc) => sc.size().into(),
        }
    }

    #[inline]
    pub fn view<O>(&self, f: impl FnOnce(&TextureView) -> O) -> Result<O, SwapChainError> {
        match self {
            Self::Texture { view, .. } => Ok(f(view)),
            Self::SwapChain(sc) => {
                let frame = sc.get_current_frame()?;
                let view = frame.output.view();

                Ok(f(&view))
            }
        }
    }

    #[inline]
    pub fn recreate(&mut self, instance: &Instance, width: u32, height: u32) {
        match self {
            Self::Texture { view, desc } => {
                desc.size.width = width;
                desc.size.height = height;

                let texture = instance.create_texture(desc);

                *view = texture.view();
            }
            Self::SwapChain(sc) => {
                sc.recreate(instance, width, height);
            }
        }
    }
}
