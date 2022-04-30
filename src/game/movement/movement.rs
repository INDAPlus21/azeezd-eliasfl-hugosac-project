use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage}, input::{InputHandler, StringBindings}, shrev::{EventChannel, ReaderId},
    winit::{Event, DeviceEvent}
};

use crate::game::{Block, Player};

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Block>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (blocks, players, mut locals, time, input): Self::SystemData) {
        for (_, local) in (&players, &mut locals).join() {
            let x_mov = input.axis_value("move_x");
            let z_mov = input.axis_value("move_z");

            if let Some(movement) = x_mov {
                local.append_translation_xyz(movement*10.*time.delta_seconds(), 0.0, 0.0);
            }
            if let Some(movement) = z_mov {
                local.append_translation_xyz(0., 0., movement*10.*time.delta_seconds());
            }
        }
    }
}