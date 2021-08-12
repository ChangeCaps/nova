use crate::render_commands::RenderCommands;
use nova_core::{component::Component, node::Node, world::World};

#[allow(unused)]
pub trait Renderable: Component {
    #[inline]
    fn pre_render(&mut self, node: &Node, world: &World) {}

    #[inline]
    fn render(&mut self, node: &Node, world: &World, render_commands: &mut RenderCommands) {}
}
