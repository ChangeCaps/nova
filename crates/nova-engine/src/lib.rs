use nova_core::{App, AppBuilder};
use nova_input::{key::Key, mouse_button::MouseButton, Input, Mouse, TextInput};
use nova_render::{render_node::Target, render_target::RenderTarget, renderer::Renderer};
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

#[inline]
pub fn run(title: &str, func: impl FnOnce(AppBuilder) -> App) -> ! {
    let _ = simple_logger::SimpleLogger::new()
        .with_module_level("gfx", log::LevelFilter::Error)
        .with_module_level("wgpu", log::LevelFilter::Error)
        .with_module_level("winit", log::LevelFilter::Error)
        .with_module_level("naga", log::LevelFilter::Error)
        .init();

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title(title);
    let window = window_builder.build(&event_loop).unwrap();

    let (instance, sc) = pollster::block_on(init_wgpu(&window));

    let mut app = AppBuilder::new();

    app.insert_resource(instance);
    app.insert_resource(RenderTarget::SwapChain(sc));
    app.insert_resource(Windows::new(window));

    let mut app = func(app);

    app.startup_schedule
        .execute(&mut app.world, &mut app.resources);

    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            app.resources
                .get_mut::<Windows>()
                .unwrap()
                .primary()
                .request_redraw();
        }
        Event::RedrawRequested(_) => {
            app.schedule.execute(&mut app.world, &mut app.resources);

            if let Some(mut renderer) = app.resources.get_mut::<Renderer>() {
                let swap_chain = app.resources.get::<SwapChain>().unwrap();
                let frame = swap_chain.get_current_frame().unwrap();

                let target = Target {
                    view: &frame.output.view(),
                    depth: None,
                    size: swap_chain.size().into(),
                    format: swap_chain.format(),
                };

                drop(swap_chain);

                renderer.render_view(&app.world, &app.resources, &target);
            }
        }
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::Resized(size) => {
                let instance = app.resources.get::<Instance>().unwrap();
                let mut target = app.resources.get_mut::<RenderTarget>().unwrap();

                target.recreate(&instance, size.width, size.height);
            }
            WindowEvent::ScaleFactorChanged {
                new_inner_size: size,
                ..
            } => {
                let instance = app.resources.get::<Instance>().unwrap();
                let mut target = app.resources.get_mut::<RenderTarget>().unwrap();

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
                let mut input = app.resources.get_mut::<Input<Key>>().unwrap();

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
            WindowEvent::ReceivedCharacter(c) => {
                app.resources.get_mut::<TextInput>().unwrap().chars.push(c);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let mut input = app.resources.get_mut::<Input<MouseButton>>().unwrap();

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
                let mut mouse = app.resources.get_mut::<Mouse>().unwrap();

                mouse.position.x = position.x as f32;
                mouse.position.y = position.y as f32;
            }
            _ => {}
        },
        _ => {}
    });
}
