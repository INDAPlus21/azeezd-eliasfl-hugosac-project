extern crate amethyst;
use amethyst::{
    core::Transform,
    prelude::*,
    renderer::light::{Light, PointLight},
    renderer::palette::rgb::Rgb,
    window::ScreenDimensions,
    SimpleState,
};

mod block;
pub use block::{initialize_blocks, Block, BLOCK_SIZE_FROM_CENTER};
mod gravity;
pub use gravity::*;
mod player;
pub use player::*;

pub struct InGame;

impl SimpleState for InGame {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        init_light(world);
        init_player(world, 1.5, 0.5, 0., 20., 0., &dimensions);

        initialize_blocks(world, &{
            let mut blocks: Vec<Block> = Vec::with_capacity(400);
            for i in -10..10 {
                for j in -10..10 {
                    blocks.push(Block::new(i as f32, 0.0, j as f32));
                }
            }

            blocks.extend_from_slice(&[
                Block::new(1., 5., 0.),
                Block::new(-1., 5., 0.),
                Block::new(0., 5., 1.),
                Block::new(0., 5., -1.),
            ]);
            
            blocks
        });
    }
}

fn init_light(world: &mut World) {
    // Currently does not affect lighting due to flat 3d shading
    let mut transform = Transform::default();
    transform.append_translation_xyz(0.0, 4.0, 0.0);

    world
        .create_entity()
        .with(Light::Point(PointLight {
            color: Rgb::new(1.0, 1.0, 1.0),
            intensity: 500.0,
            radius: 5.0,
            smoothness: 1.0,
        }))
        .with(transform)
        .build();
}
