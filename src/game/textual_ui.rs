use amethyst::{
    assets::Loader,
    ecs::{Component, DenseVecStorage, Entity, ReadStorage, WriteStorage, Join},
    prelude::{Builder, WorldExt},
    shred::{System, World, ReadExpect},
    ui::{Anchor, FontHandle, LineMode, TtfFormat, UiText, UiTransform}, core::Transform,
};

use super::Player;

pub struct TextualUi {
    block_text: Entity, // Block name
    coordinates_text: Entity
}

impl Component for TextualUi {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_textual_ui(world: &mut World) {
    // Load font
    let font: FontHandle = world.read_resource::<Loader>().load(
        "Minecraft.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    // Set up text position and size
    let block_text_transform = UiTransform::new(
        "current block".to_string(),
        Anchor::BottomMiddle,
        Anchor::BottomMiddle,
        0.,
        80.,
        0.,
        200.,
        50.,
    );

    let coordinates_transform = UiTransform::new(
        "coordinates".to_string(),
        Anchor::TopLeft,
        Anchor::TopLeft,
        0.,
        0.,
        0.,
        270.,
        50.,
    );

    // Set up UI components for texts
    let block_text = world
        .create_entity()
        .with(block_text_transform)
        .with(UiText::new(
            font.clone(),
            "None".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            24.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    let coordinates_text = world
        .create_entity()
        .with(coordinates_transform)
        .with(UiText::new(
            font,
            "None".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            24.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    world.insert(TextualUi { block_text, coordinates_text });
}

pub struct CurrentBlockUiSystem;

impl<'s> System<'s> for CurrentBlockUiSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, TextualUi>
    );

    fn run(&mut self, (players, locals, mut text_uis, block_uis): Self::SystemData) {
        let (player, transform) = { // Get player entity
            let mut data = (None, None);
            for (local, player) in (&locals, &players).join() {
                data = (Some(player), Some(local));
            }
            data
        };

        // Update block name text based on currently held block
        if let Some(text) = text_uis.get_mut(block_uis.block_text) {
            if let Some(block_name) = &player.unwrap().current_block {
                text.text = block_name.1.to_string();
            }
        }

        // Update coordinates
        if let Some(text) = text_uis.get_mut(block_uis.coordinates_text) {
            if let Some(t) = transform {
                let transl = t.translation();
                text.text = format!("{:.3} {:.3} {:.3}", transl.x, transl.y, transl.z);
            }
        }
    }
}
