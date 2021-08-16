use crossbeam::queue::SegQueue;
use glam::UVec2;
use nova_assets::{Assets, Handle};
use nova_core::{node::NodeId, system::System, world::SystemWorld};
use nova_render::render_texture::RenderTexture;
use nova_wgpu::{Instance, TextureFormat, TextureView};
use nova_window::Window;

pub const PRIMARY_VIEW: Handle<View> = Handle::from_u64(14687236);

#[derive(Default)]
pub struct ViewSystem;

impl System for ViewSystem {
    #[inline]
    fn init(&mut self, world: &mut SystemWorld) {
        let instance = world.read_resource::<Instance>().unwrap();
        let mut views = world.write_system::<Assets<View>>().unwrap();
        let mut render_textures = world.write_system::<Assets<RenderTexture>>().unwrap();
        let mut textures = world.write_system::<Assets<TextureView>>().unwrap();

        let render_texture =
            RenderTexture::new(&instance, TextureFormat::Rgba8UnormSrgb, (32, 32), 1);
        let view = render_texture.texture.view();

        let texture = render_textures.add(render_texture);
        textures.insert_untracked(texture.clone().cast(), view);

        views.insert_untracked(
            PRIMARY_VIEW,
            View {
                ty: ViewType::MainCamera,
                texture,
            },
        );
    }
}

pub enum ViewType {
    Camera(NodeId),
    MainCamera,
}

pub struct View {
    pub ty: ViewType,
    pub texture: Handle<RenderTexture>,
}

enum Command {
    RequestRedraw,
}

pub struct ViewWindow {
    queue: SegQueue<Command>,
    pub size: UVec2,
}

impl ViewWindow {
    #[inline]
    pub fn new(size: UVec2) -> Self {
        Self {
            queue: Default::default(),
            size,
        }
    }

    #[inline]
    pub fn dequeue(&self, outer: &dyn Window) {
        for _ in 0..self.queue.len() {
            match self.queue.pop().unwrap() {
                Command::RequestRedraw => outer.request_redraw(),
            }
        }
    }
}

impl Window for ViewWindow {
    #[inline]
    fn request_redraw(&self) {
        self.queue.push(Command::RequestRedraw);
    }

    fn size(&self) -> UVec2 {
        self.size
    }
}
