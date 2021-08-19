use bytemuck::cast_slice;
use egui::*;
use nova_assets::{Assets, Handle};
use nova_core::{stage, AppBuilder, Plugin, Resources, World};
use nova_input::{key::Key, mouse_button::MouseButton, Input, Mouse, TextInput};
use nova_render::render_target::RenderTarget;
use nova_wgpu::*;
use nova_window::Windows;

fn to_key(key: Key) -> Option<egui::Key> {
    use egui::Key as K;

    match key {
        Key::Down => Some(K::ArrowDown),
        Key::Left => Some(K::ArrowLeft),
        Key::Right => Some(K::ArrowRight),
        Key::Up => Some(K::ArrowUp),
        Key::Escape => Some(K::Escape),
        Key::Tab => Some(K::Tab),
        Key::Backspace => Some(K::Backspace),
        Key::Return => Some(K::Enter),
        Key::Space => Some(K::Space),
        Key::Insert => Some(K::Insert),
        Key::Delete => Some(K::Delete),
        Key::Home => Some(K::Home),
        Key::End => Some(K::End),
        Key::PageUp => Some(K::PageUp),
        Key::PageDown => Some(K::PageDown),
        Key::Key0 => Some(K::Num0),
        Key::Key1 => Some(K::Num1),
        Key::Key2 => Some(K::Num2),
        Key::Key3 => Some(K::Num3),
        Key::Key4 => Some(K::Num4),
        Key::Key5 => Some(K::Num5),
        Key::Key6 => Some(K::Num6),
        Key::Key7 => Some(K::Num7),
        Key::Key8 => Some(K::Num8),
        Key::Key9 => Some(K::Num9),
        Key::A => Some(K::A),
        Key::B => Some(K::B),
        Key::C => Some(K::C),
        Key::D => Some(K::D),
        Key::E => Some(K::E),
        Key::F => Some(K::F),
        Key::G => Some(K::G),
        Key::H => Some(K::H),
        Key::I => Some(K::I),
        Key::J => Some(K::J),
        Key::K => Some(K::K),
        Key::L => Some(K::L),
        Key::M => Some(K::M),
        Key::N => Some(K::N),
        Key::O => Some(K::O),
        Key::P => Some(K::P),
        Key::Q => Some(K::Q),
        Key::R => Some(K::R),
        Key::S => Some(K::S),
        Key::T => Some(K::T),
        Key::U => Some(K::U),
        Key::V => Some(K::V),
        Key::W => Some(K::W),
        Key::X => Some(K::X),
        Key::Y => Some(K::Y),
        Key::Z => Some(K::Z),
        _ => None,
    }
}

pub struct EguiResource<F> {
    pub raw_input: RawInput,
    pub bind_group_layout: Option<BindGroupLayout>,
    pub render_pipeline: Option<RenderPipeline>,
    pub egui_texture: Option<TextureView<'static>>,
    pub sampler: Option<Sampler>,
    pub prev_cursor_pos: Pos2,
    pub func: F,
}

impl<F> EguiResource<F> {
    #[inline]
    pub fn new(func: F) -> Self {
        Self {
            raw_input: Default::default(),
            bind_group_layout: None,
            render_pipeline: None,
            egui_texture: None,
            sampler: None,
            prev_cursor_pos: Default::default(),
            func,
        }
    }
}

pub struct EguiPlugin<F> {
    pub func: F,
}

