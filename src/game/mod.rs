extern crate amethyst;
use amethyst::{
    SimpleState,
    window::ScreenDimensions,
    prelude::*,
    core::Transform,
    renderer::{
        Camera, 
    }
    
};

pub struct InGame;

impl SimpleState for InGame {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        init_camera(world, &dimensions);
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.0);

    world.create_entity()
        .with(Camera::standard_3d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}