use crate::world::World;

pub trait Plugin {
    fn build(self, world: &mut World);
}
