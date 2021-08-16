use std::{any::TypeId, collections::BTreeMap};

use crate::render_stage::{RenderData, RenderStage, Target};
use nova_core::{system::System, world::SystemWorld};

struct Stage {
    render: Box<dyn RenderStage>,
    prev: Option<TypeId>,
    next: Option<TypeId>,
}

pub struct RendererSystem {
    data: RenderData,
    first: Option<TypeId>,
    last: Option<TypeId>,
    stages: BTreeMap<TypeId, Stage>,
}

impl Default for RendererSystem {
    #[inline]
    fn default() -> Self {
        Self {
            data: RenderData::default(),
            first: None,
            last: None,
            stages: BTreeMap::new(),
        }
    }
}

impl RendererSystem {
    #[inline]
    pub fn add_stage<T: RenderStage>(&mut self, stage: T) {
        let id = TypeId::of::<T>();

        if let Some(last) = &mut self.last {
            self.stages.get_mut(last).unwrap().next = Some(id);
            self.stages.insert(
                id,
                Stage {
                    render: Box::new(stage),
                    prev: Some(*last),
                    next: None,
                },
            );
            *last = id;
        } else {
            self.first = Some(id);
            self.last = Some(id);
            self.stages.insert(
                id,
                Stage {
                    render: Box::new(stage),
                    prev: None,
                    next: None,
                },
            );
        }
    }

    #[inline]
    pub fn add_stage_after<T: RenderStage, U: RenderStage>(&mut self, stage: T) {
        let id = TypeId::of::<T>();
        let before_id = TypeId::of::<U>();

        if let Some(before) = self.stages.get(&before_id) {
            let after_id = before.next;
            if let Some(after) = &after_id {
                self.stages.get_mut(after).unwrap().prev = Some(id);
            }

            self.stages.get_mut(&before_id).unwrap().next = Some(id);
            self.stages.insert(
                id,
                Stage {
                    render: Box::new(stage),
                    prev: Some(before_id),
                    next: after_id,
                },
            );
        } else {
            self.add_stage(stage);
        }
    }

    #[inline]
    pub fn render_view(&mut self, world: &mut SystemWorld, target: &Target) {
        let mut next = self.first;

        while let Some(id) = next {
            let stage = self.stages.get_mut(&id).unwrap();
            next = stage.next;

            stage.render.render(world, target, &mut self.data);
        }
    }
}

impl System for RendererSystem {}
