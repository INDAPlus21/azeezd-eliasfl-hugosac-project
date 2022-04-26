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
use noise::{NoiseFn, Perlin};
use rand::prelude::*;

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
            let mut blocks: Vec<Block> = Vec::with_capacity(10_000);
            let perlin = Perlin::new();
            let map_size = 100.0;

            let mut rng = rand::thread_rng();

            // Random frequency in the range [1, 4)
            let freq = rng.gen::<f64>() * 3.0 + 1.0;

            for x in -(map_size as isize / 2)..(map_size as isize / 2) {
                for z in -(map_size as isize / 2)..(map_size as isize / 2) {
                    let nx = (x as f32 / map_size - 1.0) as f64;
                    let nz = (z as f32 / map_size - 1.0) as f64;
                    // 3 octaves of Perlin noise
                    let y = (18.0
                        * (perlin.get([nx, nz])
                            + 0.5 * perlin.get([freq * nx, freq * nz])
                            + 0.25 * perlin.get([2.0 * freq * nx, 2.0 * freq * nz]))
                        / (1.0 + 0.5 + 0.25))
                        .round();

                    blocks.push(Block::new(x as f32, y as f32, z as f32));
                }
            }

            blocks
        });
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 30., 80.);
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
