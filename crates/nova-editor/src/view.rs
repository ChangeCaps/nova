use crossbeam::queue::SegQueue;
use glam::UVec2;
use nova_assets::{Assets, Handle};
use nova_core::{stage, Entity, Plugin, Resources, World};
use nova_render::{render_node::Target, render_texture::RenderTexture};
use nova_wgpu::{Instance, TextureFormat, TextureView};
use nova_window::Window;

use crate::{load::Game, scenes::Scenes};

pub const PRIMARY_VIEW: Handle<View> = Handle::from_u64(14687236);

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(self, app: &mut nova_core::AppBuilder) {
        app.add_thread_local_to_stage(stage::UPDATE, render_view_system);

        let instance = app.resources.get::<Instance>().unwrap();
        let mut views = app.resources.get_mut::<Assets<View>>().unwrap();
        let mut render_textures = app.resources.get_mut::<Assets<RenderTexture>>().unwrap();
        let mut textures = app.resources.get_mut::<Assets<TextureView>>().unwrap();

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

pub fn render_view_system(_world: &mut World, resources: &mut Resources) {
    let views = resources.get::<Assets<View>>().unwrap();
    let render_textures = resources.get::<Assets<RenderTexture>>().unwrap();
    let game = resources.get::<Game>().unwrap();
    let mut scenes = resources.get_mut::<Scenes>().unwrap();

    if let Some(game) = &game.loaded {
        let scenes = &mut *scenes;

        if let Some(open) = &scenes.open {
            let scene = &mut **scenes.instances.get_mut(open).unwrap();

            for view in views.iter() {
                let render_texture = render_textures.get(&view.texture).unwrap();

                let target = Target {
                    view: &render_texture.view,
                    depth: None,
                    format: render_texture.desc.format,
                    size: render_texture.size(),
                };

                let res = unsafe {
                    game.render_view(&mut scene.app.world, &mut scene.app.resources, &target)
                };

                if let Err(err) = res {
                    log::error!("failed to render view: {}", err);
                }
            }
        }
    }
}

pub enum ViewType {
    Camera(Entity),
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
