use glam::*;
use nova_wgpu::{VertexBufferLayout, VertexFormat};

pub trait Vertex {
    fn layout() -> VertexBufferLayout<'static>;
}

pub trait AsVertexFormat: Sized {
    const FORMAT: VertexFormat;
    const SIZE: u64 = std::mem::size_of::<Self>() as u64;
}

macro_rules! impl_as_vertex_format {
    ($ty:ty => $format:expr) => {
        impl AsVertexFormat for $ty {
            const FORMAT: VertexFormat = $format;
        }
    };
    ($ty:ty => $format:expr, $size:expr) => {
        impl AsVertexFormat for $ty {
            const FORMAT: VertexFormat = $format;
            const SIZE: u64 = $size;
        }
    };
}

impl_as_vertex_format!(f32 => VertexFormat::Float32);
impl_as_vertex_format!(i32 => VertexFormat::Sint32);
impl_as_vertex_format!(u32 => VertexFormat::Uint32);
impl_as_vertex_format!(Vec2 => VertexFormat::Float32x2, 16);
impl_as_vertex_format!(IVec2 => VertexFormat::Sint32x2, 16);
impl_as_vertex_format!(UVec2 => VertexFormat::Uint32x2, 16);
impl_as_vertex_format!(Vec3 => VertexFormat::Float32x3, 16);
impl_as_vertex_format!(IVec3 => VertexFormat::Sint32x3, 16);
impl_as_vertex_format!(UVec3 => VertexFormat::Uint32x3, 16);
impl_as_vertex_format!(Vec4 => VertexFormat::Float32x4);
impl_as_vertex_format!(IVec4 => VertexFormat::Sint32x4);
impl_as_vertex_format!(UVec4 => VertexFormat::Uint32x4);
