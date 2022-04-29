use amethyst::{
    core::Transform,
    ecs::{Component, DenseVecStorage},
    controls::{FlyControlTag},
    prelude::*,
    renderer::Camera,
    window::ScreenDimensions,
};

pub struct Player {
    pub height: f32,
    pub base_square_size: f32,
    pub y_velocity: f32
}

impl Player {
    pub fn new(height: f32, base_square_size: f32) -> Self {
        Self {
            height,
            base_square_size,
            y_velocity: 0.
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

pub fn init_player(
    world: &mut World,
    x: f32, y:f32, z:f32,
    height: f32,
    half_square_side: f32,
    camera_dimensions: &ScreenDimensions,
) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, z);
    transform.set_rotation_x_axis(-3.14 / 3.0);

    world
        .create_entity()
        .with(Camera::standard_3d(
            camera_dimensions.width(),
            camera_dimensions.height(),
        ))
        .with(transform)
        .with(FlyControlTag)
        .with(Player::new(height, half_square_side))
        .build();
}
