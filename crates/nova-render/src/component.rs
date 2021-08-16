use std::collections::BTreeMap;

use nova_assets::Handle;
use nova_core::{component::Component, node::NodeId};
use nova_wgpu::*;

use crate::mesh::MeshData;

#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct MeshInstance {
    pub mesh_data: Handle<MeshData>,
    pub pipeline: Handle<RenderPipeline>,
    #[cfg_attr(feature = "serialize", serde(skip))]
    pub bindings: BTreeMap<u32, BindGroup>,
    pub camera: Option<NodeId>,
}

impl Component for MeshInstance {}
