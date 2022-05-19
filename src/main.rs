extern crate amethyst;
mod game;

use amethyst::{
    controls::{CursorHideSystemDesc, FreeRotationSystemDesc, MouseFocusUpdateSystemDesc},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings, InputHandler},
    prelude::*,
    renderer::{bundle::RenderingBundle, types::DefaultBackend, RenderFlat3D, RenderToWindow},
    start_logger,
    utils::application_root_dir, ui::{RenderUi, UiBundle},
};

fn main() -> amethyst::Result<()> {
    start_logger(Default::default());

    let root = application_root_dir()?;
    let disp = root.join("config/display.ron");
    let key_bindings_path = root.join("config/input.ron");
    let assets = root.join("assets");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings_path)?;

    let sensitivity_x = 1.0;
    let sensitivity_y = 1.0;
    let game_data = GameDataBuilder::default()
        .with_system_desc(
            FreeRotationSystemDesc::new(sensitivity_x, sensitivity_y),
            "free_rotation",
            &[],
        )
        .with_system_desc(
            MouseFocusUpdateSystemDesc::default(),
            "mouse_focus",
            &["free_rotation"],
        )
        .with_system_desc(
            CursorHideSystemDesc::default(),
            "cursor_hide",
            &["mouse_focus"],
        )
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(disp)?.with_clear([0.2, 0.5, 1.0, 1.0]),
                )
                .with_plugin(RenderFlat3D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(game::movement::MovementBundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(game::CurrentBlockUiSystem, "BlockUiSystem", &[])
        .with_system_desc(game::MouseRaycastSystemDesc, "mouse_raycast", &[]);

    let mut game = Application::new(assets, game::InGame, game_data)?;
    game.run();

    Ok(())
}
