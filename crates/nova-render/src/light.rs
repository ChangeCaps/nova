use crate::color::Color;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PointLight {
    pub color: Color,
    pub intensity: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
