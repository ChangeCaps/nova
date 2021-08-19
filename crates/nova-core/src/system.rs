use legion::systems::Runnable;

// i shouldn't have to do this
pub trait RunnableContainer: Runnable + Send + Sync {}

impl<T: Runnable + Send + Sync> RunnableContainer for T {}

impl Runnable for Box<dyn RunnableContainer> {
    fn name(&self) -> Option<&legion::systems::SystemId> {
        self.as_ref().name()
    }

    fn reads(
        &self,
    ) -> (
        &[legion::systems::ResourceTypeId],
        &[legion::storage::ComponentTypeId],
    ) {
        self.as_ref().reads()
    }

    fn writes(
        &self,
    ) -> (
        &[legion::systems::ResourceTypeId],
        &[legion::storage::ComponentTypeId],
    ) {
        self.as_ref().writes()
    }

    fn prepare(&mut self, world: &legion::World) {
        self.as_mut().prepare(world)
    }

    fn accesses_archetypes(&self) -> &legion::world::ArchetypeAccess {
        self.as_ref().accesses_archetypes()
    }

    unsafe fn run_unsafe(
        &mut self,
        world: &legion::World,
        resources: &legion::systems::UnsafeResources,
    ) {
        self.as_mut().run_unsafe(world, resources)
    }

    fn command_buffer_mut(
        &mut self,
        world: legion::world::WorldId,
    ) -> Option<&mut legion::systems::CommandBuffer> {
        self.as_mut().command_buffer_mut(world)
    }
}
