extern crate amethyst;
use amethyst::{
    controls::HideCursor,
    core::Transform,
    input::{is_key_down, is_mouse_button_down, VirtualKeyCode},
    prelude::*,
    renderer::light::{Light, PointLight},
    renderer::palette::rgb::Rgb,
    renderer::Camera,
    window::ScreenDimensions,
    winit::MouseButton,
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

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let StateData { world, .. } = data;
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                let mut hide_cursor = world.write_resource::<HideCursor>();
                hide_cursor.hide = false;
            } else if is_mouse_button_down(&event, MouseButton::Left) {
                let mut hide_cursor = world.write_resource::<HideCursor>();
                hide_cursor.hide = true;
            } else if is_key_down(&event, VirtualKeyCode::Space) {
                println!("Pressed spacebar")
            }
        }
        Trans::None
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
