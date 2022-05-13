use amethyst::{
    core::{SystemBundle, SystemDesc},
    Error,
};

mod movement;
pub use movement::*;

mod rotation;
pub use rotation::*;

mod collision_handler;
pub use collision_handler::*;

pub(crate) struct MovementBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MovementBundle {
    fn build(
        self,
        world: &mut amethyst::shred::World,
        dispatcher: &mut amethyst::shred::DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        dispatcher.add(MovementSystem { speed: 5. }, "movement", &["input_system"]);
        dispatcher.add(RotationSystemDesc::default().build(world), "rotation", &[]);

        Ok(())
    }
}
