use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

const GRAVITY: f32 = -10.;
const HALF_GRAVITY: f32 = GRAVITY / 2.0;
const TERMINAL_VELOCITY: f32 = -50.0;

use super::{Block, Player, BLOCK_SIZE_FROM_CENTER};

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

            let mut dy = v*dt + HALF_GRAVITY*dt*dt; // dy = v dt + g dt^2 
            let mut v_new = (v + GRAVITY * dt).max(TERMINAL_VELOCITY); // v = v0 + g dt

            for block in (&blocks).join() {
                if player.y - player.height + dy < block.y
                    && player.y >= block.y + BLOCK_SIZE_FROM_CENTER
                    && player.x <= block.x + BLOCK_SIZE_FROM_CENTER
                    && player.x >= block.x - BLOCK_SIZE_FROM_CENTER
                    && player.z <= block.z + BLOCK_SIZE_FROM_CENTER
                    && player.z >= block.z - BLOCK_SIZE_FROM_CENTER
                {
                    dy = 0.0;
                    v_new = 0.0;
                    break;
                }
            }

            local.prepend_translation_y(dy);
            player.y += dy;
            player.y_velocity = v_new;
        }
    }
}
