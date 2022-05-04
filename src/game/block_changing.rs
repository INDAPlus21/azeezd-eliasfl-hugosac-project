use amethyst::{
    core::{
        geometry::Plane,
        math::{Point2, Vector2},
        Transform,
    },
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{ActiveCamera, Camera},
    window::ScreenDimensions,
    winit::MouseButton,
};

use super::{Block, Player};

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
        (entities, _blocks, _players, locals, cameras, active_camera, screen_dimensions, input): Self::SystemData,
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
                    // Project a ray from the camera to the 0z axis
                    let ray = camera.screen_ray(
                        Point2::new(mouse_position.0, mouse_position.1),
                        Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                        camera_transform,
                    );
                    let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
                    let mouse_world_position = ray.at_distance(distance);

                    println!(
                        "({:.0}, {:.0})",
                        mouse_world_position.x, mouse_world_position.y
                    );
                }
            }
        }
    }
}
