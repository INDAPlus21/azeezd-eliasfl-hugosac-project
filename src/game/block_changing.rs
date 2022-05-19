use amethyst::{
    assets::{Handle},
    core::{
        math::{Point2, Vector2},
        Transform,
    },
    derive::SystemDesc,
    ecs::{
        Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage,
    },
    input::{InputEvent, StringBindings},
    renderer::{ActiveCamera, Camera, Material, Mesh},
    shrev::{EventChannel, ReaderId},
    window::ScreenDimensions,
    winit::MouseButton,
};

use super::{Block, Player, BLOCK_SIZE_FROM_CENTER};

/// How low the player can reach to break and replace blocks
pub const PLAYER_REACH: f32 = 5.0;

#[derive(SystemDesc)]
#[system_desc(name(MouseRaycastSystemDesc))]
pub struct MouseRaycastSystem {
    #[system_desc(event_channel_reader)]
    event_reader: ReaderId<InputEvent<StringBindings>>,
}

impl MouseRaycastSystem {
    pub fn new(event_reader: ReaderId<InputEvent<StringBindings>>) -> Self {
        Self { event_reader }
    }
}

impl<'s> System<'s> for MouseRaycastSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Block>,
        WriteStorage<'s, Handle<Mesh>>,
        WriteStorage<'s, Handle<Material>>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Player>,
        ReadStorage<'s, Camera>,
        Read<'s, ActiveCamera>,
        ReadExpect<'s, ScreenDimensions>,
        Read<'s, EventChannel<InputEvent<StringBindings>>>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut blocks,
            mut meshes,
            mut materials,
            mut locals,
            mut players,
            cameras,
            active_camera,
            screen_dimensions,
            events,
        ): Self::SystemData,
    ) {
        for event in events.read(&mut self.event_reader) {
            // if left or right mouse button is pressed
            if let InputEvent::MouseButtonPressed(
                button @ (MouseButton::Left | MouseButton::Right | MouseButton::Middle),
            ) = *event
            {
                // Get the active camera if it is spawned and ready
                let mut camera_join = (&cameras, &locals).join();
                if let Some((camera, camera_transform)) = active_camera
                    .entity
                    .and_then(|a| camera_join.get(a, &entities))
                    .or_else(|| camera_join.next())
                {
                    // Project a ray from the camera
                    let ray = camera.screen_ray(
                        // Middle of screen, crosshair position
                        Point2::new(
                            screen_dimensions.width() / 2.0,
                            screen_dimensions.height() / 2.0,
                        ),
                        Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                        camera_transform,
                    );
                    // Nearest block and distance from camera
                    let mut nearest_block: Option<(&Block, f32, Entity)> = None;

                    for (entity, block) in (&entities, &blocks).join() {
                        // Raycasting using AABB (axis aligned bounding box)
                        // Inspiration from https://gdbooks.gitbooks.io/3dcollisions/content/Chapter3/raycast_aabb.html
                        let min_x = block.x - BLOCK_SIZE_FROM_CENTER;
                        let max_x = block.x + BLOCK_SIZE_FROM_CENTER;
                        let min_y = block.y - BLOCK_SIZE_FROM_CENTER;
                        let max_y = block.y + BLOCK_SIZE_FROM_CENTER;
                        let min_z = block.z - BLOCK_SIZE_FROM_CENTER;
                        let max_z = block.z + BLOCK_SIZE_FROM_CENTER;
                        let t1 = (min_x - ray.origin.x) / ray.direction.x;
                        let t2 = (max_x - ray.origin.x) / ray.direction.x;
                        let t3 = (min_y - ray.origin.y) / ray.direction.y;
                        let t4 = (max_y - ray.origin.y) / ray.direction.y;
                        let t5 = (min_z - ray.origin.z) / ray.direction.z;
                        let t6 = (max_z - ray.origin.z) / ray.direction.z;

                        let tmin = f32::max(
                            f32::max(f32::min(t1, t2), f32::min(t3, t4)),
                            f32::min(t5, t6),
                        );
                        let tmax = f32::min(
                            f32::min(f32::max(t1, t2), f32::max(t3, t4)),
                            f32::max(t5, t6),
                        );
                        // if tmax < 0, ray (line) is intersecting AABB, but whole AABB is behind us
                        if tmax < 0.0 {
                            continue;
                        }

                        // if tmin > tmax, ray doesn't intersect AABB
                        if tmin > tmax {
                            continue;
                        }

                        // t-value for ray to block collision point
                        let dist = if tmin < 0.0 { tmax } else { tmin };

                        // if block is further away than a certain threshold
                        if dist > PLAYER_REACH {
                            continue;
                        }

                        // update nearest block
                        if let Some((_, block_dist, _)) = nearest_block {
                            if dist < block_dist {
                                nearest_block = Some((block, dist, entity));
                            }
                        } else {
                            nearest_block = Some((block, dist, entity));
                        }
                    }

                    // If left mouse is pressed (destroy block)
                    if let MouseButton::Left = button {
                        // destroy nearest block (if any)
                        if let Some((_, _, entity)) = nearest_block {
                            entities.delete(entity).unwrap();
                        }
                    }

                    // If middle mouse clicked (store block material)
                    if let MouseButton::Middle = button {
                        if let Some((block, _, entity)) = nearest_block {
                            for player in (&mut players).join() {
                                player.current_block = Some((materials.get(entity).unwrap().clone(), block.surface));
                            }
                        }
                    }

                    // If right mouse is pressed (place block)
                    if let MouseButton::Right = button {
                        // place block on top of
                        if let Some((block, _dist, entity)) = nearest_block {
                            let mut transform = Transform::default();
                            transform.append_translation_xyz(block.x, block.y + 1.0, block.z);

                            // Possible example for entity mesh: https://github.dev/amethyst/amethyst/blob/v0.15.3/amethyst_assets/examples/hl.rs
                            // Another resource: https://community.amethyst.rs/t/runtime-based-meshes/610/3
                            
                            // Get material and surface stored in player (if they have picked one using middle click)
                            let current_block = {
                                let mut block = None;
                                for player in (&mut players).join() {
                                    block = player.current_block.as_ref();
                                }
                                block
                            };

                            // If there is a material place the block
                            if let Some((material, surface)) = current_block {
                                let mesh = meshes.get(entity).unwrap(); // Get mesh of nearest block (easy way to get block, maybe can be better)

                                entities
                                    .build_entity()
                                    .with(
                                        Block::new(
                                            block.x,
                                            block.y + 1.0,
                                            block.z,
                                            surface.clone(),
                                        ),
                                        &mut blocks,
                                    )
                                    .with(transform, &mut locals)
                                    .with(mesh.clone(), &mut meshes)
                                    .with(material.clone(), &mut materials)
                                    .build();
                            }
                        }
                    }
                }
            }
        }
    }
}
