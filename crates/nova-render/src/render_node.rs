use std::{any::Any, collections::BTreeMap};

use glam::UVec2;
use nova_core::{Resources, World};
use nova_wgpu::{TextureFormat, TextureView};

#[allow(unused)]
pub trait RenderNode: Send + Sync + 'static {
    #[inline]
    fn run(&mut self, world: &World, resources: &Resources, target: &Target, data: &mut RenderData);
}

pub struct Target<'a> {
    pub view: &'a TextureView<'a>,
    pub size: UVec2,
    pub format: TextureFormat,
}

#[derive(Default)]
pub struct RenderData {
    data: BTreeMap<&'static str, Box<dyn Any + Send + Sync>>,
}

impl RenderData {
    #[inline]
    pub fn insert<T: Any + Send + Sync>(&mut self, ident: &'static str, data: T) {
        self.data.insert(ident, Box::new(data));
    }

    #[inline]
    pub fn get<T: Any + Send + Sync>(&self, ident: &'static str) -> Option<&T> {
        self.data.get(ident)?.downcast_ref()
    }

    #[inline]
    pub fn get_mut<T: Any + Send + Sync>(&mut self, ident: &'static str) -> Option<&mut T> {
        self.data.get_mut(ident)?.downcast_mut()
    }
}
