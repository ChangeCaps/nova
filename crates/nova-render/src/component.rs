use std::collections::BTreeMap;

use nova_assets::Handle;
use nova_inspect::Inspectable;
use nova_wgpu::*;

use crate::mesh::MeshData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Inspectable)]
pub struct MeshInstance {
    pub mesh_data: Handle<MeshData>,
    pub pipeline: Handle<RenderPipeline>,
    #[serde(skip)]
    #[inspectable(ignore)]
    pub bindings: BTreeMap<u32, BindGroup>,
}
