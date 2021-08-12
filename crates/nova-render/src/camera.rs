use glam::Mat4;
use nova_core::{
    component::Component,
    node::{Node, NodeId},
    system::System,
    world::World,
};

#[derive(Clone, Debug, Default)]
pub struct CameraSystem {
    pub main: Option<NodeId>,
}

impl System for CameraSystem {}

#[derive(Clone, Debug)]
pub struct MainCamera;

impl Component for MainCamera {
    #[inline]
    fn init(&mut self, node: &Node, world: &World) {
        world.system_mut::<CameraSystem>().unwrap().main = Some(node.id());
    }
}

#[derive(Clone, Debug)]
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
    Custom(Mat4),
}

impl Camera {
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
            Self::Custom(mat) => *mat,
        }
    }
}

impl Component for Camera {}
