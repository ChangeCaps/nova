use glam::Mat4;
use nova_core::Entity;
use nova_inspect::Inspectable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default)]
pub struct Cameras {
    pub main: Option<Entity>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Inspectable)]
pub struct MainCamera;

#[derive(Clone, Debug, Serialize, Deserialize, Inspectable)]
pub enum Camera {
    Perspective {
        fov: f32,
        aspect: f32,
        near: f32,
    },
    Orthographic {
        left: f32,
        bottom: f32,
        right: f32,
        top: f32,
        near: f32,
        far: f32,
    },
}

impl Camera {
    #[inline]
    pub fn set_aspect(&mut self, new_aspect: f32) {
        match self {
            Self::Perspective { aspect, .. } => *aspect = new_aspect,
            _ => {}
        }
    }

    #[inline]
    pub fn proj_matrix(&self) -> Mat4 {
        match self {
            Self::Perspective { fov, aspect, near } => {
                Mat4::perspective_infinite_rh(*fov, *aspect, *near)
            }
            Self::Orthographic {
                left,
                bottom,
                right,
                top,
                near,
                far,
            } => Mat4::orthographic_rh(*left, *right, *bottom, *top, *near, *far),
        }
    }
}
