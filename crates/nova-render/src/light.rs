use nova_core::component::Component;

use crate::color::Color;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct PointLight {
    pub color: Color,
    pub intensity: f32,
}

impl Component for PointLight {}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct AmbientLight {
    pub color: Color,
    pub intensity: f32,
}

impl Default for AmbientLight {
    #[inline]
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            intensity: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PointLightRaw {
    pub position: [f32; 3],
    pub intensity: f32,
    pub color: [f32; 4],
}

unsafe impl bytemuck::Zeroable for PointLightRaw {}
unsafe impl bytemuck::Pod for PointLightRaw {}
