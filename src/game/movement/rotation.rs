use amethyst::{
    controls::{HideCursor, WindowFocus},
    ecs::{Join, Read, System, SystemData, WriteStorage},
    shrev::{EventChannel, ReaderId},
    winit::{DeviceEvent, Event},
    SystemDesc,
};

use std::f32::consts::FRAC_PI_2;

use derive_new::new as New;

use crate::game::{Player, Transform};

#[derive(SystemDesc, New)]
#[system_desc(name(RotationSystemDesc))]
pub struct RotationSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<Event>,
}

impl<'a> System<'a> for RotationSystem {
    type SystemData = (
        WriteStorage<'a, Player>,
        WriteStorage<'a, Transform>,
        Read<'a, EventChannel<Event>>,
        Read<'a, WindowFocus>,
        Read<'a, HideCursor>,
    );

    fn run(&mut self, (mut players, mut transform, events, focus, hide): Self::SystemData) {
        for (player, local) in (&mut players, &mut transform).join() {
            for event in events.read(&mut self.reader) {
                if focus.is_focused && hide.hide {
                    if let Event::DeviceEvent {
                        event: DeviceEvent::MouseMotion { delta: (x, y) },
                        ..
                    } = *event
                    {
                        let theta = player.vert_rotation;

                        let dy = -(y as f32 * 0.1).to_radians();
                        let dy = if theta + dy < FRAC_PI_2 && theta + dy > -FRAC_PI_2 {
                            dy
                        } else {
                            0.
                        };

                        let dx = -(x as f32 * 0.1).to_radians();

                        player.vert_rotation += dy;
                        local.append_rotation_x_axis(dy);
                        local.prepend_rotation_y_axis(dx);
                    }
                }
            }
        }
    }
}
