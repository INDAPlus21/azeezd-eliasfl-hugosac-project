use amethyst::{
    assets::{AssetLoaderSystemData, Handle},
    core::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{formats::mesh::ObjFormat, ImageFormat, Material, MaterialDefaults, Mesh, Texture},
};

pub const BLOCK_SIZE_FROM_CENTER: f32 = 0.5; // Defined from mesh in cube.obj

#[derive(Clone, Copy)]
pub enum BlockSurface {
    Grass,
    Dirt,
    Gravel,
    StoneRough,
    StoneSmooth,
    Snow
}

#[derive(Clone, Copy)]
pub struct Block {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub surface: BlockSurface
}

impl Block {
    pub fn new(x: f32, y: f32, z: f32, surface: BlockSurface) -> Self {
        Block { x, y, z, surface }
    }

    pub fn as_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}

fn get_mat(world: &mut World, file_name: &str, mat_default: Material) -> Handle<Material> {
    let texture = world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
        loader.load(format!("texture/{file_name}"), ImageFormat::default(), ())
    });

    let mat_handle = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
        loader.load_from_data(
            Material {
                albedo: texture,
                ..mat_default
            }, 
            ()
        )
    });

    mat_handle
}

pub fn initialize_blocks(world: &mut World, blocks: &Vec<Block>) {
    let mesh = world
        .exec(|loader: AssetLoaderSystemData<'_, Mesh>| loader.load("mesh/cube.obj", ObjFormat, ()));
    
    let mat_default = world.read_resource::<MaterialDefaults>().0.clone();

    // Load all materials
    let dirt_mat = get_mat(world, "dirt.png", mat_default.clone());
    let grass_mat = get_mat(world, "grass.png", mat_default.clone());
    let gravel_mat = get_mat(world, "gravel.png", mat_default.clone());
    let snow_mat = get_mat(world, "snow.png", mat_default.clone());
    let stone_rough_mat = get_mat(world, "stone_rough.png", mat_default.clone());
    let stone_smooth_mat = get_mat(world, "stone_smooth.png", mat_default.clone());
    
    for block in blocks.iter() {
        let mut transform = Transform::default();
        transform.append_translation_xyz(block.x, block.y, block.z);

        let material = match block.surface {
            BlockSurface::Dirt => dirt_mat.clone(),
            BlockSurface::Grass => grass_mat.clone(),
            BlockSurface::Gravel => gravel_mat.clone(),
            BlockSurface::Snow => snow_mat.clone(),
            BlockSurface::StoneRough => stone_rough_mat.clone(),
            BlockSurface::StoneSmooth => stone_smooth_mat.clone()
        };

        world
            .create_entity()
            .with(block.clone())
            .with(mesh.clone())
            .with(material.clone())
            .with(transform)
            .build();
    }
}
