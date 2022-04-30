extern crate amethyst;
mod game;

use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{bundle::RenderingBundle, types::DefaultBackend, RenderFlat3D, RenderToWindow},
    start_logger,
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    start_logger(Default::default());

    let root = application_root_dir()?;
    let disp = root.join("config/display.ron");
    let input = root.join("config/input.ron");
    let assets = root.join("assets");
    let input_bundle = InputBundle::<StringBindings>::new().with_bindings_from_file(input)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(disp)?.with_clear([0.2, 0.5, 1.0, 1.0]),
                )
                .with_plugin(RenderFlat3D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(game::movement::MovementBundle)?;

    let mut game = Application::new(assets, game::InGame, game_data)?;
    game.run();

    Ok(())
}
