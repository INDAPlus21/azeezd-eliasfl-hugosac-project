use amethyst::{
    core::{
        math::{Point2, Vector2},
        Transform,
    },
    derive::SystemDesc,
    ecs::{Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, SystemData},
    input::{InputHandler, StringBindings},
    renderer::{ActiveCamera, Camera},
    window::ScreenDimensions,
    winit::MouseButton,
};

use super::{Block, Player, BLOCK_SIZE_FROM_CENTER};

/// How low the player can reach to break and replace blocks
pub const PLAYER_REACH: f32 = 5.0;

#[derive(SystemDesc)]
pub struct MouseRaycastSystem;

impl<'s> System<'s> for MouseRaycastSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Block>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, ActiveCamera>,
        ReadExpect<'s, ScreenDimensions>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (
            entities,
            blocks,
            locals,
            cameras,
            active_camera,
            screen_dimensions,
            input,
        ): Self::SystemData,
    ) {
        // If left mouse is pressed
        if input.mouse_button_is_down(MouseButton::Left) {
            // Get the active camera if it is spawned and ready
            let mut camera_join = (&cameras, &locals).join();
            if let Some((camera, camera_transform)) = active_camera
                .entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
            {
                // Project a ray from the camera
                let ray = camera.screen_ray(
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

                    if let Some((_, block_dist, _)) = nearest_block {
                        if dist < block_dist {
                            nearest_block = Some((block, dist, entity));
                        }
                    } else {
                        nearest_block = Some((block, dist, entity));
                    }
                }
                // destroy nearest block
                if let Some((block, dist, entity)) = nearest_block {
                    println!("({}, {}, {})", block.x, block.y, block.z);
                    println!("Distance: {}", dist);

                    entities.delete(entity).unwrap();
                }
            }
        }
    }
}
