use crate::{Instance, SwapChainError, SwapChainFrame};

pub trait SwapChain {
    fn format(&self) -> wgpu_types::TextureFormat;

    fn size(&self) -> (u32, u32);

    fn recreate(&mut self, instance: &dyn Instance, width: u32, height: u32);

    fn get_current_frame(&self) -> Result<SwapChainFrame, SwapChainError>;
}
