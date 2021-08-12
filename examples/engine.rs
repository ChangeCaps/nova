use nova_3d::{D3Plugin, Vertex3d, PBR_PIPELINE_HANDLE};
use nova_engine::app::App;
use nova_game::prelude::*;

struct Rotate;

impl Component for Rotate {
    fn update(&mut self, node: &Node, _world: &World) {
        node.component_mut::<Transform>().unwrap().rotation *= Quat::from_rotation_y(0.01);
    }
}

fn main() {
    let mut world = World::new();

    world.with_plugin(RenderPlugin).with_plugin(D3Plugin);

    let mut meshes = world.system_mut::<Assets<MeshData>>().unwrap();

    let mesh = Mesh {
        vertices: vec![
            Vertex3d {
                position: Vec3::new(-1.0, 0.0, 0.0),
                uv: Vec2::ZERO,
                color: Vec4::ONE,
            },
            Vertex3d {
                position: Vec3::new(0.0, 1.0, 0.0),
                uv: Vec2::ZERO,
                color: Vec4::ONE,
            },
            Vertex3d {
                position: Vec3::new(1.0, 0.0, 0.0),
                uv: Vec2::ZERO,
                color: Vec4::ONE,
            },
        ],
        indices: vec![0, 1, 2],
    };

    let mesh_data = meshes.add(mesh.into());

    drop(meshes);

    let mut camera = Node::new("Camera");

    let mut transform = Transform::from_xyz(3.0, 3.0, 3.0);
    transform.look_at(Vec3::ZERO, Vec3::Y);

    camera.add_component(MainCamera);
    camera.add_component(Camera::Perspective {
        fov: std::f32::consts::PI / 2.0,
        aspect: 1.0,
        near: 0.1,
    });
    camera.add_component(transform);

    for x in -10..=10 {
        for z in -10..=10 {
            let mut node = Node::new("Mesh");

            node.add_component(Transform::from_xyz(x as f32, 0.0, z as f32));
            node.add_component(MeshInstance {
                mesh_data: mesh_data.clone(),
                pipeline: PBR_PIPELINE_HANDLE,
                bindings: Default::default(),
                camera: None,
                buffer: None,
            });
            node.add_component(Rotate);
            world.insert_node(node);
        }
    }

    world.insert_node(camera);

    App::new().run(world);
}
