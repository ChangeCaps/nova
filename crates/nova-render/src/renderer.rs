use std::collections::HashMap;

use nova_core::{App, Resources, World};

use crate::{
    camera_node::CameraNode,
    depth_node::DepthNode,
    light_node::LightNode,
    msaa_node::MsaaNode,
    render_node::{RenderData, RenderNode, Target},
};

pub struct Renderer {
    data: RenderData,
    order: Vec<String>,
    stages: HashMap<String, Vec<Box<dyn RenderNode>>>,
}

impl Default for Renderer {
    #[inline]
    fn default() -> Self {
        Self {
            data: RenderData::default(),
            order: Vec::new(),
            stages: HashMap::new(),
        }
    }
}

impl Renderer {
    pub const PRE_RENDER: &'static str = "pre_render";
    pub const RENDER: &'static str = "render";

    #[inline]
    pub fn new() -> Self {
        let mut renderer = Self::default();

        renderer.push_stage(Self::PRE_RENDER);
        renderer.push_stage(Self::RENDER);

        renderer
    }

    #[inline]
    pub fn add_default_nodes(&mut self) {
        self.add_node_to_stage(Self::PRE_RENDER, MsaaNode);
        self.add_node_to_stage(Self::PRE_RENDER, DepthNode);
        self.add_node_to_stage(Self::PRE_RENDER, LightNode::default());
        self.add_node_to_stage(Self::PRE_RENDER, CameraNode);
    }

    #[inline]
    pub fn render_view(&mut self, world: &World, resources: &Resources, target: &Target) {
        for stage in &self.order {
            for node in self.stages.get_mut(stage).unwrap() {
                node.run(world, resources, target, &mut self.data);
            }
        }
    }

    #[inline]
    pub fn push_stage(&mut self, stage: impl Into<String>) {
        let name = stage.into();

        self.order.push(name.clone());
        self.stages.insert(name, Vec::new());
    }

    #[inline]
    pub fn add_node_to_stage(&mut self, stage: &str, node: impl RenderNode) {
        self.stages.get_mut(stage).unwrap().push(Box::new(node));
    }
}
