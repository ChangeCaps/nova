use glam::UVec2;

use crate::Window;

impl Window for winit::window::Window {
    fn request_redraw(&self) {
        self.request_redraw();
    }

    fn size(&self) -> UVec2 {
        let size = self.inner_size();

        UVec2::new(size.width, size.height)
    }
}
