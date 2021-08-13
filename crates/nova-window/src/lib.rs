#[cfg(feature = "winit-impl")]
pub mod winit_impl;

use glam::UVec2;
use nova_core::system::System;

pub trait Window: Send + Sync + 'static {
    fn request_redraw(&self);

    fn size(&self) -> UVec2;
}

pub struct WindowSystem {
    pub window: Box<dyn Window>,
}

impl WindowSystem {
    #[inline]
    pub fn new(window: impl Window) -> Self {
        Self {
            window: Box::new(window),
        }
    }
}

impl System for WindowSystem {}
