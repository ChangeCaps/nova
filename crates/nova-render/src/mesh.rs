use bytemuck::{cast_slice, Pod};
use nova_wgpu::Buffer;

#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct MeshData {
    pub vertices: Vec<u8>,
    pub indices: Vec<u32>,
    #[cfg_attr(feature = "serialize", serde(skip))]
    pub vertex_buffer: Option<Buffer>,
    #[cfg_attr(feature = "serialize", serde(skip))]
    pub index_buffer: Option<Buffer>,
}

#[derive(Clone, Debug)]
pub struct Mesh<V> {
    pub vertices: Vec<V>,
    pub indices: Vec<u32>,
}

impl<V> Default for Mesh<V> {
    #[inline]
    fn default() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }
}

impl<V: Pod> From<Mesh<V>> for MeshData {
    fn from(mesh: Mesh<V>) -> Self {
        MeshData {
            vertices: cast_slice(&mesh.vertices).to_vec(),
            indices: mesh.indices,
            vertex_buffer: None,
            index_buffer: None,
        }
    }
}

impl<V: Pod> From<&Mesh<V>> for MeshData {
    fn from(mesh: &Mesh<V>) -> Self {
        MeshData {
            vertices: cast_slice(&mesh.vertices).to_vec(),
            indices: mesh.indices.clone(),
            vertex_buffer: None,
            index_buffer: None,
        }
    }
}
