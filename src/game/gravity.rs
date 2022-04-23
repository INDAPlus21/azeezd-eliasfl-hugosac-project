use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

const GRAVITY: f32 = -10.;

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
            let mut step = GRAVITY * time.delta_seconds(); // TODO: Δs = g/2*Δt^2 + v*Δt

            for block in (&blocks).join() {
                if player.y - player.height + step < block.y
                    && player.x <= block.x + BLOCK_SIZE_FROM_CENTER
                    && player.x >= block.x - BLOCK_SIZE_FROM_CENTER
                    && player.z <= block.z + BLOCK_SIZE_FROM_CENTER
                    && player.z >= block.z - BLOCK_SIZE_FROM_CENTER
                {
                    step = 0.0;
                    break;
                }
            }

            local.prepend_translation_y(step);
            player.y += step;
        }
    }
}
