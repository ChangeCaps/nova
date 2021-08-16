use crate::{Instance, SwapChainError, SwapChainFrame};
use std::ops::{Deref, DerefMut};

pub trait SwapChainTrait: Send + Sync + 'static {
    fn format(&self) -> wgpu_types::TextureFormat;

    fn size(&self) -> (u32, u32);

    fn recreate(&mut self, instance: &Instance, width: u32, height: u32);

    fn get_current_frame(&self) -> Result<SwapChainFrame, SwapChainError>;
}

pub struct SwapChain(pub(crate) Box<dyn SwapChainTrait>);

impl<T: SwapChainTrait> From<T> for SwapChain {
    #[inline]
    fn from(inner: T) -> Self {
        SwapChain(Box::new(inner))
    }
}

impl Deref for SwapChain {
    type Target = dyn SwapChainTrait;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl DerefMut for SwapChain {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut()
    }
}
