#[cfg(feature = "winit-impl")]
pub mod winit_impl;

use glam::UVec2;

pub trait Window: Send + Sync + 'static {
    fn request_redraw(&self);

    fn size(&self) -> UVec2;
}

pub struct Windows {
    window: Box<dyn Window>,
}

impl Windows {
    #[inline]
    pub fn new(window: impl Window) -> Self {
        Self {
            window: Box::new(window),
        }
    }

    #[inline]
    pub fn primary(&self) -> &dyn Window {
        self.window.as_ref()
    }
}
