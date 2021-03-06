use std::{any::Any, fmt::Debug, sync::Arc};

pub trait TextureTrait: Debug + Send + Sync {
    fn view(&self) -> TextureView<'static>;

    fn any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct Texture(pub(crate) Box<dyn TextureTrait>);

impl Texture {
    #[inline]
    pub fn view(&self) -> TextureView<'static> {
        self.0.view()
    }
}

pub trait SwapChainTextureTrait {
    fn view(&self) -> TextureView;
}

pub struct SwapChainFrame {
    pub output: SwapChainTexture,
    pub suboptimal: bool,
}

#[derive(Clone, Debug)]
pub enum SwapChainError {
    Timeout,
    Outdated,
    Lost,
    OutOfMemory,
}

pub struct SwapChainTexture(pub(crate) Box<dyn SwapChainTextureTrait>);

impl SwapChainTexture {
    #[inline]
    pub fn view(&self) -> TextureView {
        self.0.view()
    }
}

#[derive(Clone)]
pub enum TextureView<'a> {
    Owned(Arc<dyn Any + Send + Sync>),
    Borrowed(&'a (dyn Any + Send + Sync)),
}

impl<'a> TextureView<'a> {
    #[inline]
    #[allow(unused)]
    pub(crate) fn any(&'a self) -> &'a dyn Any {
        match self {
            TextureView::Owned(any) => any.as_ref(),
            TextureView::Borrowed(any) => *any,
        }
    }
}
