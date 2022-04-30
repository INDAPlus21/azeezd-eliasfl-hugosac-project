extern crate amethyst;
use amethyst::{
    controls::HideCursor,
    core::Transform,
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::light::{Light, PointLight},
    renderer::palette::rgb::Rgb,
    window::ScreenDimensions,
    SimpleState,
    winit::{Event, DeviceEvent}
};
use noise::{NoiseFn, Perlin};
use rand::prelude::*;

mod block;
pub use block::{initialize_blocks, Block, BLOCK_SIZE_FROM_CENTER};

pub mod movement;


mod player;
pub use player::*;

pub struct InGame;

impl SimpleState for InGame {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        init_light(world);
        init_player(world, 0., 5., 0., 2., 2.0, &dimensions);

        initialize_blocks(world, &{
            let mut blocks: Vec<Block> = Vec::with_capacity(10_000);
            let perlin = Perlin::new();
            let map_size = 128.;
            let chunk_size = 256;

            let mut rng = rand::thread_rng();

            // Random frequency in the range [1, 4)
            let freq = rng.gen::<f64>() * 3.0 + 1.0;

            for x in -(chunk_size / 2)..(chunk_size / 2) {
                for z in -(chunk_size / 2)..(chunk_size / 2) {
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

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let StateData { world, .. } = data;
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                let mut hide_cursor = world.write_resource::<HideCursor>();
                hide_cursor.hide = !hide_cursor.hide;
            }
        }
        Trans::None
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
