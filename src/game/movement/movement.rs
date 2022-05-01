use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::game::{Block, Player, BLOCK_SIZE_FROM_CENTER};

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

            let mut transf = local.clone();
            let pos = local.translation();
            
            // Find matrix of new position
            transf.append_rotation_x_axis(-player.vert_rotation);
            if let Some(movement) = x_mov {
                transf.append_translation_xyz(movement * dv, 0., 0.);
            }
            if let Some(movement) = z_mov {
                transf.append_translation_xyz(0., 0., movement * dv);
            }
            transf.append_rotation_x_axis(player.vert_rotation);

            let mut delta: [f32; 3] = (transf.translation() - local.translation()).into();
            let new_pos = [pos[0] + delta[0], pos[1] + delta[1], pos[2] + delta[2]];

            for block in (&blocks).join() {
                if new_pos[0] - player.half_base_size < block.x + BLOCK_SIZE_FROM_CENTER
                && new_pos[0] + player.half_base_size > block.x - BLOCK_SIZE_FROM_CENTER
                && new_pos[2] - player.half_base_size < block.z + BLOCK_SIZE_FROM_CENTER
                && new_pos[2] + player.half_base_size > block.z - BLOCK_SIZE_FROM_CENTER
                && new_pos[1] - player.height > block.y - 2. * BLOCK_SIZE_FROM_CENTER // collision at player's legs' height
                && new_pos[1] - player.height < block.y
                {
                    // TODO: Find direction of collision and elimiate it for more realistic collisions
                    delta[0] = 0.;
                    delta[2] = 0.;
                }
            }

            local.prepend_translation_x(delta[0]);
            local.prepend_translation_y(delta[1]);
            local.prepend_translation_z(delta[2]);

            if let Some(movement) = y_mov {
                if player.can_jump {
                    player.y_velocity += 5. * movement;
                }
            }
        }
    }
}
