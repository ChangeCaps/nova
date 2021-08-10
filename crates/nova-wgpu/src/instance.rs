use crate::{
    bind_group::BindGroupLayoutDescriptor,
    buffer::Buffer,
    command_encoder::CommandEncoder,
    pipeline::{ShaderModule, ShaderModuleDescriptor},
    texture::{SwapChainError, SwapChainFrame, Texture},
    BindGroup, BindGroupDescriptor, BindGroupLayout, PipelineLayout, PipelineLayoutDescriptor,
    RenderPipeline, RenderPipelineDescriptor,
};

pub trait Instance {
    fn swapchain_format(&self) -> wgpu_types::TextureFormat;

    fn swapchain_size(&self) -> (u32, u32);

    fn recreate(&mut self, width: u32, height: u32);

    fn create_buffer(&self, desc: &wgpu_types::BufferDescriptor<Option<&str>>) -> Buffer;

    fn create_texture(&self, desc: &wgpu_types::TextureDescriptor<Option<&str>>) -> Texture;

    fn create_command_encoder(
        &self,
        desc: &wgpu_types::CommandEncoderDescriptor<Option<&str>>,
    ) -> CommandEncoder;

    fn create_bind_group_layout(&self, desc: &BindGroupLayoutDescriptor) -> BindGroupLayout;

    fn create_bind_group(&self, desc: &BindGroupDescriptor) -> BindGroup;

    fn create_shader_module(&self, desc: &ShaderModuleDescriptor) -> ShaderModule;

    fn create_pipeline_layout(&self, desc: &PipelineLayoutDescriptor) -> PipelineLayout;

    fn create_render_pipeline(&self, desc: &RenderPipelineDescriptor) -> RenderPipeline;

    fn submit(&self, command_encoder: CommandEncoder);

    fn get_current_frame(&self) -> Result<SwapChainFrame, SwapChainError>;

    fn write_buffer(&self, buffer: &Buffer, offset: u64, data: &[u8]);
}
