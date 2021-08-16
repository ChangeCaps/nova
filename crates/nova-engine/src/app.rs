use nova_core::world::{World, WorldData};
use nova_input::{key::Key, mouse_button::MouseButton, Input, InputPlugin, Mouse};
use nova_render::{render_stage::Target, render_target::RenderTarget, renderer::RendererSystem};
use nova_wgpu::{
    instance::Instance,
    wgpu_impl::{WgpuInstance, WgpuSwapChain},
    SwapChain,
};
use nova_window::Windows;
use winit::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub async fn init_wgpu(window: &Window) -> (Instance, SwapChain) {
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let surface = unsafe { instance.create_surface(window) };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
        })
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        )
        .await
        .unwrap();

    let size = window.inner_size();

    let desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
    };

    let sc = WgpuSwapChain::new(&device, surface, desc);

    (WgpuInstance::new(device, queue).into(), sc.into())
}

pub struct App {
    title: String,
    world: WorldData,
}

impl App {
    #[inline]
    pub fn new() -> Self {
        Self {
            title: String::from("Nova App"),
            world: WorldData::new(),
        }
    }

    #[inline]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    #[inline]
    pub fn world(&mut self) -> World {
        self.world.world()
    }

    #[inline]
    pub fn run(mut self) -> ! {
        simple_logger::SimpleLogger::new()
            .with_module_level("gfx", log::LevelFilter::Error)
            .with_module_level("wgpu", log::LevelFilter::Error)
            .with_module_level("winit", log::LevelFilter::Error)
            .with_module_level("naga", log::LevelFilter::Error)
            .init()
            .unwrap();

        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new().with_title(&self.title);
        let window = window_builder.build(&event_loop).unwrap();

        let (instance, sc) = pollster::block_on(init_wgpu(&window));

        let mut world = self.world.world();
        world.with_plugin(InputPlugin);

        world.insert_resource(instance);
        world.insert_resource(RenderTarget::SwapChain(sc));
        world.insert_resource(Windows::new(window));
        drop(world);

        self.world.dequeue();

        self.world.running = true;

        self.world.init();

        self.world.dequeue();

        event_loop.run(move |event, _, control_flow| match event {
            Event::MainEventsCleared => {
                self.world
                    .resources
                    .get_mut::<Windows>()
                    .unwrap()
                    .primary()
                    .request_redraw();
            }
            Event::RedrawRequested(_) => {
                self.world.pre_update();
                self.world.update();
                self.world.post_update();

                let mut world = self.world.system_world();

                if let Some(mut renderer) = world.systems.write::<RendererSystem>() {
                    let swap_chain = world.resources.read::<SwapChain>().unwrap();
                    let frame = swap_chain.get_current_frame().unwrap();

                    let target = Target {
                        view: &frame.output.view(),
                        size: swap_chain.size().into(),
                        format: swap_chain.format(),
                    };

                    drop(swap_chain);

                    renderer.render_view(&mut world, &target);
                }
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(size) => {
                    let instance = self.world.resources.read::<Instance>().unwrap();
                    let mut target = self.world.resources.write::<RenderTarget>().unwrap();

                    target.recreate(&instance, size.width, size.height);
                }
                WindowEvent::ScaleFactorChanged {
                    new_inner_size: size,
                    ..
                } => {
                    let instance = self.world.resources.read::<Instance>().unwrap();
                    let mut target = self.world.resources.write::<RenderTarget>().unwrap();

                    target.recreate(&instance, size.width, size.height);
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode,
                            state,
                            ..
                        },
                    ..
                } => {
                    let input = self.world.systems.get_mut::<Input<Key>>().unwrap();

                    if let Some(keycode) = virtual_keycode {
                        match state {
                            ElementState::Pressed => {
                                input.press(keycode.into());
                            }
                            ElementState::Released => {
                                input.release(keycode.into());
                            }
                        }
                    }
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    let input = self.world.systems.get_mut::<Input<MouseButton>>().unwrap();

                    match state {
                        ElementState::Pressed => {
                            input.press(button.into());
                        }
                        ElementState::Released => {
                            input.release(button.into());
                        }
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let mouse = self.world.resources.get_mut::<Mouse>().unwrap();

                    mouse.position.x = position.x as f32;
                    mouse.position.y = position.y as f32;
                }
                _ => {}
            },
            _ => {}
        });
    }
}