impl<F> EguiPlugin<F> {
    #[inline]
    pub fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F> Plugin for EguiPlugin<F>
where
    F: FnMut(&CtxRef, &World, &Resources) + 'static,
{
    #[inline]
    fn build(self, app: &mut AppBuilder) {
        let instance = app.resources.get::<Instance>().unwrap();
        let target = app.resources.get::<RenderTarget>().unwrap();

        let shader_module = instance.create_shader_module(&ShaderModuleDescriptor {
            label: Some("egui"),
            source: ShaderSource::Wgsl(include_str!("egui.wgsl").into()),
            flags: ShaderFlags::all(),
        });

        let bind_group_layout = instance.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("egui"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStage::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        min_binding_size: None,
                        has_dynamic_offset: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStage::VERTEX_FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStage::VERTEX_FRAGMENT,
                    ty: BindingType::Sampler {
                        filtering: true,
                        comparison: false,
                    },
                    count: None,
                },
            ],
        });

        let layout = instance.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("egui"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = instance.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("egui"),
            layout: Some(&layout),
            vertex: VertexState {
                module: &shader_module,
                entry_point: "main",
                buffers: &[VertexBufferLayout {
                    array_stride: 32,
                    step_mode: InputStepMode::Vertex,
                    attributes: &[
                        VertexAttribute {
                            offset: 0,
                            format: VertexFormat::Float32x2,
                            shader_location: 0,
                        },
                        VertexAttribute {
                            offset: 8,
                            format: VertexFormat::Float32x2,
                            shader_location: 1,
                        },
                        VertexAttribute {
                            offset: 16,
                            format: VertexFormat::Float32x4,
                            shader_location: 2,
                        },
                    ],
                }],
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: "main",
                targets: &[ColorTargetState {
                    format: target.format(),
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrite::ALL,
                }],
            }),
            primitive: PrimitiveState::default(),
            multisample: MultisampleState::default(),
            depth_stencil: None,
        });

        let sampler = instance.create_sampler(&SamplerDescriptor {
            label: Some("egui"),
            ..Default::default()
        });

        let mut egui_resource = EguiResource::new(self.func);

        egui_resource.sampler = Some(sampler);
        egui_resource.bind_group_layout = Some(bind_group_layout);
        egui_resource.render_pipeline = Some(pipeline);

        drop(instance);
        drop(target);

        app.add_thread_local_to_stage(stage::POST_UPDATE, egui_system::<F>);
        app.insert_resource(egui_resource);
        app.register_resource::<CtxRef>();
    }
}

