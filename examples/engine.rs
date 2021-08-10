use nova::prelude::*;
use nova_engine::app::App;
use nova_wgpu::*;

struct TestSystem;

impl System for TestSystem {
    fn update(&self, world: &World) {
        let gpu_system = world.system::<GpuSystem>().unwrap();

        let buffer = gpu_system.instance.create_buffer(&BufferDescriptor {
            label: None,
            size: 128,
            usage: BufferUsage::INDEX,
            mapped_at_creation: false,
        });

        let frame = gpu_system.instance.get_current_frame().unwrap();

        let frame_view = frame.output.view();

        let mut encoder = gpu_system
            .instance
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Command encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[RenderPassColorAttachment {
                view: &frame_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLUE),
                    store: true,
                },
            }],
            depth_stencil_attachment: None
        }); 

        render_pass.set_index_buffer(buffer.slice(0..128), IndexFormat::Uint32);

        drop(render_pass);

        gpu_system.instance.submit(encoder);
    }
}

fn main() {
    let mut world = World::new();

    world.insert_system(TestSystem);

    App::new().run(world);
}
