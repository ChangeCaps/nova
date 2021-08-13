use nova_3d::{shape, D3Plugin, PBR_PIPELINE_HANDLE};
use nova_engine::app::App;
use nova_game::prelude::*;
use nova_transform::component::GlobalTransform;

struct Rotate;

impl Component for Rotate {
    fn update(&mut self, node: &Node, _world: &World) {
        node.component_mut::<Transform>().unwrap().rotation *= Quat::from_rotation_y(0.01);
        node.component_mut::<Transform>().unwrap().rotation *= Quat::from_rotation_x(0.005);
        node.component_mut::<Transform>().unwrap().rotation *= Quat::from_rotation_z(0.02);
    }
}

fn main() {
    let mut world = World::new();

    world
        .with_plugin(RenderPlugin)
        .with_plugin(D3Plugin)
        .with_plugin(TransformPlugin);

    let mut meshes = world.system_mut::<Assets<MeshData>>().unwrap();

    let mesh: Mesh<_> = shape::Cube {
        size: Vec3::ONE / 2.0,
    }
    .into();
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
    camera.add_component(GlobalTransform::default());

    world.insert_node(camera);

    let mut light_parent = Node::new("LightParent");

    light_parent.add_component(Transform::default());
    light_parent.add_component(GlobalTransform::default());
    light_parent.add_component(Rotate);

    let light_parent = world.insert_node(light_parent);

    let mut light = Node::new("Light");

    light.add_component(PointLight {
        color: Color::rgb(1.0, 1.0, 1.0),
        intensity: 20.0,
    });
    light.add_component(MeshInstance {
        mesh_data: mesh_data.clone(),
        pipeline: PBR_PIPELINE_HANDLE,
        ..Default::default()
    });
    light.add_component(Transform::from_xyz(0.0, 3.0, 0.0));
    light.add_component(GlobalTransform::default());
    light.add_component(Parent(light_parent));

    world.insert_node(light);

    for x in -20..=20 {
        for z in -20..=20 {
            let mut node = Node::new("Mesh");

            node.add_component(Transform::from_xyz(x as f32, 0.0, z as f32));
            node.add_component(GlobalTransform::default());
            node.add_component(MeshInstance {
                mesh_data: mesh_data.clone(),
                pipeline: PBR_PIPELINE_HANDLE,
                ..Default::default()
            });
            node.add_component(Rotate);

            world.insert_node(node);
        }
    }

    App::new().run(world);
}
