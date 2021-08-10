Pseudo code detailing the design of the engine.
```rust
struct RegisteredState {
	systems: RegisteredSystems,
	components: RegisteredComponents,
}

struct GameState {
	world: World,
}

struct World {
	systems: Systems,
	root_node: NodeId,
	nodes: Map<NodeId, Node>,
}

struct Node {
	name: String,
	parent: NodeId,
	components: Vec<Component>,
}
```