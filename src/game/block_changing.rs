use amethyst::{
    core::{
        geometry::Plane,
        math::{Point2, Vector2, Vector3},
        Transform,
    },
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{ActiveCamera, Camera},
    window::ScreenDimensions,
    winit::MouseButton,
};

use super::{Block, Player, BLOCK_SIZE_FROM_CENTER};

#[derive(SystemDesc)]
pub struct MouseRaycastSystem;

impl<'s> System<'s> for MouseRaycastSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Block>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, ActiveCamera>,
        ReadExpect<'s, ScreenDimensions>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (entities, blocks, _players, locals, cameras, active_camera, screen_dimensions, input): Self::SystemData,
    ) {
        // If left mouse is pressed
        if input.mouse_button_is_down(MouseButton::Left) {
            // Get the mouse position if its available
            if let Some(mouse_position) = input.mouse_position() {
                // Get the active camera if it is spawned and ready
                let mut camera_join = (&cameras, &locals).join();
                if let Some((camera, camera_transform)) = active_camera
                    .entity
                    .and_then(|a| camera_join.get(a, &entities))
                    .or_else(|| camera_join.next())
                {
                    println!("Clicked");
                    // Project a ray from the camera
                    let ray = camera.screen_ray(
                        Point2::new(mouse_position.0, mouse_position.1),
                        Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                        camera_transform,
                    );
                    for block in (&blocks).join() {
                        let block_pos = Vector3::new(block.x, block.y, block.z);
                        // distance from camera to block plane
                        let distance_x = ray.intersect_plane(&Plane::with_x(block.x)).unwrap();
                        let distance_y = ray.intersect_plane(&Plane::with_y(block.y)).unwrap();
                        let distance_z = ray.intersect_plane(&Plane::with_z(block.z)).unwrap();
                        let mut collision_x = ray.at_distance(distance_x).coords; // collision with x-plane for block
                        let mut collision_y = ray.at_distance(distance_y).coords; // collision with y-plane for block
                        let mut collision_z = ray.at_distance(distance_z).coords; // collision with z-plane for block
                        collision_x -= block_pos; // new vector is distance to block center
                        collision_y -= block_pos; // new vector is distance to block center
                        collision_z -= block_pos; // new vector is distance to block center
                        if collision_x.norm() < 5.0
                            && collision_y.norm() < 5.0
                            && collision_z.norm() < 5.0
                        {
                            println!("({}, {}, {})", block.x, block.y, block.z);
                            // println!(
                            //     "({} {}, {} {}, {} {})",
                            //     collision_x,
                            //     collision_x.norm(),
                            //     collision_y,
                            //     collision_y.norm(),
                            //     collision_z,
                            //     collision_z.norm()
                            // );
                        }

                        // if all distances is within block
                        if [distance_x, distance_y, distance_z].iter().all(|&dist| {
                            let mut block_point = Vector3::new(block.x, block.y, block.z);
                            block_point -= ray.at_distance(dist).coords;
                            // distance from intersection to block center
                            return block_point.norm() < BLOCK_SIZE_FROM_CENTER;
                        }) {
                            // let mouse_world_position = ray.at_distance(distance);
                            println!("({:.0}, {:.0}, {:.0})", block.x, block.y, block.z);
                        }
                    }
                }
            }
        }
    }
}
