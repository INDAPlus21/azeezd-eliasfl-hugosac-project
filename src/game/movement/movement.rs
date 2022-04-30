use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::game::{Block, Player};

use std::f32::consts::FRAC_1_SQRT_2;

#[derive(SystemDesc)]
pub struct MovementSystem {
    pub speed: f32,
}

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Block>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (blocks, mut players, mut locals, time, input): Self::SystemData) {
        for (player, local) in (&mut players, &mut locals).join() {
            let x_mov = input.axis_value("move_x");
            let y_mov = input.axis_value("move_y");
            let z_mov = input.axis_value("move_z");

            let dt = time.delta_seconds();

            // Normalize movement such that ||Diagonal Movement|| == ||Orthogonal Movement||
            let dv = if x_mov != Some(0.) && z_mov != Some(0.) {
                FRAC_1_SQRT_2 * self.speed * dt
            } else {
                self.speed * dt
            };

            if let Some(movement) = x_mov {
                local.append_rotation_x_axis(-player.vert_rotation);
                local.append_translation_xyz(movement * dv, 0., 0.);
                local.append_rotation_x_axis(player.vert_rotation);
            }
            if let Some(movement) = z_mov {
                local.append_rotation_x_axis(-player.vert_rotation);
                local.append_translation_xyz(0., 0., movement * dv);
                local.append_rotation_x_axis(player.vert_rotation);
            }

            if let Some(movement) = y_mov {
                if player.can_jump {
                    player.y_velocity += 5. * movement;
                }
            }
        }
    }
}
