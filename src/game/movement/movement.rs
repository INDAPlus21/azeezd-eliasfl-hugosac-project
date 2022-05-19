use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use super::collision;
use crate::game::{
    Block, Player, BLOCK_SIZE_FROM_CENTER, HEAD_HEIGHT, HEIGHT, PLAYER_SIZE_FROM_CENTER,
};

use std::f32::consts::FRAC_1_SQRT_2;

const GRAVITY: f32 = -10.;
const TERMINAL_VELOCITY: f32 = -50.0;
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
            // Get key pressed and direction
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

            // Transformation for new position
            let mut transf = local.clone();

            // x movement and collision
            if let Some(movement) = x_mov {
                let current = transf.translation().clone();

                // Find matrix of new position
                transf.append_rotation_x_axis(-player.vert_rotation);

                transf.append_translation_xyz(movement * dv, 0., 0.);

                // Rotate back
                transf.append_rotation_x_axis(player.vert_rotation);

                let x_transf = transf.translation().clone();

                let mut new_pos: [f32; 3] = [x_transf[0], x_transf[1], x_transf[2]];

                // Check for collisions with blocks
                for block in (&blocks).join() {
                    if collision(
                        [current[0], current[1], current[2]],
                        [x_transf[0], x_transf[1], x_transf[2]],
                        block.as_array(),
                    ) {
                        // Positive if moving forward, negative if moving backwards
                        if movement > 0. {
                            // Place on correct side of block
                            new_pos[0] = block.x - BLOCK_SIZE_FROM_CENTER - PLAYER_SIZE_FROM_CENTER;
                        } else {
                            new_pos[0] = block.x + BLOCK_SIZE_FROM_CENTER + PLAYER_SIZE_FROM_CENTER;
                        }

                        // Player can't collide with multiple blocks at the same time
                        break;
                    }
                }

                // Move to new position
                let delta_x = new_pos[1] - [current[0], current[1], current[2]][0];

                transf.append_translation_xyz(delta_x, 0., 0.);
            }

            // z movement and collision
            if let Some(movement) = z_mov {
                let current = transf.translation().clone();

                // Find matrix of new position
                transf.append_rotation_x_axis(-player.vert_rotation);

                transf.append_translation_xyz(0., 0., movement * dv);

                // Rotate back
                transf.append_rotation_x_axis(player.vert_rotation);

                let z_transf = transf.translation().clone();

                let mut new_pos: [f32; 3] = [z_transf[0], z_transf[1], z_transf[2]];

                // Check for collisions with blocks
                for block in (&blocks).join() {
                    if collision(
                        [current[0], current[1], current[2]],
                        [z_transf[0], z_transf[1], z_transf[2]],
                        block.as_array(),
                    ) {
                        // Positive if moving forward, negative if moving backwards
                        if movement > 0. {
                            // Place on correct side of block
                            new_pos[2] = block.z - BLOCK_SIZE_FROM_CENTER - PLAYER_SIZE_FROM_CENTER;
                        } else {
                            new_pos[2] = block.z + BLOCK_SIZE_FROM_CENTER + PLAYER_SIZE_FROM_CENTER;
                        }

                        // Player can't collide with multiple blocks at the same time
                        break;
                    }
                }

                // Move to new position

                // The movement in the z direction
                let delta_z = new_pos[2] - [current[0], current[1], current[2]][2];

                // Move to new position
                transf.append_translation_xyz(0., 0., delta_z);
            }

            // Find matrix of new position
            transf.append_rotation_x_axis(-player.vert_rotation);

            // y movement and collision
            if let Some(movement) = y_mov {
                if player.can_jump && movement > 0. {
                    player.y_velocity += 5. * movement;
                }

                let current = transf.translation().clone();

                // Calculate gravity
                let v = player.y_velocity;
                let dy = v * dt + GRAVITY * dt * dt; // dy = v dt + g dt^2
                let mut v_new = (v + GRAVITY * dt).max(TERMINAL_VELOCITY); // v = v0 + g dt
                transf.append_translation_xyz(0.0, dy, 0.0);

                player.can_jump = false;

                // Rotate back
                transf.append_rotation_x_axis(player.vert_rotation);

                let y_transf = transf.translation().clone();

                let mut new_pos: [f32; 3] = [y_transf[0], y_transf[1], y_transf[2]];

                // Check for collisions with blocks
                for block in (&blocks).join() {
                    if collision(
                        [current[0], current[1], current[2]],
                        [y_transf[0], y_transf[1], y_transf[2]],
                        block.as_array(),
                    ) {
                        // Move player in y direction up to bottom of block
                        new_pos[1] = block.y - BLOCK_SIZE_FROM_CENTER - HEAD_HEIGHT;

                        // Positive if moving upwards, negative if moving down
                        if dy < 0. {
                            // moving down
                            // Place on correct side of block
                            new_pos[1] = block.y + BLOCK_SIZE_FROM_CENTER + HEIGHT;

                            player.can_jump = true;
                        }

                        v_new = 0.;

                        // Player can't collide with multiple blocks at the same time
                        break;
                    }
                }

                // The movement in the y direction
                let delta_y = new_pos[2] - [current[0], current[1], current[2]][2];

                // Move to new position
                transf.append_translation_xyz(0., delta_y, 0.);

                player.y_velocity = v_new;
            }

            let delta: [f32; 3] = (transf.translation() - local.translation()).into();
            local.prepend_translation_x(delta[0]);
            local.prepend_translation_y(delta[1]);
            local.prepend_translation_z(delta[2]);
        }
    }
}