fn egui_system<F>(world: &mut World, resources: &mut Resources)
where
    F: FnMut(&CtxRef, &World, &Resources) + 'static,
{
    let mouse = resources.get::<Mouse>().unwrap();
    let key_input = resources.get::<Input<Key>>().unwrap();
    let mouse_input = resources.get::<Input<MouseButton>>().unwrap();
    let text_input = resources.get::<TextInput>().unwrap();
    let windows = resources.get::<Windows>().unwrap();
    let mut ctx_ref = resources.get_mut::<CtxRef>().unwrap();
    let mut egui_resource = resources.get_mut::<EguiResource<F>>().unwrap();

    let mouse_pos = Pos2::new(mouse.position.x, mouse.position.y);

    let modifiers = Modifiers {
        alt: key_input.down(&Key::LAlt),
        ctrl: key_input.down(&Key::LControl),
        shift: key_input.down(&Key::LShift),
        mac_cmd: key_input.down(&Key::LWin),
        command: key_input.down(&Key::LWin),
    };

    for key in key_input.iter_pressed() {
        if let Some(key) = to_key(*key) {
            egui_resource.raw_input.events.push(Event::Key {
                key,
                modifiers,
                pressed: true,
            });
        }
    }

    for key in key_input.iter_released() {
        if let Some(key) = to_key(*key) {
            egui_resource.raw_input.events.push(Event::Key {
                key,
                modifiers,
                pressed: false,
            });
        }
    }

    for button in mouse_input.iter_pressed() {
        let button = match button {
            MouseButton::Left => PointerButton::Primary,
            MouseButton::Right => PointerButton::Secondary,
            MouseButton::Middle => PointerButton::Middle,
            _ => continue,
        };

        egui_resource.raw_input.events.push(Event::PointerButton {
            pos: mouse_pos,
            button,
            pressed: true,
            modifiers,
        });
    }

    for button in mouse_input.iter_released() {
        let button = match button {
            MouseButton::Left => PointerButton::Primary,
            MouseButton::Right => PointerButton::Secondary,
            MouseButton::Middle => PointerButton::Middle,
            _ => continue,
        };

        egui_resource.raw_input.events.push(Event::PointerButton {
            pos: mouse_pos,
            button,
            pressed: false,
            modifiers,
        });
    }

    for c in &text_input.chars {
        if !c.is_control() {
            egui_resource
                .raw_input
                .events
                .push(Event::Text(c.to_string()));
        }
    }

    egui_resource
        .raw_input
        .events
        .push(Event::PointerMoved(mouse_pos));
    egui_resource.raw_input.modifiers = modifiers;

    let size = windows.primary().size();

    egui_resource.raw_input.screen_rect = Some(Rect::from_min_size(
        Pos2::ZERO,
        Vec2::new(size.x as f32, size.y as f32),
    ));

    ctx_ref.begin_frame(egui_resource.raw_input.take());

    drop(mouse);
    drop(key_input);
    drop(mouse_input);
    drop(text_input);
    drop(windows);

    (egui_resource.func)(&ctx_ref, world, resources);

    let instance = resources.get::<Instance>().unwrap();
    let target = resources.get::<RenderTarget>().unwrap();
    let textures = resources.get::<Assets<TextureView>>().unwrap();

    let (_output, shapes) = ctx_ref.end_frame();
    let meshes = ctx_ref.tessellate(shapes);

    if egui_resource.egui_texture.is_none() {
        let texture = ctx_ref.texture();

        let data: Vec<u8> = texture
            .srgba_pixels()
            .flat_map(|pixel| [pixel.r(), pixel.g(), pixel.b(), pixel.a()])
            .collect();

        let texture = instance.create_texture_with_data(
            &TextureDescriptor {
                label: Some("egui_texture"),
                size: Extent3d {
                    width: texture.width as u32,
                    height: texture.height as u32,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST,
            },
            &data,
        );

        egui_resource.egui_texture = Some(texture.view());
    }

    target
        .view(|target_view| {
            let mut vertex_buffers = Vec::new();
            let mut index_buffers = Vec::new();
            let mut bind_groups = Vec::new();

            let mut encoder = instance.create_command_encoder(&CommandEncoderDescriptor {
                label: Some("egui"),
            });

            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("egui"),
                color_attachments: &[RenderPassColorAttachment {
                    view: target_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::TRANSPARENT),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&egui_resource.render_pipeline.as_ref().unwrap());

            for ClippedMesh(_clip_rect, mesh) in &meshes {
                let data: Vec<f32> = mesh
                    .vertices
                    .iter()
                    .flat_map(|vert| {
                        let color = Rgba::from(vert.color);

                        [
                            vert.pos.x,
                            vert.pos.y,
                            vert.uv.x,
                            vert.uv.y,
                            color.r(),
                            color.g(),
                            color.b(),
                            color.a(),
                        ]
                    })
                    .collect();

                let vertex_buffer = instance.create_buffer_init(&BufferInitDescriptor {
                    label: Some("egui"),
                    contents: cast_slice(&data),
                    usage: BufferUsage::COPY_DST | BufferUsage::VERTEX,
                });

                let index_buffer = instance.create_buffer_init(&BufferInitDescriptor {
                    label: Some("egui"),
                    contents: cast_slice(&mesh.indices),
                    usage: BufferUsage::COPY_DST | BufferUsage::INDEX,
                });

                let uniform_buffer = instance.create_buffer_init(&BufferInitDescriptor {
                    label: Some("egui"),
                    contents: cast_slice(&[size.x as f32, size.y as f32]),
                    usage: BufferUsage::COPY_DST | BufferUsage::UNIFORM,
                });

                let texture = match mesh.texture_id {
                    TextureId::Egui => egui_resource.egui_texture.as_ref().unwrap(),
                    TextureId::User(id) => textures.get(&Handle::from(id)).unwrap(),
                };

                let bind_group = instance.create_bind_group(&BindGroupDescriptor {
                    label: Some("egui"),
                    layout: egui_resource.bind_group_layout.as_ref().unwrap(),
                    entries: &[
                        BindGroupEntry {
                            binding: 0,
                            resource: BindingResource::Buffer(BufferBinding {
                                buffer: &uniform_buffer,
                                offset: 0,
                                size: None,
                            }),
                        },
                        BindGroupEntry {
                            binding: 1,
                            resource: BindingResource::TextureView(texture),
                        },
                        BindGroupEntry {
                            binding: 2,
                            resource: BindingResource::Sampler(
                                egui_resource.sampler.as_ref().unwrap(),
                            ),
                        },
                    ],
                });

                vertex_buffers.push(vertex_buffer);
                index_buffers.push(index_buffer);
                bind_groups.push(bind_group);
            }

            let mut vertex_buffers = vertex_buffers.iter();
            let mut index_buffers = index_buffers.iter();
            let mut bind_groups = bind_groups.iter();

            for ClippedMesh(clip_rect, mesh) in meshes {
                render_pass.set_scissor_rect(
                    clip_rect.left() as u32,
                    clip_rect.top() as u32,
                    clip_rect.width() as u32,
                    clip_rect.height() as u32,
                );

                render_pass.set_bind_group(0, bind_groups.next().unwrap(), &[]);
                render_pass.set_vertex_buffer(0, vertex_buffers.next().unwrap().slice(..));
                render_pass
                    .set_index_buffer(index_buffers.next().unwrap().slice(..), IndexFormat::Uint32);

                render_pass.draw_indexed(0..mesh.indices.len() as u32, 0, 0..1);
            }

            drop(render_pass);

            instance.submit(encoder);
        })
        .unwrap();
}
