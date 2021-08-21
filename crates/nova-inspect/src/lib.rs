pub mod base_impl;

#[doc(hidden)]
pub use egui;
pub use nova_derive::Inspectable;

use egui::{Response, Ui};

pub trait Inspectable {
    fn name(&self) -> &'static str;
    fn inspect(&mut self, ui: &mut Ui) -> Option<Response>;
}
