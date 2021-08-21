use std::{any::TypeId, collections::HashMap};

use egui::{Response, Ui};
use legion::{storage::Component, world::Entry};
use nova_inspect::Inspectable;

#[derive(Default)]
pub struct Inspectables {
    inspectables: HashMap<TypeId, fn(&mut Entry, &mut Ui) -> Option<Response>>,
}

impl Inspectables {
    #[inline]
    pub fn register<T: Inspectable + Component>(&mut self) {
        fn inspect<T: Inspectable + Component>(entry: &mut Entry, ui: &mut Ui) -> Option<Response> {
            entry.get_component_mut::<T>().ok()?.inspect(ui)
        }

        self.inspectables.insert(TypeId::of::<T>(), inspect::<T>);
    }
}
