use crate::mesh::MeshData;
use nova_assets::Handle;
use nova_wgpu::{BindGroup, RenderPipeline};
use std::{borrow::Cow, ops::Range};

pub enum RenderCommand {
    SetPipeline(Handle<RenderPipeline>),
    SetMesh(Handle<MeshData>),
    SetBindGroup(u32, BindGroup, Cow<'static, [u32]>),
    DrawIndexed(Range<u32>, i32, Range<u32>),
}

#[derive(Default)]
pub struct RenderCommands {
    pub commands: Vec<RenderCommand>,
}

impl RenderCommands {
    #[inline]
    pub fn set_pipeline(&mut self, handle: impl Into<Handle<RenderPipeline>>) {
        self.commands
            .push(RenderCommand::SetPipeline(handle.into()));
    }

    #[inline]
    pub fn set_mesh(&mut self, handle: impl Into<Handle<MeshData>>) {
        self.commands.push(RenderCommand::SetMesh(handle.into()));
    }

    #[inline]
    pub fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: BindGroup,
        offsets: impl Into<Cow<'static, [u32]>>,
    ) {
        self.commands.push(RenderCommand::SetBindGroup(
            index,
            bind_group,
            offsets.into(),
        ));
    }

    #[inline]
    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.commands
            .push(RenderCommand::DrawIndexed(indices, base_vertex, instances));
    }
}
