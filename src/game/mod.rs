extern crate amethyst;
use amethyst::{
    controls::HideCursor,
    core::Transform,
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::light::{Light, PointLight},
    renderer::{palette::rgb::Rgb, Texture, ImageFormat},
    window::ScreenDimensions,
    SimpleState, ui::{Anchor, UiTransform, UiText, UiImage}, assets::AssetLoaderSystemData,
};
use noise::{NoiseFn, Perlin};
use rand::prelude::*;

mod block;
pub use block::*;

pub mod movement;

mod player;
pub use player::*;

mod block_changing;
pub use block_changing::*;

pub struct InGame;

impl SimpleState for InGame {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        init_light(world);
        init_player(world, 0., 9., 0., &dimensions);

        initialize_blocks(world, &{
            let mut blocks: Vec<Block> = Vec::with_capacity(1_000_000);
            let perlin = Perlin::new();
            let map_size = 128.;
            let chunk_size = 256;
            let min_height = -15;
            let max_height = 15.0;

            let mut rng = rand::thread_rng();

            // Random frequency in the range [7, 12)
            let freq = rng.gen::<f64>() * 5.0 + 7.0;

            // Iterate through x and z values of the map
            for x in -(chunk_size / 2)..(chunk_size / 2) {
                for z in -(chunk_size / 2)..(chunk_size / 2) {
                    let nx = (x as f32 / map_size - 1.0) as f64;
                    let nz = (z as f32 / map_size - 1.0) as f64;
                    
                    // 3 octaves of Perlin noise
                    let y = (
                        max_height * (
                            perlin.get([nx, nz])
                            + 0.5 * perlin.get([freq * nx, freq * nz])
                            + 0.25 * perlin.get([2.0 * freq * nx, 2.0 * freq * nz])
                        )/ (1.0 + 0.5 + 0.25)
                    ).round();

                    // Add top layer block
                    if y > 6.0 {
                        blocks.push(Block::new(x as f32, y as f32, z as f32, BlockSurface::Snow));
                    } else if y > -8.0 {
                        blocks.push(Block::new(x as f32, y as f32, z as f32, BlockSurface::Grass));
                    } else if y > -10.0 {
                        blocks.push(Block::new(x as f32, y as f32, z as f32, BlockSurface::Gravel));
                    } else {
                        // 50 % change of each type of stone
                        if rand::random() {
                            blocks.push(Block::new(x as f32, y as f32, z as f32, BlockSurface::StoneRough));
                        } else {
                            blocks.push(Block::new(x as f32, y as f32, z as f32, BlockSurface::StoneSmooth));
                        }
                    }

                    // Add blocks below down to the minimum height.
                    // The type of block that is added depends on the height.
                    for i in min_height..y as isize {
                        if i > -5 {
                            blocks.push(Block::new(x as f32, i as f32, z as f32, BlockSurface::Dirt));
                        } else if i > -8 {
                            blocks.push(Block::new(x as f32, i as f32, z as f32, BlockSurface::Gravel));
                        } else {
                            // 50 % change of each type of stone
                            if rand::random() {
                                blocks.push(Block::new(x as f32, i as f32, z as f32, BlockSurface::StoneRough));
                            } else {
                                blocks.push(Block::new(x as f32, i as f32, z as f32, BlockSurface::StoneSmooth));
                            }
                        }
                    }
                }
            }

            blocks.push(Block::new(0.0, 4.0, 0.0, BlockSurface::Dirt));
            blocks.push(Block::new(0.0, 1.0, 0.0, BlockSurface::Dirt));

            blocks
        });

        initialize_ui(world);
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

pub fn initialize_ui(world: &mut World) {
    let material = world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
        loader.load(format!("texture/crosshair.png"), ImageFormat::default(), ())
    });
    world.create_entity()
        .with(UiTransform::new("cross".to_string(), Anchor::Middle, Anchor::Middle, 0.0, 0.0, 0.0, 40.0, 40.0))
        .with(UiImage::Texture(material))
        .build();
}
