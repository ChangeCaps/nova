pub use nova_core as core;

pub mod prelude {
    pub use nova_core::{
        component::Component,
        node::{Node, NodeId},
        system::System,
        world::World,
        Read, Write,
    };
}
