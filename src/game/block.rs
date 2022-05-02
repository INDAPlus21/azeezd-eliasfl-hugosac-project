use amethyst::{
    assets::AssetLoaderSystemData,
    core::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{formats::mesh::ObjFormat, ImageFormat, Material, MaterialDefaults, Mesh, Texture},
};

pub const BLOCK_SIZE_FROM_CENTER: f32 = 0.5; // Defined from mesh in cube.obj

#[derive(Clone, Copy)]
pub struct Block {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Block {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Block { x, y, z }
    }

    pub fn as_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_blocks(world: &mut World, blocks: &Vec<Block>) {
    let mesh = world
        .exec(|loader: AssetLoaderSystemData<'_, Mesh>| loader.load("cube.obj", ObjFormat, ()));

    let texture = world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
        loader.load("grass.png", ImageFormat::default(), ())
    });

    let mat_default = world.read_resource::<MaterialDefaults>().0.clone();
    let material = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
        loader.load_from_data(
            Material {
                albedo: texture,
                ..mat_default
            },
            (),
        )
    });

    for block in blocks.iter() {
        let mut transform = Transform::default();
        transform.append_translation_xyz(block.x, block.y, block.z);
        world
            .create_entity()
            .with(block.clone())
            .with(mesh.clone())
            .with(material.clone())
            .with(transform)
            .build();
    }
}
