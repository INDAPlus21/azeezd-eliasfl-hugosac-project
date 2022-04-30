use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

const GRAVITY: f32 = -10.;
const TERMINAL_VELOCITY: f32 = -50.0;

use crate::game::{Block, Player, BLOCK_SIZE_FROM_CENTER};

#[derive(SystemDesc)]
pub struct Gravity;

impl<'s> System<'s> for Gravity {
    type SystemData = (
        ReadStorage<'s, Block>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (blocks, mut players, mut locals, time): Self::SystemData) {
        for (player, local) in (&mut players, &mut locals).join() {
            let dt = time.delta_seconds();
            let v = player.y_velocity;
            let trans = local.translation();

            let mut dy = v * dt + GRAVITY * dt * dt; // dy = v dt + g dt^2
            let mut v_new = (v + GRAVITY * dt).max(TERMINAL_VELOCITY); // v = v0 + g dt

            player.can_jump = false;
            for block in (&blocks).join() {
                if trans[1] - player.height + dy < block.y
                    && trans[1] > block.y + BLOCK_SIZE_FROM_CENTER
                    && trans[0] < block.x + BLOCK_SIZE_FROM_CENTER
                    && trans[0] > block.x - BLOCK_SIZE_FROM_CENTER
                    && trans[2] < block.z + BLOCK_SIZE_FROM_CENTER
                    && trans[2] > block.z - BLOCK_SIZE_FROM_CENTER
                {
                    dy = 0.0;
                    v_new = 0.0;
                    player.can_jump = true;
                    break;
                }
            }

            local.prepend_translation_y(dy);

            player.y_velocity = v_new;
        }
    }
}
