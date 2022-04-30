use amethyst::{
    core::{shrev::EventChannel, SystemBundle, SystemDesc},
    Error, winit::Event, shrev::ReaderId,
};

mod movement;
pub use movement::*;

mod gravity;
pub use gravity::*;

mod rotation;
pub use rotation::*;

pub(crate) struct MovementBundle;

impl <'a, 'b> SystemBundle<'a, 'b> for MovementBundle {
    fn build(
        self,
        world: &mut amethyst::shred::World,
        dispatcher: &mut amethyst::shred::DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        dispatcher.add(Gravity, "gravity", &[]);
        dispatcher.add(MovementSystem, "movement", &["input_system"]);
        dispatcher.add(RotationSystemDesc::default().build(world), "rotation", &[]);

        Ok(())
    }
}