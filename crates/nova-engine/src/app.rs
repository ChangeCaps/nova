use nova_core::world::World;
use nova_wgpu::{
    gpu_system::GpuSystem,
    instance::Instance,
    wgpu_impl::{WgpuInstance, WgpuSwapChain},
    SwapChain,
};
use nova_window::WindowSystem;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub async fn init_wgpu(window: &Window) -> (impl Instance, impl SwapChain) {
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

    (WgpuInstance::new(device, queue), sc)
}

pub struct App {}

impl App {
    #[inline]
    pub fn new() -> Self {
        Self {}
    }

    #[inline]
    pub fn run(self, mut world: World) -> ! {
        simple_logger::SimpleLogger::new()
            .with_module_level("gfx", log::LevelFilter::Error)
            .with_module_level("wgpu", log::LevelFilter::Error)
            .with_module_level("winit", log::LevelFilter::Error)
            .with_module_level("naga", log::LevelFilter::Error)
            .init()
            .unwrap();

        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new();
        let window = window_builder.build(&event_loop).unwrap();

        let (instance, sc) = pollster::block_on(init_wgpu(&window));

        world.insert_system(GpuSystem::new(instance, sc));
        world.insert_system(WindowSystem::new(window));
        world.dequeue();

        world.running = true;

        world.init();

        world.dequeue();

        event_loop.run(move |event, _, control_flow| match event {
            Event::MainEventsCleared => {
                world
                    .system_mut::<WindowSystem>()
                    .unwrap()
                    .window
                    .request_redraw();
            }
            Event::RedrawRequested(_) => {
                world.pre_update();
                world.update();
                world.post_update();
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(size) => {
                    let mut gpu_system = world.system_mut::<GpuSystem>().unwrap();

                    let GpuSystem {
                        instance,
                        swapchain,
                    } = gpu_system.as_mut();

                    swapchain.recreate(instance.as_ref(), size.width, size.height);
                }
                WindowEvent::ScaleFactorChanged {
                    new_inner_size: size,
                    ..
                } => {
                    let mut gpu_system = world.system_mut::<GpuSystem>().unwrap();

                    let GpuSystem {
                        instance,
                        swapchain,
                    } = gpu_system.as_mut();

                    swapchain.recreate(instance.as_ref(), size.width, size.height);
                }
                _ => {}
            },
            _ => {}
        });
    }
}
