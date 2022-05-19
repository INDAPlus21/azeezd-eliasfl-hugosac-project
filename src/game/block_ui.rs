use amethyst::{
    assets::Loader,
    ecs::{Component, DenseVecStorage, Entity, ReadStorage, WriteStorage, Join},
    prelude::{Builder, WorldExt},
    shred::{System, World, ReadExpect},
    ui::{Anchor, FontHandle, LineMode, TtfFormat, UiText, UiTransform},
};

use super::Player;

pub struct CurrentBlockUi {
    text: Entity,
}

impl Component for CurrentBlockUi {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_block_ui(world: &mut World) {
    let font: FontHandle = world.read_resource::<Loader>().load(
        "Minecraft.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let text_transform = UiTransform::new(
        "current block".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.,
        -200.,
        0.,
        200.,
        50.,
    );

    let text = world
        .create_entity()
        .with(text_transform)
        .with(UiText::new(
            font,
            "None".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            32.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    world.insert(CurrentBlockUi { text });
}

pub struct CurrentBlockUiSystem;

impl<'s> System<'s> for CurrentBlockUiSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, CurrentBlockUi>
    );

    fn run(&mut self, (players, mut text_uis, block_uis): Self::SystemData) {
        let current_block = {
            let mut b = None;
            for player in (&players).join() {
                b = player.current_block.as_ref();
            }
            b
        };

        if let Some(text) = text_uis.get_mut(block_uis.text) {
            text.text = "a".to_string();
        }
    }
}
