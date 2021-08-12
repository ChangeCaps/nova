use nova_game::prelude::*;

struct TestComponent(u32);

impl Component for TestComponent {
    fn update(&mut self, _node: &Node, _world: &World) {
        println!("{}", self.0);
    }
}

fn main() {
    let mut world = World::new();

    let mut node = Node::new("Node");
    node.add_component(TestComponent(4));

    world.insert_node(node);

    world.update();
}
