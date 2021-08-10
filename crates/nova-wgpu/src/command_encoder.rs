use std::any::Any;

use crate::render_pass::{RenderPass, RenderPassDescriptor};

pub trait CommandEncoderTrait {
    fn begin_render_pass<'a>(&'a mut self, desc: &RenderPassDescriptor<'a, '_>) -> RenderPass<'a>;

    fn any(&self) -> &dyn Any;
}

pub struct CommandEncoder(pub(crate) Box<dyn CommandEncoderTrait>);

impl CommandEncoder {
    #[inline]
    pub fn begin_render_pass<'a>(
        &'a mut self,
        desc: &RenderPassDescriptor<'a, '_>,
    ) -> RenderPass<'a> {
        self.0.begin_render_pass(desc)
    }
}
