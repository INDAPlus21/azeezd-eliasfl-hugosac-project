use amethyst::{
    core::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, Material},
    window::ScreenDimensions, assets::Handle,
};

use super::block::BlockSurface;

pub const HEIGHT: f32 = 2.0; // From eyes to feet
pub const HEAD_HEIGHT: f32 = 0.2; // From eyes to top of head
pub const PLAYER_SIZE_FROM_CENTER: f32 = 0.4;

pub struct Player {
    pub y_velocity: f32,
    pub can_jump: bool,
    pub vert_rotation: f32,
    pub current_block: Option<(Handle<Material>, BlockSurface)> // Saves material of middle clicked block
}

impl Player {
    pub fn new() -> Self {
        Self {
            y_velocity: 0.,
            can_jump: true,
            vert_rotation: 0.,
            current_block: None
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

pub fn init_player(
    world: &mut World,
    x: f32,
    y: f32,
    z: f32,
    camera_dimensions: &ScreenDimensions,
) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, z);

    world
        .create_entity()
        .with(Camera::standard_3d(
            camera_dimensions.width(),
            camera_dimensions.height(),
        ))
        .with(transform)
        .with(Player::new())
        .build();
}
