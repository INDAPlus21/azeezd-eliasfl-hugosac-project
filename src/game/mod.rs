extern crate amethyst;
use amethyst::{
    core::Transform,
    prelude::*,
    renderer::light::{Light, PointLight},
    renderer::palette::rgb::Rgb,
    renderer::Camera,
    window::ScreenDimensions,
    SimpleState,
};

mod block;
pub use block::{initialize_blocks, Block};

pub struct InGame;

impl SimpleState for InGame {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        world.register::<Block>(); // To be removed after we add a system for the blocks

        init_light(world);
        init_camera(world, &dimensions);

        initialize_blocks(world, &{
            let mut blocks: Vec<Block> = Vec::with_capacity(400);
            for i in -10..10 {
                for j in -10..10 {
                    blocks.push(Block::new(2.0 * i as f32, 5.0, 2.0 * j as f32));
                }
            }
            blocks
        });
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 10., 20.);
    transform.append_rotation_x_axis(-3.14 / 6.0);

    world
        .create_entity()
        .with(Camera::standard_3d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
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
